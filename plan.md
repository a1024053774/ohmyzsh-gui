# Implementation plan — cross-platform MVP

Owner: user. Execution defaults delegated by user after design answers; no further routine approval round required.

## Goal and boundaries

Deliver a real Tauri 2 desktop utility with `.zshrc` settings, official + GitHub catalogue, optional OS-credential-store token, plugin install/enable/disable/update/remove, commit previews, and recoverable history. Use Apple HIG principles and the Homebrew update list/detail reference. macOS/Linux operate on local zsh; Windows uses default WSL distribution. Never source the user's shell files or plugin scripts.

Preserve unknown text, comments, permissions, symlinks and stale-edit detection. Refuse ambiguous structured editing; raw editor remains available with syntax validation. Official plugins ship with Oh My Zsh and update through the framework as one repository, not individual repositories. No remote publish, signing, telemetry, or changes to the user's live configuration during testing.

## Ordered work

1. Preserve red cases for unsafe path components and parser corruption. Replace naive line rewriting with source-span edits; support raw content and safe literal settings; bind preview to source digest and target paths.
2. Introduce backend snapshot, GitHub client (token remains in backend), local inventory, and settings credential-store status.
3. Implement prepare/apply transactions with preview IDs, fixed target SHA, clean-tree checks, hooks disabled, backup/quarantine, journal and restore. Only trusted backend paths reach subprocesses.
4. Replace browser fake-success fallback with explicit visual-only state. Build live Installed / Discover / Updates / Configuration / Activity screens, preview dialog, settings, keyboard and resize behavior.
5. Test on isolated home and local Git fixtures; preserve filesystem/Git readback. Build actual desktop app, visually inspect it, and package a local macOS artifact. Add CI definition for Linux/Windows without claiming it ran.
6. Focused completion review and status report, distinguishing macOS observed runtime from Linux/Windows unrun validation.

## Acceptance

- No mutation before explicit preview confirmation; cancel changes nothing; stale preview rejected.
- Syntax error/missing zsh leaves original bytes unchanged; symlink remains a symlink; backups and history restore work.
- Unknown or unaudited shell forms are preserved, not interpreted.
- Install uses actual selected GitHub repository and commit, enable is a separate `.zshrc` change, update shows old/new SHA and commit summary, remove is reversible and disables first.
- Real local inventory and restart persist state; unknown update state is not labeled up-to-date.
- Token never enters catalogue payloads, logs or browser storage; anonymous search works and rate limit state is explicit.
- Actual desktop build, frontend visual QA, targeted regression and filesystem integration tests.

## Current unknowns

Cross-platform runtime is only available on this Mac. Windows WSL and Linux adapters can be implemented and checked structurally here; on-device runs remain NOT_RUN until a runner exists. Keep the overall goal active while required evidence is missing.
