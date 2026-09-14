use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ops::Range;

pub fn digest(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}
pub fn component(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 100
        && s != "."
        && s != ".."
        && !s.starts_with('.')
        && !s.starts_with('-')
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}
pub fn theme_literal(s: &str) -> bool {
    s.is_empty() || s.split('/').all(component)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Values {
    pub theme: String,
    pub plugins: Vec<String>,
}
#[derive(Clone, Debug)]
struct Scalar {
    span: Range<usize>,
    value: String,
}
#[derive(Clone, Debug)]
pub struct Document {
    pub source: String,
    pub values: Values,
    pub warnings: Vec<String>,
    theme: Option<Scalar>,
    plugin_tokens: Vec<Scalar>,
    close: Option<usize>,
    pub prefixes: Vec<usize>,
}

fn pattern(name: &str) -> Regex {
    Regex::new(&format!(
        r"(?m)^(?:export[ \t]+)?{}[ \t]*(\+)?=[ \t]*",
        regex::escape(name)
    ))
    .unwrap()
}
fn scalar(source: &str, name: &str) -> Result<Option<Scalar>, String> {
    let re = pattern(name);
    let matches: Vec<_> = re.captures_iter(source).collect();
    if matches.len() > 1 || matches.iter().any(|m| m.get(1).is_some()) {
        return Err(format!("{name} has multiple or incremental assignments"));
    }
    let Some(m) = matches.first() else {
        return Ok(None);
    };
    let start = m.get(0).unwrap().end();
    let tail = &source[start..];
    let (end, value) = if tail.starts_with(['\"', '\'']) {
        let q = tail.as_bytes()[0] as char;
        let n = tail[1..]
            .find(q)
            .ok_or_else(|| format!("{name}: unclosed quote"))?
            + 1;
        if tail[1..n].contains(['\n', '\r', '\\', '`']) {
            return Err(format!("{name}: dynamic value"));
        }
        (start + n + 1, tail[1..n].to_string())
    } else {
        let n = tail
            .find(|c: char| c.is_whitespace() || c == '#' || c == ';')
            .unwrap_or(tail.len());
        (start + n, tail[..n].to_string())
    };
    let remainder = source[end..].split('\n').next().unwrap_or("").trim();
    if !remainder.is_empty() && !remainder.starts_with('#') {
        return Err(format!("{name}: trailing shell expression"));
    }
    Ok(Some(Scalar {
        span: start..end,
        value,
    }))
}
pub fn literal_path(source: &str, name: &str) -> Result<Option<String>, String> {
    Ok(scalar(source, name)?.map(|s| s.value))
}
impl Document {
    pub fn parse(source: String) -> Self {
        let mut warnings = vec![];
        let mut prefixes = vec![];
        let theme = match scalar(&source, "ZSH_THEME") {
            Ok(x) => x,
            Err(e) => {
                warnings.push(e);
                None
            }
        };
        if let Some(m) = pattern("ZSH_THEME").find(&source) {
            prefixes.push(m.start());
        }
        let re = pattern("plugins");
        let matches: Vec<_> = re.captures_iter(&source).collect();
        let mut tokens = vec![];
        let mut close = None;
        if matches.len() != 1 || matches.first().and_then(|m| m.get(1)).is_some() {
            warnings.push("plugins must have one top-level literal array assignment".into());
        } else {
            let m = matches[0].get(0).unwrap();
            prefixes.push(m.start());
            let start = m.end();
            let bytes = source.as_bytes();
            if bytes.get(start) != Some(&b'(') {
                warnings.push("plugins is not a literal array".into());
            } else {
                let mut i = start + 1;
                while i < bytes.len() {
                    if bytes[i].is_ascii_whitespace() {
                        i += 1;
                        continue;
                    }
                    if bytes[i] == b'#' {
                        while i < bytes.len() && bytes[i] != b'\n' {
                            i += 1
                        }
                        continue;
                    }
                    if bytes[i] == b')' {
                        close = Some(i);
                        break;
                    }
                    let begin = i;
                    let value = if bytes[i] == b'\'' || bytes[i] == b'"' {
                        let q = bytes[i];
                        i += 1;
                        let s = i;
                        while i < bytes.len() && bytes[i] != q {
                            i += 1
                        }
                        let v = source[s..i].to_string();
                        if i < bytes.len() {
                            i += 1
                        };
                        v
                    } else {
                        while i < bytes.len()
                            && !bytes[i].is_ascii_whitespace()
                            && !b")#".contains(&bytes[i])
                        {
                            i += 1
                        }
                        source[begin..i].to_string()
                    };
                    if !component(&value) {
                        warnings.push("plugins contains expansions or nested shell syntax".into());
                        break;
                    }
                    tokens.push(Scalar {
                        span: begin..i,
                        value,
                    });
                }
                if close.is_none() {
                    warnings.push("plugins array is not safely editable".into())
                }
                if let Some(c) = close {
                    let rest = source[c + 1..].split('\n').next().unwrap_or("").trim();
                    if !rest.is_empty() && !rest.starts_with('#') {
                        warnings.push("plugins has a trailing shell expression".into())
                    }
                }
            }
        }
        // Any other assignments could override a top-level edit, even inside a compound command.
        for name in ["plugins", "ZSH_THEME"] {
            let broad = Regex::new(&format!(r"\b{}\s*\+?=", name)).unwrap();
            let count = source
                .lines()
                .filter(|l| !l.trim_start().starts_with('#'))
                .map(|l| broad.find_iter(l.split('#').next().unwrap_or("")).count())
                .sum::<usize>();
            if count > pattern(name).find_iter(&source).count() {
                warnings.push(format!("{name} has non-top-level assignments"));
            }
        }
        let values = Values {
            theme: theme.as_ref().map(|x| x.value.clone()).unwrap_or_default(),
            plugins: tokens.iter().map(|x| x.value.clone()).collect(),
        };
        Self {
            source,
            values,
            warnings,
            theme,
            plugin_tokens: tokens,
            close,
            prefixes,
        }
    }
    pub fn edit(&self, values: &Values) -> Result<String, String> {
        if !self.warnings.is_empty() {
            return Err(self.warnings.join("; "));
        }
        if !theme_literal(&values.theme) || values.plugins.iter().any(|n| !component(n)) {
            return Err("Theme or plugin name contains shell syntax".into());
        }
        let mut unique = values.plugins.clone();
        unique.sort();
        unique.dedup();
        if unique.len() != values.plugins.len() {
            return Err("Duplicate plugin name".into());
        }
        let mut edits: Vec<(Range<usize>, String)> = vec![];
        if values.theme != self.values.theme {
            if let Some(t) = &self.theme {
                edits.push((t.span.clone(), format!("\"{}\"", values.theme)))
            } else {
                return Err(
                    "ZSH_THEME is missing. Add it before sourcing Oh My Zsh in the source editor."
                        .into(),
                );
            }
        }
        for token in &self.plugin_tokens {
            if !values.plugins.contains(&token.value) {
                edits.push((token.span.clone(), String::new()))
            }
        }
        let extra: Vec<_> = values
            .plugins
            .iter()
            .filter(|n| !self.values.plugins.contains(n))
            .cloned()
            .collect();
        if !extra.is_empty() {
            let pos = self.close.ok_or("Missing plugins array")?;
            edits.push((pos..pos, format!(" {}", extra.join(" "))));
        }
        let mut out = self.source.clone();
        edits.sort_by_key(|(r, _)| std::cmp::Reverse(r.start));
        for (r, s) in edits {
            out.replace_range(r, &s)
        }
        Ok(out)
    }
}
pub fn diff(before: &str, after: &str) -> String {
    if before == after {
        return "No changes".into();
    }
    // A full-file unified hunk is exact, including additions/deletions and newline state.
    let a: Vec<_> = before.split_inclusive('\n').collect();
    let b: Vec<_> = after.split_inclusive('\n').collect();
    let mut out = format!(
        "--- .zshrc (current)\n+++ .zshrc (proposed)\n@@ -1,{} +1,{} @@\n",
        a.len(),
        b.len()
    );
    for (sign, lines) in [('-', a), ('+', b)] {
        for l in lines {
            out.push(sign);
            out.push_str(l);
            if !l.ends_with('\n') {
                out.push_str("\n\\ No newline at end of file\n")
            }
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parent_directory_is_never_a_repository_name() {
        for s in [".", "..", "../evil", "x/y", "-x"] {
            assert!(!component(s))
        }
    }
    #[test]
    fn comments_are_not_enabled_plugins() {
        assert_eq!(
            Document::parse("plugins=(git # keep git enabled\n brew)\n".into())
                .values
                .plugins,
            vec!["git", "brew"]
        )
    }
    #[test]
    fn changing_theme_preserves_other_variables() {
        let d = Document::parse(
            "ZSH_THEME=\"old\" # hello\r\nZSH_THEME_RANDOM_CANDIDATES=(one two)\r\nplugins=(git)"
                .into(),
        );
        let s = d
            .edit(&Values {
                theme: "new".into(),
                plugins: vec!["git".into()],
            })
            .unwrap();
        assert_eq!(
            s,
            "ZSH_THEME=\"new\" # hello\r\nZSH_THEME_RANDOM_CANDIDATES=(one two)\r\nplugins=(git)"
        );
    }
    #[test]
    fn noop_and_comments_roundtrip() {
        let s = "ZSH_THEME=\"a\"\r\nplugins=(\r\n git # preserve\r\n brew\r\n)";
        let d = Document::parse(s.into());
        assert_eq!(d.edit(&d.values).unwrap(), s);
        let n = d
            .edit(&Values {
                theme: "a".into(),
                plugins: vec!["git".into(), "sudo".into()],
            })
            .unwrap();
        assert!(n.contains("# preserve\r\n"));
        assert!(n.contains("sudo)"));
    }
    #[test]
    fn dynamic_and_duplicate_fail() {
        for s in [
            "plugins=($(echo git))",
            "plugins=(git)\nplugins+=(brew)",
            "if true; then plugins=(git); fi",
            "plugins=(git)\nplugins=(brew)",
        ] {
            let d = Document::parse(s.into());
            assert!(d.edit(&d.values).is_err(), "{s}");
        }
    }
    #[test]
    fn injected_value_rejected() {
        let d = Document::parse("ZSH_THEME=\"a\"\nplugins=(git)".into());
        assert!(d
            .edit(&Values {
                theme: "$(touch /tmp/no)".into(),
                plugins: vec![]
            })
            .is_err());
    }
}
