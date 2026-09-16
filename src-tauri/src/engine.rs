use crate::{
    config::{self, Document, Values},
    github::{self, GitHub, Repo},
    system::{self, Environment},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

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
    #[serde(default)]
    pub official_plugins: Vec<CatalogItem>,
    #[serde(default)]
    pub themes: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CatalogItem {
    pub name: String,
    pub summary: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InstalledPlugin {
    pub name: String,
    pub repo: Option<Repo>,
    pub current_sha: Option<String>,
    #[serde(default)]
    pub loadable: bool,
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
    #[serde(default)]
    pub previous_commit: Option<String>,
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
    Readme {
        owner: String,
        repository: String,
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
    RestoreBackup {
        path: String,
    },
    DeleteBackup {
        path: String,
    },
    PluginReadme {
        name: String,
    },
    OpenUrl {
        url: String,
    },
    OpenTerminal,
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
        let mut installed_custom = list_custom_dirs(&self.env.custom.join("plugins"));
        for name in list_custom_dirs(&self.env.custom.join("themes")) {
            if !installed_custom.iter().any(|n| n == &name) {
                installed_custom.push(name);
            }
        }
        let installed_custom_info = installed_custom
            .iter()
            .map(|name| {
                let path = self.checkout_dir(name).ok();
                let current_sha = path.as_deref().and_then(|p| self.env.repo_clean(p).ok());
                let repo = path.as_deref().and_then(|p| {
                    self.journal
                        .iter()
                        .rev()
                        .find_map(|h| {
                            h.files
                                .iter()
                                .any(|f| f == &p.display().to_string())
                                .then(|| h.repo.clone())
                                .flatten()
                        })
                        .or_else(|| origin_repo(&self.env, p))
                });
                InstalledPlugin {
                    name: name.clone(),
                    repo,
                    current_sha,
                    loadable: path
                        .as_ref()
                        .map(|p| omz_plugin_file(p, name))
                        .unwrap_or(false),
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
            official_plugins: catalog_dirs(&self.env.zsh.join("plugins")),
            themes: list_themes(&self.env.custom.join("themes")),
        })
    }
    fn plugin_readme(&self, name: &str) -> Result<String, String> {
        if !config::component(name) {
            return Err("Invalid plugin name".into());
        }
        let candidates = [
            self.env.zsh.join("plugins").join(name).join("README.md"),
            self.env.custom.join("plugins").join(name).join("README.md"),
        ];
        for path in candidates {
            if !path.is_file() {
                continue;
            }
            let root = path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .ok_or("Invalid README path")?;
            if !contained(&path, root) {
                continue;
            }
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&bytes[..bytes.len().min(48_000)]).into_owned();
            if !text.trim().is_empty() {
                return Ok(text);
            }
        }
        Err("README not found for this plugin".into())
    }
    fn open_url(&self, url: &str) -> Result<(), String> {
        let url = safe_github_url(url)?;
        let status = {
            #[cfg(target_os = "macos")]
            {
                Command::new("open").arg(&url).status()
            }
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd").args(["/C", "start", "", &url]).status()
            }
            #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
            {
                Command::new("xdg-open").arg(&url).status()
            }
        }
        .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Could not open the GitHub link".into())
        }
    }
    fn open_terminal(&self) -> Result<(), String> {
        let status = {
            #[cfg(target_os = "macos")]
            {
                Command::new("osascript")
                    .args(["-e", "tell application \"Terminal\" to do script \"\""])
                    .status()
            }
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd").args(["/C", "start", "cmd"]).status()
            }
            #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
            {
                Command::new("x-terminal-emulator").status()
            }
        }
        .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Could not open a new terminal".into())
        }
    }
    fn can_enable_plugin(&self, name: &str) -> bool {
        if !config::component(name) {
            return false;
        }
        let official = self
            .env
            .zsh
            .join("plugins")
            .join(name)
            .join(format!("{name}.plugin.zsh"));
        official.is_file() || omz_plugin_file(&self.env.custom.join("plugins").join(name), name)
    }
    fn checkout_dir(&self, name: &str) -> Result<PathBuf, String> {
        let plugin = self.env.plugin_path(name)?;
        if plugin.exists() {
            return Ok(plugin);
        }
        let theme = self.env.theme_path(name)?;
        if theme.exists() {
            return Ok(theme);
        }
        Err(format!("{name} is not installed"))
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
        for name in &plugins {
            if d.values.plugins.iter().any(|p| p == name) {
                continue;
            }
            if !self.can_enable_plugin(name) {
                return Err(format!(
                    "{name} is not a loadable Oh My Zsh plugin. Install a regular plugin, or keep it out of plugins=()."
                ));
            }
        }
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
                if self.env.plugin_path(&repo.name)?.exists()
                    || self.env.theme_path(&repo.name)?.exists()
                {
                    return Err("Plugin already installed".into());
                }
                format!(
                    "Install {} at commit {}. Regular Oh My Zsh plugins are enabled in .zshrc. Themes go into custom/themes. Other checkouts are not added to plugins=(). Git hooks and submodules are disabled.",
                    repo.slug(),
                    &target[..8]
                )
            }
            "update" => {
                let path = self.checkout_dir(&repo.name)?;
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
                let path = self.checkout_dir(&repo.name)?;
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
            None,
            true,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status: "applied".into(),
            backup: Some(b.display().to_string()),
            commit: None,
            undo_available: true,
        })
    }
    fn apply_install(&mut self, id: &str, p: &Plan) -> Result<Applied, String> {
        let repo = p.repo.as_ref().unwrap();
        let target = p.target.as_ref().unwrap();
        let staging_root = self.env.custom.join("plugins");
        fs::create_dir_all(&staging_root).map_err(|e| e.to_string())?;
        let tmp = staging_root.join(format!(".{}-{}", repo.name, system::id()));
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
        let loadable = omz_plugin_file(&tmp, &repo.name);
        let theme = theme_literal_in(&tmp, &repo.name);
        let path = if !loadable && theme.is_some() {
            self.env.theme_path(&repo.name)?
        } else {
            self.env.plugin_path(&repo.name)?
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::rename(&tmp, &path).map_err(|e| format!("Plugin activation failed: {e}"))?;
        let mut backup = None;
        let mut status = "installed".to_string();
        if loadable || theme.is_some() {
            let source = self.env.source()?;
            let d = Document::parse(source.clone());
            let mut values = d.values.clone();
            if loadable && !values.plugins.iter().any(|n| n == &repo.name) {
                values.plugins.push(repo.name.clone());
            }
            if let Some(theme) = theme {
                values.theme = theme;
                status = "installed theme".into();
            } else if loadable {
                status = "installed and enabled".into();
            }
            if values != d.values {
                let proposed = d.edit(&values)?;
                backup = Some(self.env.write_config(&source, &proposed)?);
            }
        } else {
            status = "installed checkout; not enabled because it is not an Oh My Zsh plugin".into();
        }
        self.record(
            id,
            "install",
            "applied",
            vec![path.display().to_string()],
            backup.clone(),
            Some(repo.clone()),
            Some(got.clone()),
            None,
            true,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status,
            backup: backup.map(|b| b.display().to_string()),
            commit: Some(got),
            undo_available: true,
        })
    }
    fn apply_update(&mut self, id: &str, p: &Plan) -> Result<Applied, String> {
        let repo = p.repo.as_ref().unwrap();
        let target = p.target.as_ref().unwrap();
        let path = self.checkout_dir(&repo.name)?;
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
            Some(old.clone()),
            true,
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
        let path = self.checkout_dir(&repo.name)?;
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
            None,
            true,
            None,
        )?;
        Ok(Applied {
            id: id.into(),
            status: "removed (quarantined; manual restore is required)".into(),
            backup: Some(backup.display().to_string()),
            commit: None,
            undo_available: true,
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
        previous_commit: Option<String>,
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
            previous_commit,
            error,
        });
        self.save_history()
    }
    fn undo(&mut self, history_id: &str) -> Result<Applied, String> {
        let index = self
            .journal
            .iter()
            .position(|h| h.id == history_id)
            .ok_or("History entry not found")?;
        let history = self.journal[index].clone();
        if !history.undo_available {
            return Err("This operation cannot be restored automatically".into());
        }
        let operation_id = system::id();
        let mut files = history.files.clone();
        let mut backup = None;
        let mut restored_commit = history.commit.clone();
        let mut restored_previous_commit = history.previous_commit.clone();
        match history.operation.as_str() {
            "configuration" => {
                let backup_path = history
                    .backup
                    .as_deref()
                    .ok_or("Configuration backup missing")?;
                let restored = fs::read_to_string(backup_path).map_err(|e| e.to_string())?;
                let current = self.env.source()?;
                backup = Some(self.env.write_config(&current, &restored)?);
            }
            "install" => {
                let repo = history
                    .repo
                    .as_ref()
                    .ok_or("Plugin repository metadata missing")?;
                let path = self.env.plugin_path(&repo.name)?;
                if !path.is_dir() {
                    return Err("Installed checkout is missing".into());
                }
                let document = Document::parse(self.env.source()?);
                if document
                    .values
                    .plugins
                    .iter()
                    .any(|name| name == &repo.name)
                {
                    return Err(
                        "Disable this plugin in .zshrc before undoing its installation".into(),
                    );
                }
                self.env.repo_clean(&path)?;
                let quarantine = self.env.env_quarantine(&repo.name, &operation_id)?;
                fs::rename(&path, &quarantine).map_err(|e| e.to_string())?;
                files.push(quarantine.display().to_string());
            }
            "remove" => {
                let original =
                    PathBuf::from(history.files.first().ok_or("Removed plugin path missing")?);
                let quarantine = PathBuf::from(
                    history
                        .files
                        .get(1)
                        .ok_or("Plugin quarantine path missing")?,
                );
                if fs::symlink_metadata(&original).is_ok() {
                    return Err("Plugin destination is no longer empty".into());
                }
                fs::rename(&quarantine, &original).map_err(|e| e.to_string())?;
                if let Some(backup_path) = history.backup.as_deref() {
                    let restored = fs::read_to_string(backup_path).map_err(|e| e.to_string())?;
                    let current = self.env.source()?;
                    match self.env.write_config(&current, &restored) {
                        Ok(b) => backup = Some(b),
                        Err(e) => {
                            let _ = fs::rename(&original, &quarantine);
                            return Err(e);
                        }
                    }
                }
            }
            "update" => {
                let repo = history
                    .repo
                    .as_ref()
                    .ok_or("Plugin repository metadata missing")?;
                let previous = history
                    .previous_commit
                    .as_deref()
                    .ok_or("Previous commit metadata missing")?;
                let target = history
                    .commit
                    .as_deref()
                    .ok_or("Current commit metadata missing")?;
                let path = self.env.plugin_path(&repo.name)?;
                let current = self.env.repo_clean(&path)?;
                if current != target {
                    return Err(
                        "Checkout changed since the update; refusing automatic restore".into(),
                    );
                }
                self.env
                    .git(Some(&path), &["fetch", "--no-tags", "origin", previous])?;
                self.env
                    .git(Some(&path), &["checkout", "--detach", previous])?;
                let got = self.env.git(Some(&path), &["rev-parse", "HEAD"])?;
                if got != previous {
                    return Err("Restore did not reach the previous commit".into());
                }
                restored_commit = Some(got);
                restored_previous_commit = Some(target.into());
            }
            _ => return Err("This operation does not support automatic restore".into()),
        }
        self.journal[index].undo_available = false;
        self.record(
            &operation_id,
            "undo",
            "restored",
            files,
            backup.clone(),
            history.repo.clone(),
            restored_commit.clone(),
            restored_previous_commit,
            false,
            None,
        )?;
        Ok(Applied {
            id: operation_id,
            status: "restored".into(),
            backup: backup.map(|p| p.display().to_string()),
            commit: restored_commit,
            undo_available: false,
        })
    }
    fn checked_backup(&self, path: &str) -> Result<PathBuf, String> {
        let root = fs::canonicalize(self.env.data.join("backups"))
            .map_err(|_| "Backup directory is unavailable".to_string())?;
        let candidate =
            fs::canonicalize(path).map_err(|_| "Backup file is unavailable".to_string())?;
        if !candidate.starts_with(&root)
            || candidate.extension().and_then(|x| x.to_str()) != Some("zshrc")
        {
            return Err("Backup path is outside the managed backup directory".into());
        }
        Ok(candidate)
    }
    fn restore_backup(&mut self, path: &str) -> Result<Applied, String> {
        let backup_path = self.checked_backup(path)?;
        let restored = fs::read_to_string(&backup_path).map_err(|e| e.to_string())?;
        let current = self.env.source()?;
        let operation_id = system::id();
        let backup = self.env.write_config(&current, &restored)?;
        self.record(
            &operation_id,
            "backup_restore",
            "restored",
            vec![self.env.config.display().to_string()],
            Some(backup.clone()),
            None,
            None,
            None,
            false,
            None,
        )?;
        Ok(Applied {
            id: operation_id,
            status: "backup restored".into(),
            backup: Some(backup.display().to_string()),
            commit: None,
            undo_available: false,
        })
    }
    fn delete_backup(&mut self, path: &str) -> Result<(), String> {
        let backup_path = self.checked_backup(path)?;
        fs::remove_file(&backup_path).map_err(|e| e.to_string())?;
        let canonical = backup_path.display().to_string();
        for entry in &mut self.journal {
            if entry
                .backup
                .as_deref()
                .is_some_and(|saved| saved == path || saved == canonical)
            {
                entry.undo_available = false;
            }
        }
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
            Request::Readme { owner, repository } => {
                let repo = Repo::parse(&format!("{owner}/{repository}"))?;
                Ok(Response {
                    snapshot: None,
                    preview: None,
                    applied: None,
                    inventory: None,
                    search: None,
                    capability: None,
                    history: None,
                    updates: None,
                    message: Some(self.github()?.readme(&repo)?),
                })
            }
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
            Request::RestoreBackup { path } => Ok(Response {
                snapshot: None,
                preview: None,
                applied: Some(self.restore_backup(&path)?),
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: None,
                message: None,
            }),
            Request::DeleteBackup { path } => {
                self.delete_backup(&path)?;
                Ok(Response {
                    snapshot: None,
                    preview: None,
                    applied: None,
                    inventory: None,
                    search: None,
                    capability: None,
                    history: None,
                    updates: None,
                    message: Some("Backup deleted".into()),
                })
            }
            Request::Undo { history_id } => Ok(Response {
                snapshot: None,
                preview: None,
                applied: Some(self.undo(&history_id)?),
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: None,
                message: None,
            }),
            Request::PluginReadme { name } => Ok(Response {
                snapshot: None,
                preview: None,
                applied: None,
                inventory: None,
                search: None,
                capability: None,
                history: None,
                updates: None,
                message: Some(self.plugin_readme(&name)?),
            }),
            Request::OpenUrl { url } => {
                self.open_url(&url)?;
                Ok(Response {
                    snapshot: None,
                    preview: None,
                    applied: None,
                    inventory: None,
                    search: None,
                    capability: None,
                    history: None,
                    updates: None,
                    message: Some("opened".into()),
                })
            }
            Request::OpenTerminal => {
                self.open_terminal()?;
                Ok(Response {
                    snapshot: None,
                    preview: None,
                    applied: None,
                    inventory: None,
                    search: None,
                    capability: None,
                    history: None,
                    updates: None,
                    message: Some("opened-terminal".into()),
                })
            }
        }
    }
}

fn origin_repo(env: &Environment, path: &Path) -> Option<Repo> {
    env.git(Some(path), &["remote", "get-url", "origin"])
        .ok()
        .and_then(|url| Repo::parse(&url).ok())
}

fn contained(path: &Path, root: &Path) -> bool {
    let Ok(path) = fs::canonicalize(path) else {
        return false;
    };
    let Ok(root) = fs::canonicalize(root) else {
        return false;
    };
    path.starts_with(root)
}

fn readme_summary(dir: &Path) -> String {
    let bytes = fs::read(dir.join("README.md")).unwrap_or_default();
    let text = String::from_utf8_lossy(&bytes[..bytes.len().min(4_000)]);
    let line = text
        .lines()
        .map(str::trim)
        .find(|line| {
            !line.is_empty()
                && !line.starts_with('#')
                && !line.starts_with('<')
                && !line.starts_with("[!")
                && !line.starts_with("![")
                && !line.starts_with("---")
        })
        .or_else(|| {
            text.lines()
                .map(str::trim)
                .find(|line| line.starts_with("# "))
                .map(|line| line.trim_start_matches('#').trim())
        })
        .unwrap_or("");
    clean_markdown(line).chars().take(140).collect()
}

fn clean_markdown(s: &str) -> String {
    use std::sync::OnceLock;
    static ATTR: OnceLock<regex::Regex> = OnceLock::new();
    static LINK: OnceLock<regex::Regex> = OnceLock::new();
    static URL: OnceLock<regex::Regex> = OnceLock::new();
    let s = ATTR
        .get_or_init(|| regex::Regex::new(r"\{#[^}]*\}").expect("attr"))
        .replace_all(s, "");
    let s = LINK
        .get_or_init(|| regex::Regex::new(r"\[([^\]]+)\]\([^)]*\)").expect("link"))
        .replace_all(&s, "$1");
    let s = URL
        .get_or_init(|| regex::Regex::new(r"https?://\S+").expect("url"))
        .replace_all(&s, "");
    s.replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn catalog_dirs(root: &Path) -> Vec<CatalogItem> {
    let mut items = fs::read_dir(root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            config::component(&name).then_some(CatalogItem {
                summary: readme_summary(&e.path()),
                name,
            })
        })
        .collect::<Vec<_>>();
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

fn omz_plugin_file(dir: &Path, name: &str) -> bool {
    dir.join(format!("{name}.plugin.zsh")).is_file()
}
fn theme_literal_in(dir: &Path, name: &str) -> Option<String> {
    // Theme checkouts live in custom/themes/{name}/, so Oh My Zsh needs a slash
    // theme like {name}/{stem} rather than a bare file name.
    if dir.join(format!("{name}.zsh-theme")).is_file() {
        return Some(format!("{name}/{name}"));
    }
    if dir.join(name).join(format!("{name}.zsh-theme")).is_file() {
        return Some(format!("{name}/{name}"));
    }
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return None;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let os_name = entry.file_name();
        let Some(file_name) = os_name.to_str() else {
            continue;
        };
        if file_name.ends_with(".zsh-theme") && entry.path().is_file() {
            found.push(file_name.trim_end_matches(".zsh-theme").to_string());
        }
    }
    (found.len() == 1).then(|| format!("{name}/{}", found[0]))
}
fn list_custom_dirs(root: &Path) -> Vec<String> {
    fs::read_dir(root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|e| e.file_name().to_str().map(String::from))
        .filter(|n| config::component(n))
        .collect()
}
fn list_themes(custom: &Path) -> Vec<String> {
    let mut names = Vec::new();
    let Ok(entries) = fs::read_dir(custom) else {
        return names;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let os_name = entry.file_name();
        let Some(name) = os_name.to_str() else {
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_file() && name.ends_with(".zsh-theme") {
            let stem = name.trim_end_matches(".zsh-theme");
            if config::component(stem) {
                names.push(stem.to_string());
            }
        } else if file_type.is_dir() && config::component(name) {
            let nested = entry.path().join(format!("{name}.zsh-theme"));
            if nested.is_file() {
                names.push(format!("{name}/{name}"));
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

fn safe_github_url(url: &str) -> Result<String, String> {
    let url = url.trim();
    let rest = url
        .strip_prefix("https://github.com/")
        .ok_or("Only GitHub https links can be opened")?;
    if rest.is_empty()
        || rest.contains(['?', '#', '\\', ' ', '\n', '\r', '\t'])
        || rest.contains("..")
    {
        return Err("Unsupported GitHub link".into());
    }
    let parts = rest.split('/').collect::<Vec<_>>();
    if parts.len() < 2 || parts.len() > 8 || parts.iter().any(|part| !config::component(part)) {
        return Err("Unsupported GitHub link".into());
    }
    Ok(format!("https://github.com/{rest}"))
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
            previous_commit: Some("fedcba9876543210fedcba9876543210fedcba98".into()),
            error: None,
        };
        let encoded = serde_json::to_string(&history).unwrap();
        let decoded: History = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.repo, history.repo);
        assert_eq!(decoded.commit, history.commit);
        assert_eq!(decoded.previous_commit, history.previous_commit);

        let legacy = r#"{"id":"old","at":1,"operation":"configuration","status":"applied","files":[],"backup":null,"undo_available":true,"error":null}"#;
        let decoded: History = serde_json::from_str(legacy).unwrap();
        assert!(decoded.repo.is_none());
        assert!(decoded.commit.is_none());
    }

    #[test]
    fn backup_path_is_scoped_to_managed_directory() {
        let dir = tempfile::tempdir().unwrap();
        let env = Environment::at(dir.path().to_path_buf()).unwrap();
        let backups = env.data.join("backups");
        fs::create_dir_all(&backups).unwrap();
        let managed = backups.join("one.zshrc");
        fs::write(&managed, "ZSH_THEME=\"a\"\nplugins=(git)\n").unwrap();
        let manager = Manager {
            journal_path: env.data.join("history.json"),
            env,
            plans: HashMap::new(),
            journal: vec![],
        };
        assert!(manager
            .checked_backup(&managed.display().to_string())
            .is_ok());
        assert!(manager
            .checked_backup(&dir.path().join("outside.zshrc").display().to_string())
            .is_err());
    }

    #[test]
    fn undo_restores_configuration_from_backup_and_consumes_history_entry() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().to_path_buf();
        let config_path = home.join(".zshrc");
        let original = "ZSH_THEME=\"old\"\nplugins=(git)\n";
        let changed = "ZSH_THEME=\"new\"\nplugins=(git brew)\n";
        fs::write(&config_path, original).unwrap();
        let env = Environment::at(home).unwrap();
        if !env.tool_available("zsh") {
            return;
        }
        let backup = env.write_config(original, changed).unwrap();
        let mut manager = Manager {
            env,
            plans: HashMap::new(),
            journal: vec![History {
                id: "apply-1".into(),
                at: 1,
                operation: "configuration".into(),
                status: "applied".into(),
                files: vec![config_path.display().to_string()],
                backup: Some(backup.display().to_string()),
                undo_available: true,
                repo: None,
                commit: None,
                previous_commit: None,
                error: None,
            }],
            journal_path: dir.path().join("history.json"),
        };
        let applied = manager.undo("apply-1").unwrap();
        assert_eq!(applied.status, "restored");
        assert_eq!(fs::read_to_string(config_path).unwrap(), original);
        assert!(!manager.journal[0].undo_available);
        assert_eq!(manager.journal[1].operation, "undo");
    }

    #[test]
    fn undo_install_refuses_enabled_plugin() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().to_path_buf();
        fs::write(
            home.join(".zshrc"),
            "ZSH_THEME=\"old\"\nplugins=(example)\n",
        )
        .unwrap();
        let env = Environment::at(home.clone()).unwrap();
        let plugin = env.custom.join("plugins/example");
        fs::create_dir_all(plugin.join(".git")).unwrap();
        let mut manager = Manager {
            env,
            plans: HashMap::new(),
            journal: vec![History {
                id: "install-1".into(),
                at: 1,
                operation: "install".into(),
                status: "applied".into(),
                files: vec![plugin.display().to_string()],
                backup: None,
                undo_available: true,
                repo: Some(Repo {
                    owner: "example".into(),
                    name: "example".into(),
                }),
                commit: Some("0123456789012345678901234567890123456789".into()),
                previous_commit: None,
                error: None,
            }],
            journal_path: dir.path().join("history.json"),
        };
        let error = manager.undo("install-1").unwrap_err();
        assert!(error.contains("Disable this plugin"));
        assert!(plugin.exists());
    }

    #[test]
    fn snapshot_lists_official_plugins_and_themes() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().to_path_buf();
        fs::write(
            home.join(".zshrc"),
            "ZSH_THEME=\"robbyrussell\"\nplugins=(git)\n",
        )
        .unwrap();
        let plugin = home.join(".oh-my-zsh/plugins/git");
        fs::create_dir_all(&plugin).unwrap();
        fs::write(
            plugin.join("README.md"),
            "# git\nGit aliases and completions\n",
        )
        .unwrap();
        fs::create_dir_all(home.join(".oh-my-zsh/themes")).unwrap();
        fs::write(
            home.join(".oh-my-zsh/themes/robbyrussell.zsh-theme"),
            "# prompt\n",
        )
        .unwrap();
        let custom_theme = home.join(".oh-my-zsh/custom/themes/powerlevel10k");
        fs::create_dir_all(&custom_theme).unwrap();
        fs::write(custom_theme.join("powerlevel10k.zsh-theme"), "# p10k\n").unwrap();
        let mut manager = Manager {
            env: Environment::at(home).unwrap(),
            plans: HashMap::new(),
            journal: vec![],
            journal_path: dir.path().join("history.json"),
        };
        let snapshot = manager.snap().unwrap();
        assert_eq!(snapshot.official_plugins[0].name, "git");
        assert_eq!(
            snapshot.official_plugins[0].summary,
            "Git aliases and completions"
        );
        assert!(
            !snapshot.themes.iter().any(|t| t == "robbyrussell"),
            "official themes stay out of the configuration picker"
        );
        assert!(snapshot
            .themes
            .iter()
            .any(|t| t == "powerlevel10k/powerlevel10k"));
        assert_eq!(
            manager
                .plugin_readme("git")
                .unwrap()
                .contains("Git aliases"),
            true
        );
    }

    #[test]
    fn checkout_kind_is_detected_from_plugin_or_theme_files() {
        let dir = tempfile::tempdir().unwrap();
        let plugin = dir.path().join("extract");
        fs::create_dir_all(&plugin).unwrap();
        assert!(!omz_plugin_file(&plugin, "extract"));
        fs::write(plugin.join("extract.plugin.zsh"), "#\n").unwrap();
        assert!(omz_plugin_file(&plugin, "extract"));
        let theme = dir.path().join("spaceship-prompt");
        fs::create_dir_all(&theme).unwrap();
        fs::write(theme.join("spaceship.zsh-theme"), "#\n").unwrap();
        assert_eq!(
            theme_literal_in(&theme, "spaceship-prompt").as_deref(),
            Some("spaceship-prompt/spaceship")
        );
        let named = dir.path().join("hyperzsh");
        fs::create_dir_all(&named).unwrap();
        fs::write(named.join("hyperzsh.zsh-theme"), "#\n").unwrap();
        assert_eq!(
            theme_literal_in(&named, "hyperzsh").as_deref(),
            Some("hyperzsh/hyperzsh")
        );
    }

    #[test]
    fn github_links_are_limited_to_safe_https_paths() {
        assert!(
            safe_github_url("https://github.com/ohmyzsh/ohmyzsh/tree/master/plugins/git").is_ok()
        );
        assert!(safe_github_url("https://evil.example/ohmyzsh/ohmyzsh").is_err());
        assert!(safe_github_url("https://github.com/ohmyzsh/ohmyzsh/../../etc/passwd").is_err());
        assert!(safe_github_url("https://github.com/ohmyzsh/ohmyzsh?q=1").is_err());
    }

    #[test]
    #[ignore = "live GitHub smoke test; uses only a temporary HOME"]
    fn live_plugin_lifecycle_smoke_uses_temporary_home() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().to_path_buf();
        fs::write(
            home.join(".zshrc"),
            "ZSH_THEME=\"robbyrussell\"\nplugins=(git)\n",
        )
        .unwrap();
        let env = Environment::at(home.clone()).unwrap();
        let mut manager = Manager {
            env,
            plans: HashMap::new(),
            journal: vec![],
            journal_path: dir.path().join("history.json"),
        };
        let repo = Repo::parse("syi0808/shellsuggest").unwrap();
        let head = manager.github().unwrap().head(&repo).unwrap();
        let install_plan = Plan {
            operation: "install".into(),
            expected_source: None,
            proposed_source: None,
            repo: Some(repo.clone()),
            target: Some(head.clone()),
        };
        let installed = manager
            .apply_install("live-install", &install_plan)
            .unwrap();
        assert_eq!(installed.commit.as_deref(), Some(head.as_str()));
        let path = manager.env.plugin_path(&repo.name).unwrap();
        let commits = manager
            .env
            .git(Some(&path), &["rev-list", "--max-count=2", "HEAD"])
            .unwrap();
        let mut commits = commits.lines();
        let current = commits.next().unwrap().to_string();
        let previous = commits
            .next()
            .expect("repository needs two commits")
            .to_string();
        manager
            .env
            .git(Some(&path), &["checkout", "--detach", &previous])
            .unwrap();
        let update_plan = Plan {
            operation: "update".into(),
            expected_source: None,
            proposed_source: None,
            repo: Some(repo.clone()),
            target: Some(current.clone()),
        };
        let updated = manager.apply_update("live-update", &update_plan).unwrap();
        assert_eq!(updated.commit.as_deref(), Some(current.as_str()));
        let update_history = manager
            .journal
            .iter()
            .find(|h| h.id == "live-update")
            .unwrap();
        assert_eq!(
            update_history.previous_commit.as_deref(),
            Some(previous.as_str())
        );
        manager.undo("live-update").unwrap();
        assert_eq!(manager.env.repo_clean(&path).unwrap(), previous);

        fs::write(
            &manager.env.config,
            "ZSH_THEME=\"robbyrussell\"\nplugins=(git shellsuggest)\n",
        )
        .unwrap();
        let preview = manager
            .plugin_preview("remove".into(), repo.owner.clone(), repo.name.clone(), None)
            .unwrap()
            .preview
            .unwrap();
        let remove_id = preview.id.clone();
        manager.apply(&remove_id).unwrap();
        assert!(!path.exists());
        manager.undo(&remove_id).unwrap();
        assert!(path.exists());
        assert!(manager.env.source().unwrap().contains("shellsuggest"));
    }
}
