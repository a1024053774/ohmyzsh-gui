# ohmyzsh-gui intent (draft)

Status: accepted by owner; cross-platform revision (2026-09-14)
Owner: user
Stage: Plan / intent

## Problem

Managing oh-my-zsh settings and third-party plugins currently requires editing shell files and running command-line installation/update commands. The user wants a macOS desktop interface that makes these operations visible and manageable.

## Desired outcome

Create a macOS GUI that can inspect and safely manage oh-my-zsh configuration, and provides a plugin-marketplace-like catalogue for discovering, installing, updating, uninstalling, and viewing plugin state.

A separate requested workstream is to use the visible Cursor CLI/Grok session, when available, for read-only research on oh-my-zsh settings and related implementation options. Results must be treated as research evidence and checked against first-party or local evidence before becoming requirements.

## Accepted v1 scope

- Inspect the local oh-my-zsh installation and relevant zsh startup files.
- Present settings and plugin state in a visual UI.
- Search a curated or defined plugin catalogue.
- Install, update, uninstall, enable, and disable plugins with backups, diff preview, clear progress, and rollback/error state.
- Keep all local file changes attributable and reversible.

The v1 boundary was confirmed by the owner: personal local-only MVP, `.zshrc` first, official Oh My Zsh + GitHub sources, cross-platform Tauri 2 + Rust + web UI preference.

## Confirmed non-goals

- Managing files beyond `.zshrc` in the MVP, including arbitrary shell scripts, aliases/functions as separate surfaces, terminal profiles, or package managers.
- Distribution beyond the owner's Mac in the MVP.
- Whether to support shells other than zsh and oh-my-zsh.
- Whether to execute arbitrary plugin code or only modify files and invoke approved package/git operations.

## Success measures pending confirmation

- A user can understand current oh-my-zsh state without opening `.zshrc` manually.
- A plugin can be found and its provenance/version shown before installation.
- Install/update/uninstall operations produce a preview, backup, observable result, and recoverable failure.
- The app does not silently corrupt shell startup or hide third-party code changes.

## Constraints and unknowns

- Target platform: cross-platform desktop (owner decision); macOS is the primary visual reference.
- Workspace currently contains no implementation or repository history (observed 2026-09-14).
- Current local oh-my-zsh layout, supported plugin sources, catalogue authority, trust policy, GUI technology, and Cursor/Grok availability require confirmation or read-only checks.

## Accepted stage gate

The owner confirmed: personal local-only MVP; `.zshrc` first; official Oh My Zsh + GitHub sources; read-only Cursor/Grok research is allowed; SwiftUI is preferred. Design/spec work may begin. No plugin installation or production-side shell change is authorized yet.

## Related work found (evidence, not requirements)

- [ShellCraft](https://github.com/omarshahine/ShellCraft) is a native macOS GUI that reads and safely writes shell configuration and includes an Oh My Zsh section for themes, plugins, and settings. This is the closest visible precedent.
- [`omz-plugin-browser`](https://github.com/hernanmd/omz-plugin-browser) is a Pharo GUI for listing and enabling/disabling Oh My Zsh plugins.
- Fig Plugin Store is a precedent for one-click terminal plugin discovery, but it is not a native Oh My Zsh manager.
- Official Oh My Zsh documentation describes `$ZSH` (default `~/.oh-my-zsh`), `.zshrc`, `plugins=(...)`, `ZSH_THEME`, and per-plugin README documentation as core surfaces.

The visible Cursor/Grok session in the attached image could not be verified through the current desktop tool surface: Cursor was absent from the exposed app inventory and Terminal inspection was blocked. I therefore used read-only web/official-documentation research as provisional evidence and have not claimed a Grok result.
