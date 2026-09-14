# Cursor/Grok research record — 2026-09-14

Source: Cursor GUI Agent session (model label shown as Extra High Fast; the result identifies itself as a read-only research pass). This record preserves the returned evidence; it is not treated as an implementation oracle.

## Recommendation

For a cross-platform personal desktop utility, the returned comparison recommends Tauri 2 with a Rust core and web UI. It cites the official Tauri documentation for system WebView, small bundles, security, and capabilities:

- https://v2.tauri.app/start/
- https://v2.tauri.app/concept/size/
- https://v2.tauri.app/security/
- https://v2.tauri.app/security/capabilities/

The result also states that Windows should use a WSL-provided zsh environment rather than pretending that native cmd is zsh: https://learn.microsoft.com/en-us/windows/wsl/about

## .zshrc safety

- Treat the file as lossless text and only rewrite the first top-level `ZSH_THEME=...` and `plugins=(...)` assignment.
- Refuse structured writes when `plugins+=`, multiple plugin assignments, nested parentheses, or conditional assignments make the target ambiguous; show read-only text instead.
- Use timestamped backups, temporary file then replace, unified diff, and `zsh -n` syntax validation. `-n`/`NO_EXEC` checks syntax but does not prove runtime safety: https://zsh.sourceforge.io/Doc/Release/Options.html
- Official configuration surfaces are `$ZSH`, `ZSH_THEME`, `plugins=(...)`, and `$ZSH_CUSTOM`: https://github.com/ohmyzsh/ohmyzsh/wiki/Plugins, https://github.com/ohmyzsh/ohmyzsh/wiki/Customization

## Plugin lifecycle

- Custom plugin directory names must match the name in `plugins=(...)`.
- Install into a temporary checkout, pin the selected commit SHA, disable Git hooks, avoid submodules unless explicitly approved, then atomically move into `$ZSH_CUSTOM/plugins`.
- Compare local and upstream commits through GitHub commit/compare APIs; update with fetch plus detached checkout of the selected SHA. Avoid `git pull`/rebase.
- Never source plugins, run install scripts, or execute plugin code from the manager.
- Relevant references: https://git-scm.com/docs/git-clone, https://git-scm.com/docs/githooks, https://docs.github.com/en/rest/commits/commits, https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api

## Token and UI guidance

- Unauthenticated GitHub requests remain available with rate-limit messaging; an optional token raises limits. Store it in the OS credential store with minimum public-read scope.
- Use the Homebrew-like Overview / Configuration / Discover / Updates / Activity structure. Follow Apple HIG for sidebars, split views, toolbars, settings, feedback, alerts, and keyboard focus:
  - https://developer.apple.com/design/human-interface-guidelines/sidebars
  - https://developer.apple.com/design/human-interface-guidelines/split-views
  - https://developer.apple.com/design/human-interface-guidelines/toolbars
  - https://developer.apple.com/design/human-interface-guidelines/settings
  - https://developer.apple.com/design/human-interface-guidelines/alerts
  - https://developer.apple.com/design/human-interface-guidelines/feedback

## Limitations

The result contains useful cited guidance but did not provide reproducible local package-size or memory measurements. It also referenced the earlier Swift prototype state; the current implementation direction is Tauri 2 after the owner's cross-platform decision.
