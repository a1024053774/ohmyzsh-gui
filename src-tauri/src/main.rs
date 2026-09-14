use serde::Serialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Clone)]
pub struct ZshState {
    pub path: String,
    pub source: String,
    pub theme: Option<String>,
    pub plugins: Vec<String>,
    pub zsh_available: bool,
}

fn home_dir() -> PathBuf {
    if cfg!(windows) {
        env::var_os("USERPROFILE")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
    }
}
fn zshrc_path() -> PathBuf {
    home_dir().join(".zshrc")
}
fn parse_theme(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        let t = line.trim();
        if !t.starts_with("ZSH_THEME") || !t.contains('=') {
            return None;
        }
        let value = t.split_once('=')?.1.trim().trim_matches(['\"', '\'']);
        let value = value.split('#').next()?.trim();
        (!value.is_empty()).then(|| value.to_string())
    })
}
fn parse_plugins(source: &str) -> Vec<String> {
    let mut body = String::new();
    let mut in_block = false;
    for line in source.lines() {
        let t = line.trim();
        if !in_block && t.starts_with("plugins") && t.contains("(") {
            in_block = true;
            body.push_str(t.split_once('(').map(|(_, r)| r).unwrap_or(""));
            body.push(' ');
            if t.contains(')') {
                break;
            }
        } else if in_block {
            body.push_str(t);
            body.push(' ');
            if t.contains(')') {
                break;
            }
        }
    }
    body.split(')')
        .next()
        .unwrap_or("")
        .split_whitespace()
        .map(|v| v.trim_matches(['\"', '\'']).to_string())
        .filter(|v| !v.is_empty() && !v.starts_with('#'))
        .collect()
}
fn read_source() -> Result<String, String> {
    fs::read_to_string(zshrc_path())
        .map_err(|e| format!("Could not read {}: {e}", zshrc_path().display()))
}
fn rendered_source(source: &str, theme: Option<&str>, plugins: &[String]) -> String {
    let mut lines = Vec::new();
    let mut skipped_plugins = false;
    let mut replaced_theme = false;
    let mut in_plugins = false;
    for line in source.lines() {
        let t = line.trim();
        if t.starts_with("ZSH_THEME") && t.contains('=') {
            if !replaced_theme {
                lines.push(format!("ZSH_THEME=\"{}\"", theme.unwrap_or("")));
                replaced_theme = true;
            }
            continue;
        }
        if !in_plugins && t.starts_with("plugins") && t.contains('(') {
            lines.push(format!("plugins=({})", plugins.join(" ")));
            skipped_plugins = true;
            in_plugins = !t.contains(')');
            continue;
        }
        if in_plugins {
            if t.contains(')') {
                in_plugins = false;
            }
            continue;
        }
        lines.push(line.to_string());
    }
    if !replaced_theme {
        lines.push(format!("ZSH_THEME=\"{}\"", theme.unwrap_or("")));
    }
    if !skipped_plugins {
        lines.push(format!("plugins=({})", plugins.join(" ")));
    }
    let mut out = lines.join("\n");
    out.push('\n');
    out
}
fn diff(old: &str, new: &str) -> String {
    let a: Vec<_> = old.lines().collect();
    let b: Vec<_> = new.lines().collect();
    let mut out = vec![
        "--- .zshrc (current)".to_string(),
        "+++ .zshrc (preview)".to_string(),
    ];
    for i in 0..a.len().max(b.len()) {
        match (a.get(i), b.get(i)) {
            (Some(x), Some(y)) if x == y => out.push(format!("  {x}")),
            (Some(x), Some(y)) => {
                out.push(format!("- {x}"));
                out.push(format!("+ {y}"));
            }
            (Some(x), None) => out.push(format!("- {x}")),
            (None, Some(y)) => out.push(format!("+ {y}")),
            _ => {}
        }
    }
    out.join("\n")
}
fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn safe_component(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 100
        && v.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}
fn hooks_path() -> &'static str {
    if cfg!(windows) {
        "NUL"
    } else {
        "/dev/null"
    }
}
fn plugin_dir(owner: &str, repository: &str) -> Result<PathBuf, String> {
    if !safe_component(owner) || !safe_component(repository) {
        return Err("Invalid GitHub owner or repository name".into());
    }
    Ok(home_dir()
        .join(".oh-my-zsh")
        .join("custom")
        .join("plugins")
        .join(repository))
}
fn run_git(args: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    let mut c = Command::new("git");
    c.args(args);
    if let Some(dir) = cwd {
        c.current_dir(dir);
    }
    let out = c
        .output()
        .map_err(|e| format!("Could not start git: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

#[tauri::command]
fn read_zshrc() -> Result<ZshState, String> {
    let path = zshrc_path();
    let source = read_source()?;
    Ok(ZshState {
        path: path.display().to_string(),
        theme: parse_theme(&source),
        plugins: parse_plugins(&source),
        zsh_available: Command::new("zsh").arg("--version").output().is_ok(),
        source,
    })
}
#[derive(Debug, Serialize)]
pub struct DiffResult {
    pub diff: String,
    pub proposed_source: String,
}
#[tauri::command]
fn preview_zshrc(theme: Option<String>, plugins: Vec<String>) -> Result<DiffResult, String> {
    let source = read_source()?;
    let proposed = rendered_source(&source, theme.as_deref(), &plugins);
    Ok(DiffResult {
        diff: diff(&source, &proposed),
        proposed_source: proposed,
    })
}
#[derive(Debug, Serialize)]
pub struct ApplyResult {
    pub backup: String,
}
#[tauri::command]
fn apply_zshrc(theme: Option<String>, plugins: Vec<String>) -> Result<ApplyResult, String> {
    let path = zshrc_path();
    let source = read_source()?;
    let proposed = rendered_source(&source, theme.as_deref(), &plugins);
    let backup_dir = home_dir().join(".ohmyzsh-gui").join("backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    let backup = backup_dir.join(format!("zshrc-{}", timestamp()));
    fs::copy(&path, &backup).map_err(|e| format!("Backup failed: {e}"))?;
    let temp = path.with_extension(format!("ohmyzsh-gui-{}", timestamp()));
    fs::write(&temp, proposed.as_bytes()).map_err(|e| e.to_string())?;
    let check = Command::new("zsh")
        .args(["-n", temp.to_string_lossy().as_ref()])
        .output();
    if let Ok(out) = check {
        if !out.status.success() {
            let _ = fs::remove_file(&temp);
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
    } else if !cfg!(windows) {
        let _ = fs::remove_file(&temp);
        return Err("zsh is unavailable; refusing to replace .zshrc".into());
    }
    fs::rename(&temp, &path)
        .or_else(|_| {
            fs::remove_file(&path)?;
            fs::rename(&temp, &path)
        })
        .map_err(|e| format!("Atomic replacement failed: {e}"))?;
    Ok(ApplyResult {
        backup: backup.display().to_string(),
    })
}
#[derive(Debug, Serialize)]
pub struct PluginResult {
    pub path: String,
    pub commit: String,
}
#[tauri::command]
fn install_plugin(owner: String, repository: String) -> Result<PluginResult, String> {
    let dir = plugin_dir(&owner, &repository)?;
    if dir.exists() {
        return Err("Plugin directory already exists; choose Update instead".into());
    }
    let parent = dir.parent().ok_or("Invalid plugin path")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let temp = parent.join(format!(".{}-install-{}", repository, timestamp()));
    let url = format!("https://github.com/{owner}/{repository}.git");
    let result = (|| {
        run_git(
            &[
                "-c",
                &format!("core.hooksPath={}", hooks_path()),
                "clone",
                "--filter=blob:none",
                "--no-recurse-submodules",
                &url,
                temp.to_string_lossy().as_ref(),
            ],
            None,
        )?;
        let commit = run_git(&["rev-parse", "HEAD"], Some(&temp))?;
        fs::rename(&temp, &dir).map_err(|e| format!("Atomic plugin install failed: {e}"))?;
        Ok(PluginResult {
            path: dir.display().to_string(),
            commit,
        })
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&temp);
    }
    result
}
#[tauri::command]
fn update_plugin(owner: String, repository: String) -> Result<PluginResult, String> {
    let dir = plugin_dir(&owner, &repository)?;
    if !dir.exists() {
        return Err("Plugin is not installed".into());
    }
    let hooks = format!("core.hooksPath={}", hooks_path());
    run_git(
        &["-c", &hooks, "fetch", "--tags", "--prune", "origin"],
        Some(&dir),
    )?;
    let remote = run_git(&["symbolic-ref", "refs/remotes/origin/HEAD"], Some(&dir))
        .or_else(|_| run_git(&["rev-parse", "origin/HEAD"], Some(&dir)))?;
    let target = if remote.starts_with("refs/remotes/") {
        remote
    } else {
        "origin/HEAD".to_string()
    };
    run_git(&["-c", &hooks, "checkout", "--detach", &target], Some(&dir))?;
    let commit = run_git(&["rev-parse", "HEAD"], Some(&dir))?;
    Ok(PluginResult {
        path: dir.display().to_string(),
        commit,
    })
}
#[tauri::command]
fn uninstall_plugin(owner: String, repository: String) -> Result<String, String> {
    let dir = plugin_dir(&owner, &repository)?;
    if !dir.exists() {
        return Err("Plugin is not installed".into());
    }
    fs::remove_dir_all(&dir).map_err(|e| format!("Uninstall failed: {e}"))?;
    Ok(dir.display().to_string())
}
#[tauri::command]
fn set_github_token(token: String) -> Result<(), String> {
    let dir = home_dir().join(".config").join("ohmyzsh-gui");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("settings.json");
    let value = serde_json::json!({"github_token":token});
    fs::write(
        path,
        serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(dir.join("settings.json"), fs::Permissions::from_mode(0o600))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_zshrc,
            preview_zshrc,
            apply_zshrc,
            install_plugin,
            update_plugin,
            uninstall_plugin,
            set_github_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiline_plugins_and_theme() {
        let source = "# keep\nZSH_THEME=\"robbyrussell\"\nplugins=(\n  git\n  brew\n)\nexport PATH=\"$HOME/bin:$PATH\"\n";
        assert_eq!(parse_theme(source).as_deref(), Some("robbyrussell"));
        assert_eq!(parse_plugins(source), vec!["git", "brew"]);
    }

    #[test]
    fn renders_managed_values_and_preserves_unknown_lines() {
        let source = "# keep\nZSH_THEME=\"robbyrussell\"\nplugins=(git)\nexport FOO=bar\n";
        let updated = rendered_source(source, Some("agnoster"), &["git".into(), "brew".into()]);
        assert!(updated.contains("# keep"));
        assert!(updated.contains("export FOO=bar"));
        assert!(updated.contains("ZSH_THEME=\"agnoster\""));
        assert!(updated.contains("plugins=(git brew)"));
    }

    #[test]
    fn rejects_path_traversal_components() {
        assert!(!safe_component("../evil"));
        assert!(!safe_component("owner/repo"));
        assert!(safe_component("zsh-autosuggestions"));
    }
}

#[cfg(test)]
mod regression {
    use super::*;
    #[test]
    fn parent_directory_is_never_a_repository_name() { assert!(!safe_component("..")); }
    #[test]
    fn comments_are_not_enabled_plugins() {
        assert_eq!(parse_plugins("plugins=(git # keep git enabled\n brew)\n"), vec!["git", "brew"]);
    }
    #[test]
    fn changing_theme_preserves_other_variables() {
        let result = rendered_source("ZSH_THEME=\"old\"\nZSH_THEME_RANDOM_CANDIDATES=(one two)\nplugins=(git)\n", Some("new"), &["git".into()]);
        assert!(result.contains("ZSH_THEME_RANDOM_CANDIDATES=(one two)"));
    }
}
