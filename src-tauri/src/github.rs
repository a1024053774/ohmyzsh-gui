use crate::config;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repo {
    pub owner: String,
    pub name: String,
}
impl Repo {
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim().trim_end_matches('/').trim_end_matches(".git");
        let s = s
            .strip_prefix("https://github.com/")
            .or_else(|| s.strip_prefix("git@github.com:"))
            .unwrap_or(s);
        let parts: Vec<_> = s.split('/').collect();
        if parts.len() != 2 || parts.iter().any(|p| !config::component(p)) {
            return Err("Use a GitHub owner/repository or HTTPS repository URL".into());
        }
        Ok(Self {
            owner: parts[0].into(),
            name: parts[1].into(),
        })
    }
    pub fn slug(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
    pub fn url(&self) -> String {
        format!("https://github.com/{}.git", self.slug())
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub repo: Repo,
    pub summary: String,
    pub stars: u64,
    pub license: String,
    pub updated: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Search {
    pub items: Vec<Candidate>,
    pub remaining: Option<String>,
    pub reset: Option<String>,
    pub checked: u64,
    pub cached: bool,
}
pub fn seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new("dev.ohmyzsh.gui", "github-public-read")
        .map_err(|_| "OS credential store unavailable".into())
}
pub fn token() -> Result<Option<String>, String> {
    match entry()?.get_password() {
        Ok(s) => Ok(Some(s)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(_) => Err(
            "OS credential store locked or unavailable. Unlock it or use anonymous requests."
                .into(),
        ),
    }
}
pub fn set_token(s: &str) -> Result<(), String> {
    let e = entry()?;
    if s.is_empty() {
        match e.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err("Could not remove credential".into()),
        }
    } else if s.contains(char::is_whitespace) {
        Err("Token must not contain spaces".into())
    } else {
        e.set_password(s)
            .map_err(|_| "Could not save token to OS credential store".into())
    }
}
pub struct GitHub {
    client: Client,
    pub anonymous: bool,
}
impl GitHub {
    pub fn new(anonymous: bool) -> Result<Self, String> {
        Ok(Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(25))
                .redirect(reqwest::redirect::Policy::none())
                .user_agent("ohmyzsh-gui/0.1")
                .build()
                .map_err(|e| e.to_string())?,
            anonymous,
        })
    }
    fn get(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<(Value, Option<String>, Option<String>), String> {
        let mut r = self
            .client
            .get(format!("https://api.github.com{path}"))
            .header("Accept", "application/vnd.github+json")
            .query(query);
        if !self.anonymous {
            if let Some(t) = token()? {
                r = r.bearer_auth(t)
            }
        }
        let r = r
            .send()
            .map_err(|_| "GitHub connection failed. Check your network and retry.".to_string())?;
        let h = |name: &str| {
            r.headers()
                .get(name)
                .and_then(|x| x.to_str().ok())
                .map(String::from)
        };
        let rem = h("x-ratelimit-remaining");
        let reset = h("x-ratelimit-reset");
        if r.status().as_u16() == 401 {
            return Err(
                "GitHub token rejected (401). Replace it in Settings or use anonymous access."
                    .into(),
            );
        }
        if [403, 429].contains(&r.status().as_u16()) {
            return Err(format!("GitHub limit or permission error ({}). Remaining: {}. Reset (Unix seconds): {}. Optional token in Settings.",r.status().as_u16(),rem.as_deref().unwrap_or("unknown"),reset.as_deref().unwrap_or("unknown")));
        }
        if !r.status().is_success() {
            return Err(format!("GitHub returned {}", r.status().as_u16()));
        }
        Ok((r.json().map_err(|_| "Invalid GitHub response")?, rem, reset))
    }
    pub fn search(&self, q: &str) -> Result<Search, String> {
        let q = format!("{} topic:zsh-plugin", q.trim());
        let (v, remaining, reset) = self.get(
            "/search/repositories",
            &[("q", &q), ("sort", "stars"), ("per_page", "30")],
        )?;
        let items = v["items"]
            .as_array()
            .ok_or("Missing GitHub items")?
            .iter()
            .filter_map(|v| Self::candidate(v).ok())
            .collect();
        Ok(Search {
            items,
            remaining,
            reset,
            checked: seconds(),
            cached: false,
        })
    }
    pub fn candidate(v: &Value) -> Result<Candidate, String> {
        Ok(Candidate {
            repo: Repo::parse(v["full_name"].as_str().ok_or("Missing repository name")?)?,
            summary: v["description"].as_str().unwrap_or("No description").into(),
            stars: v["stargazers_count"].as_u64().unwrap_or(0),
            license: v["license"]["spdx_id"].as_str().unwrap_or("Unknown").into(),
            updated: v["updated_at"].as_str().unwrap_or("").into(),
        })
    }
    pub fn info(&self, repo: &Repo) -> Result<Candidate, String> {
        Self::candidate(&self.get(&format!("/repos/{}", repo.slug()), &[])?.0)
    }
    pub fn head(&self, repo: &Repo) -> Result<String, String> {
        let v = self
            .get(&format!("/repos/{}/commits/HEAD", repo.slug()), &[])?
            .0;
        let s = v["sha"].as_str().ok_or("Missing SHA")?;
        if s.len() != 40 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("Invalid upstream SHA".into());
        }
        Ok(s.into())
    }
    pub fn compare(&self, r: &Repo, a: &str, b: &str) -> Result<String, String> {
        let v = self
            .get(&format!("/repos/{}/compare/{a}...{b}", r.slug()), &[])?
            .0;
        Ok(v["commits"]
            .as_array()
            .ok_or("Missing commits")?
            .iter()
            .take(30)
            .map(|c| {
                format!(
                    "{} {}",
                    c["sha"]
                        .as_str()
                        .unwrap_or("")
                        .chars()
                        .take(8)
                        .collect::<String>(),
                    c["commit"]["message"]
                        .as_str()
                        .unwrap_or("")
                        .lines()
                        .next()
                        .unwrap_or("")
                )
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
    pub fn official(&self) -> Result<Vec<String>, String> {
        let v = self.get("/repos/ohmyzsh/ohmyzsh/contents/plugins", &[])?.0;
        Ok(v.as_array()
            .ok_or("Invalid official index")?
            .iter()
            .filter(|x| x["type"] == "dir")
            .filter_map(|x| x["name"].as_str())
            .filter(|n| config::component(n))
            .map(String::from)
            .collect())
    }
}
