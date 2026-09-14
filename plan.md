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

- No mutation before explicit preview confirmation; cancel changes nothing; stale preview rejected. Configuration, install, remove, and update journal their restore data; live update rollback remains an acceptance check.
- Syntax error/missing zsh leaves original bytes unchanged; symlink remains a symlink; backups and history restore work.
- Unknown or unaudited shell forms are preserved, not interpreted.
- Install uses actual selected GitHub repository and commit, enable is a separate `.zshrc` change, update shows old/new SHA and commit summary, remove is reversible and disables first.
- Real local inventory and restart persist state; unknown update state is not labeled up-to-date.
- Token never enters catalogue payloads, logs or browser storage; anonymous search works and rate limit state is explicit.
- Actual desktop build, frontend visual QA, targeted regression and filesystem integration tests.

## Current unknowns

Cross-platform runtime is only available on this Mac. Windows WSL and Linux adapters can be implemented and checked structurally here; on-device runs remain NOT_RUN until a runner exists. Keep the overall goal active while required evidence is missing.

## Current evidence

- Focused red state preserved in `docs/testing/pre-fix.txt`: three parser/path tests failed before the fix for `..`, inline comments, and a similarly named theme variable.
- Green state in `docs/testing/post-fix.txt`: eight Rust tests, including isolated backup/syntax failure tests, passed; `cargo check` passed. The current suite is eleven tests after adding history SHA, configuration restore, and enabled-plugin safety coverage.
- `node --check src/app.js` passed.
- Cross-platform CI definition added at `.github/workflows/ci.yml`; it has not run in this local session.
- Browser preview was visually inspected before the final backend/UI hardening; the browser's localhost navigation policy blocked an exact-revision refresh. The built Tauri app was then launched and inspected directly, so exact-revision native visual evidence is PASS.
- `cargo tauri build --debug --bundles app` passed and produced `src-tauri/target/debug/bundle/macos/ohmyzsh-gui.app`; the app read the real `/Users/luckye/.zshrc`, showed the native sidebar/configuration/discover screens, and produced a no-op configuration preview without applying it. Evidence is recorded in `docs/testing/tauri-build-final.txt` and the grilling ledger.
- `cargo tauri info` confirms macOS Command Line Tools and Rust are available; full Xcode is not installed. DMG bundling was attempted separately and failed in the platform bundler, while the `.app` bundle succeeded.
- Installed plugin inventory now derives the current checkout SHA and restores the GitHub repository from the local operation journal when the app installed it; Activity displays the recorded repository and commit.
- The Updates screen now performs an explicit GitHub check for app-managed plugins and presents a Homebrew-style list with old SHA, new SHA, and commit summary before routing to the existing Review/Apply flow. A local installation without app journal metadata is intentionally shown as local-only and is not guessed as a GitHub source.
- The exact native app completed a live anonymous GitHub search and a read-only install preview for `syi0808/shellsuggest`, locking commit `6d083fbe` and showing separate Apply/Cancel controls. No install or `.zshrc` mutation was performed; evidence is in `docs/testing/live-github-preview-20260914.txt`.
- An ignored live smoke test now exercises install at a real GitHub SHA, update from an older checkout, update Undo to the previous SHA, remove with `.zshrc` disabling, and remove Undo, all inside a temporary HOME. It passed once and is recorded in `docs/testing/live-plugin-lifecycle-20260914.txt`.
- The cross-platform CI matrix now runs `cargo check` in addition to format and offline tests on macOS, Ubuntu, and Windows; local execution of those hosted runners remains pending.
- An optimized release `.app` bundle now builds locally; debug and release bundle outputs are recorded separately under `docs/testing/`.
- The optimized release bundle was launched and visually inspected with the real user configuration loaded; no Apply action was invoked.
- Marketplace safety now requires installation before enabling a GitHub plugin; uninstalled search results cannot be added to `.zshrc` by the Enable control.
- Local Linux/Windows target probes reached native dependency compilation but were blocked by missing cross compilers/sysroots; evidence is in `docs/testing/cross-target-check-20260914.txt`. Hosted CI remains the authoritative cross-platform check.
- Settings now queries only token presence/status from the OS credential store, never the token value; the native dialog visibly reports anonymous access when no token is configured and explains that saving an empty value removes it.
- History semantics are now honest: configuration/install/remove records expose tested Undo; update records persist both current and previous SHA and expose Undo, with live rollback still requiring acceptance evidence.
- Added a dedicated Installed screen following the Homebrew reference, with official and custom plugins, enabled badges, and the same detail/review actions as Discover.
- Native plugin install/update/remove, live GitHub search, token save/readback, Windows WSL, Linux runtime, and CI execution remain NOT_RUN because they require external or other-platform side effects/runners.
