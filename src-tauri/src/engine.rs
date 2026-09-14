use crate::{
    config::{self, Document, Values},
    github::{self, GitHub, Repo},
    system::{self, Environment},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub path: String,
    pub source: String,
    pub values: Values,
    pub warnings: Vec<String>,
    pub zsh_available: bool,
    pub platform: String,
    pub plugin_root: String,
    pub installed_custom: Vec<String>,
    pub installed_custom_info: Vec<InstalledPlugin>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InstalledPlugin {
    pub name: String,
    pub repo: Option<Repo>,
    pub current_sha: Option<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Preview {
    pub id: String,
    pub title: String,
    pub detail: String,
    pub diff: String,
    pub expected_source: String,
    pub proposed_source: String,
    pub target_sha: Option<String>,
    pub files: Vec<String>,
    pub can_apply: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Applied {
    pub id: String,
    pub status: String,
    pub backup: Option<String>,
    pub commit: Option<String>,
    pub undo_available: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct History {
    pub id: String,
    pub at: u64,
    pub operation: String,
    pub status: String,
    pub files: Vec<String>,
    pub backup: Option<String>,
    pub undo_available: bool,
    #[serde(default)]
    pub repo: Option<Repo>,
    #[serde(default)]
    pub commit: Option<String>,
    pub error: Option<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Capability {
    pub zsh: bool,
    pub git: bool,
    pub path: String,
    pub platform: String,
    pub warnings: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PluginView {
    pub name: String,
    pub kind: String,
    pub repo: Option<Repo>,
    pub enabled: bool,
    pub installed: bool,
    pub current_sha: Option<String>,
    pub available_sha: Option<String>,
    pub summary: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateView {
    pub name: String,
    pub repo: Repo,
    pub current_sha: String,
    pub available_sha: String,
    pub summary: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub plugins: Vec<PluginView>,
    pub capabilities: Capability,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Request {
    Read,
    Capability,
    PreviewConfig {
        theme: String,
        plugins: Vec<String>,
        expected_source: Option<String>,
    },
    Apply {
        plan_id: String,
    },
    Cancel {
        plan_id: String,
    },
    Search {
        query: String,
    },
    SetToken {
        token: String,
    },
    TokenStatus,
    PreviewPlugin {
        operation: String,
        owner: String,
        repository: String,
        requested_sha: Option<String>,
    },
    Undo {
        history_id: String,
    },
    History,
    Updates,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    pub snapshot: Option<Snapshot>,
    pub preview: Option<Preview>,
    pub applied: Option<Applied>,
    pub inventory: Option<Inventory>,
    pub search: Option<github::Search>,
    pub capability: Option<Capability>,
    pub history: Option<Vec<History>>,
    pub updates: Option<Vec<UpdateView>>,
    pub message: Option<String>,
}
struct Plan {
    operation: String,
    expected_source: Option<String>,
    proposed_source: Option<String>,
    repo: Option<Repo>,
    target: Option<String>,
}
pub struct Manager {
    pub env: Environment,
    plans: HashMap<String, Plan>,
    journal: Vec<History>,
    journal_path: PathBuf,
}
impl Manager {
    pub fn new() -> Result<Self, String> {
        let env = Environment::discover()?;
        let journal_path = env.data.join("history.json");
        let journal = fs::read(&journal_path)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default();
        Ok(Self {
            journal_path,
            env,
            plans: HashMap::new(),
            journal,
        })
    }
    fn save_history(&self) -> Result<(), String> {
        system::write_json(&self.journal_path, &self.journal)
    }
    fn snap(&mut self) -> Result<Snapshot, String> {
        self.env.resolve_layout()?;
        let source = self.env.source()?;
        let d = Document::parse(source.clone());
        let installed_custom = fs::read_dir(self.env.custom.join("plugins"))
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .filter(|n| config::component(n))
            .collect::<Vec<_>>();
        let installed_custom_info = installed_custom
            .iter()
            .map(|name| {
                let path = self.env.plugin_path(name).ok();
                let current_sha = path.as_deref().and_then(|p| self.env.repo_clean(p).ok());
                let repo = path.as_deref().and_then(|p| {
                    self.journal.iter().rev().find_map(|h| {
                        h.files
                            .iter()
                            .any(|f| f == &p.display().to_string())
                            .then(|| h.repo.clone())
                            .flatten()
                    })
                });
                InstalledPlugin {
                    name: name.clone(),
                    repo,
                    current_sha,
                }
            })
            .collect();
        Ok(Snapshot {
            path: self.env.config.display().to_string(),
            source,
            values: d.values,
            warnings: d.warnings,
            zsh_available: self.env.tool_available("zsh"),
            platform: self.env.label.clone(),
            plugin_root: self.env.custom.join("plugins").display().to_string(),
            installed_custom,
            installed_custom_info,
        })
    }
    fn cap(&mut self) -> Result<Capability, String> {
        self.env.resolve_layout()?;
        Ok(Capability {
            zsh: self.env.tool_available("zsh"),
            git: self.env.tool_available("git"),
            path: self.env.config.display().to_string(),
            platform: self.env.label.clone(),
            warnings: vec![],
        })
    }
    fn updates(&mut self) -> Result<Vec<UpdateView>, String> {
        let snapshot = self.snap()?;
        let client = self.github()?;
        let mut updates = Vec::new();
        for plugin in snapshot.installed_custom_info {
            let (Some(repo), Some(current_sha)) = (plugin.repo, plugin.current_sha) else {
                continue;
            };
            let available_sha = client.head(&repo)?;
            if available_sha == current_sha {
                continue;
            }
            let summary = client
                .compare(&repo, &current_sha, &available_sha)
                .unwrap_or_else(|_| "Commit summary unavailable".into());
            updates.push(UpdateView {
                name: plugin.name,
                repo,
                current_sha,
                available_sha,
                summary,
            });
        }
        Ok(updates)
    }
    fn github(&self) -> Result<GitHub, String> {
        GitHub::new(github::token().ok().flatten().is_none())
    }
    fn plan(
        &mut self,
        p: Plan,
        title: String,
        detail: String,
        diff: String,
        target: Option<String>,
        files: Vec<String>,
        can: bool,
    ) -> Response {
        let id = system::id();
        let expected = p.expected_source.clone().unwrap_or_default();
        let proposed = p.proposed_source.clone().unwrap_or_default();
        self.plans.insert(id.clone(), p);
        Response {
            snapshot: None,
            preview: Some(Preview {
                id,
                title,
                detail,
                diff,
                expected_source: expected,
                proposed_source: proposed,
                target_sha: target,
                files,
                can_apply: can,
            }),
            applied: None,
            inventory: None,
            search: None,
            capability: None,
            history: None,
            updates: None,
            message: None,
        }
    }
    fn config_preview(
        &mut self,
        theme: String,
        plugins: Vec<String>,
        expected: Option<String>,
    ) -> Result<Response, String> {
        let source = expected.unwrap_or(self.env.source()?);
        let d = Document::parse(source.clone());
        let proposed = d.edit(&Values { theme, plugins })?;
        let p = Plan {
            operation: "configuration".into(),
            expected_source: Some(source.clone()),
            proposed_source: Some(proposed.clone()),
            repo: None,
            target: None,
        };
        Ok(self.plan(
            p,
            "Review .zshrc changes".into(),
            "The proposed file will be syntax checked, backed up, and replaced atomically.".into(),
            config::diff(&source, &proposed),
            None,
            vec![self.env.config.display().to_string()],
            true,
        ))
    }
    fn plugin_preview(
        &mut self,
        operation: String,
        owner: String,
        repository: String,
        requested: Option<String>,
    ) -> Result<Response, String> {
        let repo = Repo::parse(&format!("{owner}/{repository}"))?;
        let source = self.env.source().ok();
        let mut files = vec![repo.slug()];
        let target = if let Some(s) = requested {
            if s.len() != 40 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err("Commit must be a 40-character SHA".into());
            }
            s
        } else if operation == "remove" {
            let path = self.env.plugin_path(&repo.name)?;
            self.env.repo_clean(&path)?
        } else {
            self.github()?.head(&repo)?
        };
        let detail = match operation.as_str() {
            "install" => {
                if self.env.plugin_path(&repo.name)?.exists() {
                    return Err("Plugin already installed".into());
                }
                format!(
                    "Install {} at commit {}. Git hooks and submodules are disabled.",
                    repo.slug(),
                    &target[..8]
                )
            }
            "update" => {
                let path = self.env.plugin_path(&repo.name)?;
                let current = self.env.repo_clean(&path)?;
                let summary = self
                    .github()?
                    .compare(&repo, &current, &target)
                    .unwrap_or_else(|_| "Commit summary unavailable".into());
                files.push(path.display().to_string());
                format!(
                    "Update {} from {} to {}.\n{}",
                    repo.slug(),
                    &current[..8],
                    &target[..8],
                    summary
                )
            }
            "remove" => {
                let path = self.env.plugin_path(&repo.name)?;
                let current = self.env.repo_clean(&path)?;
                files.push(path.display().to_string());
                format!("Remove {} at {} after disabling it in .zshrc. Checkout is quarantined and recorded for manual restore.",repo.slug(),&current[..8])
            }
            _ => return Err("Unknown plugin operation".into()),
        };
        let (proposed, diff) = if operation == "remove" {
            let d = Document::parse(source.clone().ok_or("Cannot read .zshrc")?);
            let mut v = d.values.clone();
            v.plugins.retain(|n| n != &repo.name);
            let p = d.edit(&v)?;
            (Some(p.clone()), config::diff(&d.source, &p))
        } else {
            (None, "No .zshrc change".into())
        };
        let p = Plan {
            operation,
            expected_source: source,
            proposed_source: proposed,
            repo: Some(repo.clone()),
            target: Some(target.clone()),
        };
        Ok(self.plan(
            p,
            "Review plugin operation".into(),
            detail,
            diff,
            Some(target),
            files,
            true,
        ))
    }
    fn apply(&mut self, id: &str) -> Result<Response, String> {
        let p = self
            .plans
            .remove(id)
            .ok_or("Preview expired or was cancelled; preview again")?;
        let result = match p.operation.as_str() {
            "configuration" => self.apply_config(id, &p),
            "install" => self.apply_install(id, &p),
            "update" => self.apply_update(id, &p),
            "remove" => self.apply_remove(id, &p),
            _ => Err("Unsupported operation".into()),
        };
        match result {
            Ok(a) => Ok(Response {
                snapshot: None,
                preview: None,
                applied: Some(a),
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: None,
                message: None,
            }),
            Err(e) => {
                self.plans.insert(id.to_string(), p);
                Err(e)
            }
        }
    }
    fn apply_config(&mut self, id: &str, p: &Plan) -> Result<Applied, String> {
        let before = p.expected_source.as_ref().ok_or("Missing snapshot")?;
        let after = p.proposed_source.as_ref().ok_or("Missing proposal")?;
        let b = self.env.write_config(before, after)?;
        self.record(
            id,
            "configuration",
            "applied",
            vec![self.env.config.display().to_string()],
            Some(b.clone()),
            None,
            None,
            false,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status: "applied".into(),
            backup: Some(b.display().to_string()),
            commit: None,
            undo_available: false,
        })
    }
    fn apply_install(&mut self, id: &str, p: &Plan) -> Result<Applied, String> {
        let repo = p.repo.as_ref().unwrap();
        let target = p.target.as_ref().unwrap();
        let path = self.env.plugin_path(&repo.name)?;
        let parent = path.parent().ok_or("Missing plugin root")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let tmp = parent.join(format!(".{}-{}", repo.name, system::id()));
        self.env.git(
            None,
            &[
                "clone",
                "--filter=blob:none",
                "--no-recurse-submodules",
                &repo.url(),
                &tmp.display().to_string(),
            ],
        )?;
        self.env
            .git(Some(&tmp), &["checkout", "--detach", target])?;
        let got = self.env.git(Some(&tmp), &["rev-parse", "HEAD"])?;
        if got != *target {
            let _ = fs::remove_dir_all(&tmp);
            return Err("Checkout did not reach requested commit".into());
        }
        fs::rename(&tmp, &path).map_err(|e| format!("Plugin activation failed: {e}"))?;
        self.record(
            id,
            "install",
            "applied",
            vec![path.display().to_string()],
            None,
            Some(repo.clone()),
            Some(got.clone()),
            false,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status: "installed".into(),
            backup: None,
            commit: Some(got),
            undo_available: false,
        })
    }
    fn apply_update(&mut self, id: &str, p: &Plan) -> Result<Applied, String> {
        let repo = p.repo.as_ref().unwrap();
        let target = p.target.as_ref().unwrap();
        let path = self.env.plugin_path(&repo.name)?;
        let old = self.env.repo_clean(&path)?;
        self.env
            .git(Some(&path), &["fetch", "--no-tags", "origin", target])?;
        self.env
            .git(Some(&path), &["checkout", "--detach", target])?;
        let got = self.env.git(Some(&path), &["rev-parse", "HEAD"])?;
        if got != *target {
            return Err("Update did not reach requested commit".into());
        }
        self.record(
            id,
            "update",
            "applied",
            vec![path.display().to_string()],
            None,
            Some(repo.clone()),
            Some(got.clone()),
            false,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status: format!("updated from {}", &old[..8]),
            backup: None,
            commit: Some(got),
            undo_available: false,
        })
    }
    fn apply_remove(&mut self, id: &str, p: &Plan) -> Result<Applied, String> {
        let repo = p.repo.as_ref().unwrap();
        let path = self.env.plugin_path(&repo.name)?;
        let before = p.expected_source.as_ref().ok_or("Missing snapshot")?;
        let after = p.proposed_source.as_ref().ok_or("Missing proposal")?;
        let backup = self.env.write_config(before, after)?;
        let q = self.env.env_quarantine(&repo.name, id)?;
        if let Err(e) = fs::rename(&path, &q) {
            let _ = fs::copy(&backup, &self.env.config);
            return Err(format!(
                "Plugin quarantine failed; configuration restored: {e}"
            ));
        }
        self.record(
            id,
            "remove",
            "applied",
            vec![path.display().to_string(), q.display().to_string()],
            Some(backup.clone()),
            Some(repo.clone()),
            Some(p.target.clone().unwrap_or_default()),
            false,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status: "removed (quarantined; manual restore is required)".into(),
            backup: Some(backup.display().to_string()),
            commit: None,
            undo_available: false,
        })
    }
    fn record(
        &mut self,
        id: &str,
        op: &str,
        status: &str,
        files: Vec<String>,
        backup: Option<PathBuf>,
        repo: Option<Repo>,
        commit: Option<String>,
        undo: bool,
        error: Option<String>,
    ) -> Result<(), String> {
        self.journal.push(History {
            id: id.into(),
            at: system::seconds(),
            operation: op.into(),
            status: status.into(),
            files,
            backup: backup.map(|x| x.display().to_string()),
            undo_available: undo,
            repo,
            commit,
            error,
        });
        self.save_history()
    }
    pub fn request(&mut self, r: Request) -> Result<Response, String> {
        match r {
            Request::Read => Ok(Response {
                snapshot: Some(self.snap()?),
                preview: None,
                applied: None,
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: None,
                message: None,
            }),
            Request::Capability => Ok(Response {
                snapshot: None,
                preview: None,
                applied: None,
                inventory: None,
                search: None,
                capability: Some(self.cap()?),
                history: None,
                updates: None,
                message: None,
            }),
            Request::PreviewConfig {
                theme,
                plugins,
                expected_source,
            } => self.config_preview(theme, plugins, expected_source),
            Request::Apply { plan_id } => self.apply(&plan_id),
            Request::Cancel { plan_id } => {
                self.plans.remove(&plan_id);
                Ok(Response {
                    snapshot: None,
                    preview: None,
                    applied: None,
                    inventory: None,
                    search: None,
                    capability: None,
                    history: None,
                    updates: None,
                    message: Some("Cancelled".into()),
                })
            }
            Request::Search { query } => Ok(Response {
                snapshot: None,
                preview: None,
                applied: None,
                inventory: None,
                search: Some(self.github()?.search(&query)?),
                capability: None,
                history: None,
                updates: None,
                message: None,
            }),
            Request::SetToken { token } => {
                github::set_token(&token)?;
                Ok(Response {
                    snapshot: None,
                    preview: None,
                    applied: None,
                    inventory: None,
                    search: None,
                    capability: None,
                    history: None,
                    updates: None,
                    message: Some("Token saved in OS credential store".into()),
                })
            }
            Request::TokenStatus => Ok(Response {
                snapshot: None,
                preview: None,
                applied: None,
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: None,
                message: Some(match github::token() {
                    Ok(Some(_)) => "configured".into(),
                    Ok(None) => "not_configured".into(),
                    Err(e) => format!("unavailable: {e}"),
                }),
            }),
            Request::PreviewPlugin {
                operation,
                owner,
                repository,
                requested_sha,
            } => self.plugin_preview(operation, owner, repository, requested_sha),
            Request::History => Ok(Response {
                snapshot: None,
                preview: None,
                applied: None,
                inventory: None,
                search: None,
                capability: None,
                history: Some(self.journal.clone()),
                updates: None,
                message: None,
            }),
            Request::Updates => Ok(Response {
                snapshot: None,
                preview: None,
                applied: None,
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: Some(self.updates()?),
                message: None,
            }),
            Request::Undo { history_id: _ } => Err(
                "Undo is recorded but this restore action is not yet enabled in this MVP".into(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_preserves_repository_and_commit_metadata() {
        let history = History {
            id: "op-1".into(),
            at: 1,
            operation: "install".into(),
            status: "applied".into(),
            files: vec!["/tmp/plugin".into()],
            backup: None,
            undo_available: true,
            repo: Some(Repo {
                owner: "example".into(),
                name: "plugin".into(),
            }),
            commit: Some("0123456789012345678901234567890123456789".into()),
            error: None,
        };
        let encoded = serde_json::to_string(&history).unwrap();
        let decoded: History = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.repo, history.repo);
        assert_eq!(decoded.commit, history.commit);

        let legacy = r#"{"id":"old","at":1,"operation":"configuration","status":"applied","files":[],"backup":null,"undo_available":true,"error":null}"#;
        let decoded: History = serde_json::from_str(legacy).unwrap();
        assert!(decoded.repo.is_none());
        assert!(decoded.commit.is_none());
    }
}
