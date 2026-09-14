# ohmyzsh-gui design spec

Status: accepted baseline with implementation in progress (cross-platform revision 2026-09-14)
Owner: user
Stage: Design / spec

## Product boundary

A personal, local-only cross-platform desktop application. The primary implementation is Tauri 2 with a Rust core and a web UI that follows Apple Human Interface Guidelines and the supplied Homebrew reference. SwiftUI is no longer the implementation constraint; Apple guidance is used as a visual and interaction reference. The MVP manages the user's `.zshrc` and Oh My Zsh plugin state. It does not manage arbitrary dotfiles, terminal profiles, or system-wide shell policy.

## Accepted user experience

- Main window uses a macOS sidebar for `Overview`, `Configuration`, `Plugin Marketplace`, and `Activity`.
- The marketplace uses a searchable list plus a detail pane. Each plugin shows source, repository, license when verified, version/ref, install state, enabled state, README link, and last checked time.
- Configuration presents parsed `.zshrc` settings as structured rows while preserving an escape hatch to inspect the exact source and diff.
- Install/update/uninstall flows show a preflight summary and diff, then progress and a durable result. Backups and undo/restore are first-class actions; update restore uses the journaled previous SHA and must still be verified against a live checkout.
- Settings uses a standard macOS settings window with a stable toolbar and panes.

The owner accepted this direction and asked implementation to begin.

## Proposed architecture

- Rust `ZshConfigStore`: reads `.zshrc`, identifies `ZSH`, `ZSH_THEME`, `plugins=(...)`, and managed custom-plugin declarations while preserving unknown lines and comments. It resolves HOME/USERPROFILE and reports when zsh is unavailable.
- `ConfigDocument`: lossless text model with source ranges, normalized semantic values, and a generated diff. Writes are atomic: backup, write temporary file, validate syntax, replace, and retain backup metadata.
- `PluginCatalog`: combines the built-in Oh My Zsh plugin index with explicitly selected GitHub repositories. Search and metadata retrieval are read-only network operations and cache results with timestamps.
- `PluginManager`: performs clone/fetch/checkout under `~/.oh-my-zsh/custom/plugins` (or the user-confirmed custom path), never runs plugin code itself, and records operation logs. Enabling a plugin changes the `.zshrc` array only after preview and confirmation.
- `OperationJournal`: records preflight, backup path, commands, stdout/stderr, exit status, affected files, and rollback action.
- Rust commands isolate filesystem/process work behind Tauri invoke handlers; the web UI owns presentation state and never directly writes shell files.

## Safety and trust boundaries

- Never silently overwrite `.zshrc`; create a timestamped backup before every mutation.
- Parse and validate the resulting `.zshrc` with `zsh -n` before replacement. If validation fails, restore the previous file and show the exact diagnostic.
- Treat GitHub repositories as untrusted source. Display owner/repository, default branch or pinned ref, commit SHA, license evidence, and README before install.
- Do not execute install scripts, post-checkout hooks, or plugin code. Git operations are limited to approved repository URLs and a controlled destination.
- Uninstall removes the managed checkout and edits the plugin list only after a diff preview; restore must be possible from the operation journal.
- Network failures leave the existing configuration untouched and expose cached/stale state clearly.

## Apple design alignment

- Use a macOS sidebar and split view for top-level navigation and list/detail workflows.
- Use a stable settings toolbar with panes and restore the last viewed pane.
- Put frequent actions in a concise toolbar and expose every toolbar action in the menu bar.
- Use SF Symbols and system accent colors; support Light/Dark/Auto appearance.
- Prefer passive status indicators for routine progress; reserve alerts for critical, actionable failures or irreversible actions. Keep Cancel available and support Escape/Command-Period where appropriate.
- Keep layouts resizable and keyboard navigable; provide search and command shortcuts.

## Acceptance evidence for the design

- A fixture `.zshrc` round-trips without losing comments or unknown lines.
- A malformed edit is rejected by validation and leaves the original file intact.
- A catalogue result identifies its source and freshness.
- A plugin install/update/uninstall dry run produces a visible diff, backup record, and operation result without executing plugin code.
- The UI presents the same state as the filesystem and operation journal after restart.

## Resolved design decisions

- DQ1: Use both sources. Built-in Oh My Zsh plugins come from a local index; GitHub search supplies third-party results.
- DQ2: GitHub Token is optional. Unauthenticated search remains available with visible rate-limit state; the token is stored locally by the app and never committed.
- DQ3: Record the installed commit SHA. Updates show current/new SHA and a change summary before Apply.
- DQ4: Require an explicit confirmation after the diff/update preview for install, update, uninstall, and `.zshrc` changes.
- DQ5: Build the real cross-platform app now with Tauri 2 + Rust + web UI; validate Rust core independently and verify the GUI on a machine with Tauri prerequisites.

## Technology decision

Tauri 2 + Rust is the default because it keeps the privileged filesystem/process core small and fast, uses the platform WebView instead of bundling Chromium, and can still implement the Apple-inspired sidebar, split view, badges, toolbar, keyboard navigation, and light/dark appearance in CSS. Flutter remains a fallback if native WebView rendering or accessibility proves insufficient; Electron and Avalonia are deferred because their runtime or platform fit is less favorable for this local utility.

## References

- Apple Human Interface Guidelines: [Design principles](https://developer.apple.com/design/human-interface-guidelines/design-principles), [Sidebars](https://developer.apple.com/design/human-interface-guidelines/sidebars), [Settings](https://developer.apple.com/design/human-interface-guidelines/settings), [Toolbars](https://developer.apple.com/design/human-interface-guidelines/toolbars), [Feedback](https://developer.apple.com/design/human-interface-guidelines/feedback), [Alerts](https://developer.apple.com/design/human-interface-guidelines/alerts).
- [Oh My Zsh repository](https://github.com/ohmyzsh/ohmyzsh) and [plugin documentation](https://github.com/ohmyzsh/ohmyzsh/wiki/Plugins).
