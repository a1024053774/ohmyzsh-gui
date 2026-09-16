use crate::config;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::NamedTempFile;

#[derive(Clone, Debug)]
pub struct Environment {
    pub home: PathBuf,
    pub config: PathBuf,
    pub data: PathBuf,
    pub zsh: PathBuf,
    pub custom: PathBuf,
    pub label: String,
    pub wsl: Option<String>,
}
fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}
impl Environment {
    pub fn discover() -> Result<Self, String> {
        #[cfg(windows)]
        {
            let probe = |key: &str| -> Result<String, String> {
                let o = Command::new("wsl.exe")
                    .args(["--exec", "printenv", key])
                    .output()
                    .map_err(|_| "Install WSL with zsh before opening your shell configuration")?;
                if !o.status.success() {
                    return Err("The default WSL distribution is unavailable".into());
                }
                Ok(String::from_utf8_lossy(&o.stdout).trim().into())
            };
            let distro = probe("WSL_DISTRO_NAME")?;
            let unix_home = probe("HOME")?;
            let home = PathBuf::from(format!(
                "\\\\wsl.localhost\\{}{}",
                distro,
                unix_home.replace('/', "\\")
            ));
            let mut e = Self::at(home)?;
            e.wsl = Some(distro.clone());
            e.label = format!("Windows · WSL {distro}");
            return Ok(e);
        }
        #[cfg(not(windows))]
        {
            let home = if cfg!(debug_assertions) {
                env::var_os("OHMYZSH_GUI_HOME").map(PathBuf::from)
            } else {
                None
            }
            .or_else(|| env::var_os("HOME").map(PathBuf::from))
            .ok_or("HOME is unavailable")?;
            let mut e = Self::at(home)?;
            if env::var_os("OHMYZSH_GUI_HOME").is_none() {
                if let Some(z) = env::var_os("ZDOTDIR") {
                    e.config = PathBuf::from(z).join(".zshrc");
                    e.resolve_layout()?;
                }
            }
            Ok(e)
        }
    }
    pub fn at(home: PathBuf) -> Result<Self, String> {
        let mut s = Self {
            config: home.join(".zshrc"),
            data: home.join(".ohmyzsh-gui"),
            zsh: home.join(".oh-my-zsh"),
            custom: home.join(".oh-my-zsh/custom"),
            label: env::consts::OS.into(),
            wsl: None,
            home,
        };
        s.resolve_layout()?;
        Ok(s)
    }
    fn expand(&self, s: &str) -> Result<PathBuf, String> {
        let home = self.home.to_string_lossy();
        let zsh = self.zsh.to_string_lossy();
        let v = s
            .replace("${HOME}", &home)
            .replace("$HOME", &home)
            .replace("${ZSH}", &zsh)
            .replace("$ZSH", &zsh);
        let v = if v == "~" {
            home.to_string()
        } else if v.starts_with("~/") {
            format!("{home}/{}", &v[2..])
        } else {
            v
        };
        if v.contains(['$', '`', '\n']) {
            return Err("Dynamic ZSH/ZSH_CUSTOM path needs manual configuration; no shell evaluation is performed".into());
        }
        let p = PathBuf::from(v);
        if !p.is_absolute() {
            return Err("ZSH paths must be absolute".into());
        }
        Ok(p)
    }
    pub fn resolve_layout(&mut self) -> Result<(), String> {
        if let Ok(s) = fs::read_to_string(&self.config) {
            if let Some(p) = config::literal_path(&s, "ZSH")? {
                self.zsh = self.expand(&p)?
            }
            self.custom = self.zsh.join("custom");
            if let Some(p) = config::literal_path(&s, "ZSH_CUSTOM")? {
                self.custom = self.expand(&p)?
            }
        }
        Ok(())
    }
    pub fn shell_path(&self, p: &Path) -> Result<String, String> {
        if let Some(d) = &self.wsl {
            let prefix = format!("\\\\wsl.localhost\\{d}\\");
            let s = p.to_string_lossy();
            let r = s
                .strip_prefix(&prefix)
                .ok_or("Path is outside the selected WSL distribution")?;
            Ok(format!("/{}", r.replace('\\', "/")))
        } else {
            Ok(p.to_string_lossy().into())
        }
    }
    fn command(&self, tool: &str) -> Command {
        if let Some(d) = &self.wsl {
            let mut c = Command::new("wsl.exe");
            c.args(["--distribution", d, "--exec", tool]);
            c
        } else {
            Command::new(if tool == "zsh" && cfg!(target_os = "macos") {
                "/bin/zsh"
            } else {
                tool
            })
        }
    }
    pub fn tool_available(&self, t: &str) -> bool {
        self.command(t)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    pub fn syntax(&self, s: &str) -> Result<(), String> {
        fs::create_dir_all(&self.data).map_err(err)?;
        let mut temp = NamedTempFile::new_in(&self.data).map_err(err)?;
        temp.write_all(s.as_bytes()).map_err(err)?;
        let o = self
            .command("zsh")
            .args(["-f", "-n", "--", &self.shell_path(temp.path())?])
            .env("ZDOTDIR", &self.data)
            .output()
            .map_err(|_| "zsh is unavailable; configuration was not changed")?;
        if o.status.success() {
            Ok(())
        } else {
            Err(format!(
                "zsh syntax check: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            ))
        }
    }
    pub fn git(&self, p: Option<&Path>, args: &[&str]) -> Result<String, String> {
        let hooks = self.data.join("empty-hooks");
        fs::create_dir_all(&hooks).map_err(err)?;
        let mut c = self.command("git");
        c.args([
            "-c",
            &format!("core.hooksPath={}", self.shell_path(&hooks)?),
            "-c",
            "core.fsmonitor=false",
            "-c",
            "credential.helper=",
            "-c",
            "protocol.file.allow=never",
            "-c",
            "http.lowSpeedLimit=1",
            "-c",
            "http.lowSpeedTime=30",
        ]);
        if let Some(p) = p {
            c.args(["-C", &self.shell_path(p)?]);
        }
        c.args(args)
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env(
                "GIT_CONFIG_GLOBAL",
                if cfg!(windows) { "NUL" } else { "/dev/null" },
            )
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE");
        output(c.output().map_err(|e| format!("git unavailable: {e}"))?)
    }
    pub fn source(&self) -> Result<String, String> {
        fs::read_to_string(&self.config).map_err(|e| format!("{}: {e}", self.config.display()))
    }
    pub fn write_config(&self, expected: &str, proposed: &str) -> Result<PathBuf, String> {
        self.syntax(proposed)?;
        let target = fs::canonicalize(&self.config).map_err(err)?;
        if self.source()? != expected {
            return Err(".zshrc changed since preview. Reload and preview again.".into());
        }
        let backups = self.data.join("backups");
        fs::create_dir_all(&backups).map_err(err)?;
        let backup = backups.join(format!("{}.zshrc", id()));
        fs::copy(&target, &backup).map_err(err)?;
        let mut file =
            NamedTempFile::new_in(target.parent().ok_or("Missing config parent")?).map_err(err)?;
        file.as_file()
            .set_permissions(fs::metadata(&target).map_err(err)?.permissions())
            .map_err(err)?;
        file.write_all(proposed.as_bytes()).map_err(err)?;
        file.as_file().sync_all().map_err(err)?;
        // Preserve symlink identity, metadata, and fail without deleting the original.
        if self.source()? != expected || fs::canonicalize(&self.config).map_err(err)? != target {
            return Err("Config changed during Apply; original retained".into());
        }
        file.persist(&target).map_err(err)?;
        if self.source()? != proposed {
            return Err("Write readback mismatch".into());
        }
        Ok(backup)
    }
    pub fn plugin_path(&self, name: &str) -> Result<PathBuf, String> {
        self.managed_path("plugins", name)
    }
    pub fn theme_path(&self, name: &str) -> Result<PathBuf, String> {
        self.managed_path("themes", name)
    }
    fn managed_path(&self, kind: &str, name: &str) -> Result<PathBuf, String> {
        if !config::component(name) {
            return Err("Invalid plugin directory name".into());
        }
        let p = self.custom.join(kind).join(name);
        if fs::symlink_metadata(&p).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Linked plugin checkout is read-only".into());
        }
        Ok(p)
    }
    pub fn env_quarantine(&self, name: &str, id: &str) -> Result<PathBuf, String> {
        let d = self.data.join("quarantine");
        fs::create_dir_all(&d).map_err(err)?;
        Ok(d.join(format!("{}-{}", name, id)))
    }
    pub fn repo_clean(&self, p: &Path) -> Result<String, String> {
        if !p.join(".git").is_dir()
            || fs::symlink_metadata(p.join(".git")).is_ok_and(|m| m.file_type().is_symlink())
        {
            return Err("Only normal Git checkouts can be managed".into());
        }
        if !self
            .git(Some(p), &["status", "--porcelain", "--untracked-files=all"])?
            .is_empty()
        {
            return Err("Checkout has local changes; commit or move them before applying".into());
        }
        if self
            .git(
                Some(p),
                &["config", "--local", "--get-regexp", "^filter\\."],
            )
            .is_ok()
        {
            return Err("Checkout uses custom Git filters; manual review required".into());
        }
        self.git(Some(p), &["rev-parse", "HEAD"])
    }
}
pub fn output(o: Output) -> Result<String, String> {
    if o.status.success() {
        Ok(String::from_utf8_lossy(&o.stdout).trim().into())
    } else {
        Err(String::from_utf8_lossy(&o.stderr)
            .trim()
            .chars()
            .take(1500)
            .collect())
    }
}
pub fn seconds() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub fn id() -> String {
    format!(
        "{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        std::process::id()
    )
}
pub fn write_json<T: serde::Serialize>(p: &Path, v: &T) -> Result<(), String> {
    fs::create_dir_all(p.parent().ok_or("Missing parent")?).map_err(err)?;
    let mut f = NamedTempFile::new_in(p.parent().unwrap()).map_err(err)?;
    serde_json::to_writer_pretty(&mut f, v).map_err(err)?;
    f.as_file().sync_all().map_err(err)?;
    f.persist(p).map_err(err)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Document, Values};

    #[test]
    fn applies_valid_config_in_isolated_home() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().to_path_buf();
        fs::write(
            home.join(".zshrc"),
            "ZSH_THEME=\"old\"\nplugins=(git)\n# keep\n",
        )
        .unwrap();
        let env = Environment::at(home).unwrap();
        if !env.tool_available("zsh") {
            return;
        }
        let before = env.source().unwrap();
        let d = Document::parse(before.clone());
        let after = d
            .edit(&Values {
                theme: "new".into(),
                plugins: vec!["git".into(), "brew".into()],
            })
            .unwrap();
        let backup = env.write_config(&before, &after).unwrap();
        assert_eq!(env.source().unwrap(), after);
        assert_eq!(fs::read_to_string(backup).unwrap(), before);
    }

    #[test]
    fn invalid_config_never_replaces_original() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path().to_path_buf();
        fs::write(home.join(".zshrc"), "ZSH_THEME=\"old\"\nplugins=(git)\n").unwrap();
        let env = Environment::at(home).unwrap();
        let before = env.source().unwrap();
        assert!(env.write_config(&before, "if [[ broken\n").is_err());
        assert_eq!(env.source().unwrap(), before);
    }
}
