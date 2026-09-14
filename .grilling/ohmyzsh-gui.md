---
session_id: grilling-ohmyzsh-gui-20260914
status: active
topic: oh-my-zsh macOS GUI configuration and plugin manager
created_at: 2026-09-14T15:50:00+08:00
updated_at: 2026-09-14T20:45:00+08:00
last_round: 10
---

## Goal

- G1 | state: inferred | source: user request | stage: intent | owner: user | Build a macOS desktop GUI for managing oh-my-zsh settings and plugins.
- G2 | state: inferred | source: user request | stage: intent | owner: user | Provide a visual plugin marketplace-like experience to search, install, update, uninstall, and manage plugins.
- G3 | state: inferred | source: user request | stage: intent | owner: user | Use Cursor CLI with its Grok model to research oh-my-zsh settings and related implementation options, if that CLI is available in the opened sidebar.

## Non-goals

- NG1 | state: confirmed | source: round 2 | First release focuses on `.zshrc`; broader shell files and terminal profile management are out of scope for MVP.
- NG2 | state: confirmed | source: round 2 | MVP is for the owner's own Mac; broader distribution is deferred.
- NG3 | state: confirmed | source: attached image interpretation | The text shown inside the image is treated as source material/background evidence, not as additional user instructions.

## Confirmed requirements and constraints

- C1 | state: confirmed | source: round 2 + round 4 | Platform target is cross-platform desktop; macOS is the primary design reference.
- C2 | state: confirmed | source: round 2 | The app must expose visual management of `.zshrc` and Oh My Zsh plugin state.
- C3 | state: confirmed | source: round 2 | Plugin lifecycle includes search, install, update, uninstall, enable/disable, and management.
- C4 | state: unknown | source: workspace inspection | No project code, canonical requirements artifact, or repository history exists in the workspace.

## Decision tree

- Plan / intent: accepted in round 2.
- Design / spec: accepted in round 4.
- Build / implementation: active; Tauri source and tests are present.
- Test / acceptance: not applicable yet.
- Deploy / maintain: not applicable yet.

## Decisions

- D1 | state: confirmed | source: round 1 | The earliest incomplete stage is Plan / intent.
- D2 | state: inferred | source: round 1 | Draft a canonical `intent.md` in the workspace; user is the named owner for acceptance.
- D3 | state: superseded | source: round 2, superseded round 4 | Product boundary is personal MVP; plugin sources are official Oh My Zsh + GitHub; SwiftUI-only is superseded by a cross-platform stack.

## Assumptions and facts

- A1 | state: observed | source: filesystem inspection | `/Users/luckye/Documents/ohmyzsh-gui` exists and is empty apart from files created in this round.
- A2 | state: observed | source: filesystem inspection | No Git repository is present at the workspace root.
- A3 | state: observed | source: attached image | Cursor Agent UI shows a Grok 4.6 option and working directory `~/Documents/ohmyzsh-gui`; whether it is currently interactive/available to this agent is unknown.
- A4 | state: unknown | source: external/runtime check needed | Current oh-my-zsh installation layout, shell configuration format, plugin sources, and update mechanisms on the target Mac.
- A5 | state: unknown | source: external/runtime check needed | Whether a Cursor CLI/Grok session is open and can be safely used for a read-only research prompt.

## Open questions

- Q1 | state: confirmed | source: round 2 | Personal local-only MVP first; distribution deferred.
- Q2 | state: confirmed | source: round 2 | MVP scope is `.zshrc` and Oh My Zsh settings/plugin management; broader files deferred.
- Q3 | state: confirmed | source: round 2 | Sources are official Oh My Zsh + GitHub; trust indicators and review-before-install remain design requirements.
- Q4 | state: confirmed | source: round 2 | Use the available `agent` CLI/Cursor Grok session for read-only research.
- Q5 | state: superseded | source: round 2, round 4 | SwiftUI-only was superseded by owner's cross-platform requirement; use Tauri 2 + Rust + web UI.

## Current frontier

- F1 | state: resolved | source: round 2 | Q1-Q5 confirmed; proceed to design/spec frontier.
- F2 | state: resolved | source: round 4 | Owner chose combined catalogue, optional GitHub token, Homebrew-like update view, commit SHA tracking, and confirmation after preview.
- F3 | state: resolved | source: round 4 | Owner changed platform to cross-platform and delegated technology choice; proceed with Tauri 2 + Rust + web UI.
- F4 | state: resolved | source: round 5 | Cursor/Grok GUI research returned cited Tauri, zsh safety, plugin lifecycle, token, and Apple HIG guidance; record saved under `docs/research/`.
- F5 | state: active | source: round 6-10 | Added conservative source-span parser, stale-preview transaction, isolated backup/syntax tests, OS credential store, GitHub client, inventory, update checking, and Tauri dispatch boundary. The macOS Tauri app builds and has been visually inspected; installed plugin SHA/repository metadata now survives into Discover and Activity. Other-platform runtime and side-effecting plugin lifecycle evidence remain open.

## Risks and conflicts

- R1 | state: open | Editing shell startup files can break interactive shells; safe writes require backup, parse/validation, diff preview, and rollback.
- R2 | state: open | Installing arbitrary shell plugins executes third-party shell code; source provenance, review status, permissions, and recovery need explicit policy.
- R3 | state: open | A marketplace catalogue needs a source of truth, metadata schema, freshness/update behavior, and handling of deleted or malicious repositories.
- R4 | state: mitigated | source: round 5 | Cursor/Grok GUI research was captured in `docs/research/cursor-grok-20260914.md`; direct CLI execution remains unverified because Terminal inspection was blocked.
- R5 | state: open | Cross-platform zsh availability and platform-specific home/config paths vary; runtime capability checks are required.
- R6 | state: mitigated | source: round 6 | Token uses keyring crate with native backend per OS; unavailable stores fail closed while anonymous search remains available.
- R7 | state: mitigated | source: round 9 | Browser localhost policy blocked exact-revision browser refresh, but the exact built Tauri app was launched and inspected with native macOS accessibility/screenshot evidence.

## Final baseline

- Intent baseline accepted by the owner in Round 2: personal macOS MVP, `.zshrc` first, official Oh My Zsh + GitHub sources, SwiftUI, and authorized read-only Cursor/Grok research.
- Stage gate: intent accepted; design/spec accepted in round 4; implementation active with a built/inspected macOS Tauri artifact and Rust/frontend tests. Cross-platform and side-effecting acceptance remain open.

## Changelog

- Round 1: Inspected workspace and attached image context; created intent-stage ledger and frontier. No implementation or external writes performed.
- Round 2: Owner confirmed Q1-Q5; intent gate accepted; Apple Design and agent/Grok research workstream authorized.
- Round 3: Read Apple HIG pages; created draft `spec.md`; direct Cursor Agent/Grok research attempt produced no output and was stopped.
- Round 4: Owner changed scope to cross-platform and accepted default technology selection; begin Tauri/Rust implementation while preserving Apple-inspired visual guidance.
- Round 5: Cursor GUI research completed with cited guidance; created Tauri scaffold, Rust commands, Apple-inspired frontend, and initial tests.
- Round 6: Replaced naive config/file operations with source-span parsing, stale-preview apply plans, isolated filesystem tests, GitHub/keyring backend, and explicit browser read-only mode.
- Round 7: Added custom checkout inventory, CI matrix, and corrected activity/credential semantics; latest visual refresh remained incomplete due localhost browser policy.
- Round 8: Re-ran final Rust/frontend checks; installed Tauri CLI and recorded the local environment limits.
- Round 9: Built the debug `.app`, launched the exact artifact, read the real `.zshrc` without mutation, verified Configuration preview and Discover inventory, and recorded native visual evidence. DMG bundling failed in the platform bundler; `.app` bundling passed.
- Round 10: Added persisted plugin repository/commit metadata and current checkout SHA display, added backward-compatible history serialization coverage, fixed a frontend title-map runtime regression found by launching the new bundle, rebuilt the `.app`, and verified native Discover details and single enable/disable action.

## Evidence updates

- E1 | state: observed | source: web search/open 2026-09-14 | Existing related work includes ShellCraft, a native macOS GUI that manages `.zshrc` and has an Oh My Zsh section for themes/plugins/settings; its repository is small and MIT-licensed according to its README.
- E2 | state: observed | source: web search/open 2026-09-14 | `omz-plugin-browser` is an existing Pharo GUI focused on listing and enabling/disabling Oh My Zsh plugins.
- E3 | state: observed | source: web search/open 2026-09-14 | Fig Plugin Store is a precedent for one-click terminal plugin discovery/management, but it is not the requested native oh-my-zsh manager.
- E4 | state: observed | source: official oh-my-zsh GitHub/wiki 2026-09-14 | Oh My Zsh configuration centers on `$ZSH` (default `~/.oh-my-zsh`), `.zshrc`, the `plugins=(...)` array, and `ZSH_THEME`; built-in plugins are documented in their plugin README files.
- E5 | state: observed | source: CUA app inventory 2026-09-14 | Cursor is not present in the exposed app inventory; the screenshot alone cannot establish that a live Cursor/Grok session is currently accessible.
- E6 | state: blocked | source: CUA safety boundary 2026-09-14 | Directly inspecting the macOS Terminal app was denied by the computer-use safety boundary, so Cursor CLI/Grok execution was not independently verified.
- E7 | state: observed | source: Apple HIG pages 2026-09-14 | Apple recommends macOS settings windows with stable toolbar panes, sidebars for top-level areas, concise toolbars with menu-bar equivalents, passive status feedback, and alerts reserved for critical/actionable or irreversible cases.
- E8 | state: observed | source: Cursor GUI research 2026-09-14 | Cursor/Grok recommended Tauri 2; conservative top-level `.zshrc` rewriting; `zsh -n` as syntax-only validation; temporary checkout + pinned SHA + disabled hooks for plugins; optional token with rate-limit messaging; and WSL for Windows zsh. Full record: `docs/research/cursor-grok-20260914.md`.
- E9 | state: observed | source: local commands 2026-09-14 | `cargo test` 8/8, `cargo check`, `cargo fmt --check`, and `node --check` pass; `cargo tauri info` reports macOS arm64, Rust, Node, and Command Line Tools, with full Xcode absent.
- E10 | state: observed | source: local build 2026-09-14 | `cargo tauri build --debug --bundles app` passed and produced `/Users/luckye/Documents/ohmyzsh-gui/src-tauri/target/debug/bundle/macos/ohmyzsh-gui.app`; detailed output is in `docs/testing/tauri-build-final.txt`.
- E11 | state: observed | source: CUA native app inspection 2026-09-14 | The exact built app opened as `ohmyzsh-gui` at `tauri://localhost`, loaded the real `/Users/luckye/.zshrc`, showed `powerlevel10k/powerlevel10k` and the user's enabled plugins, produced a no-op Configuration preview, and displayed local custom plugin inventory in Discover. No Apply action was invoked.
- E12 | state: incomplete | source: local commands and scope boundary 2026-09-14 | DMG packaging failed in `bundle_dmg.sh`; live GitHub search, token persistence, install/update/remove, Windows WSL, Linux runtime, and CI execution remain unrun.
- E13 | state: observed | source: local tests and CUA native app inspection 2026-09-14 | Nine Rust tests pass, including legacy history JSON compatibility and repository/commit round-trip coverage. The exact rebuilt app displays the real `zsh-autosuggestions` checkout SHA `85919cd1…` and a single Disable plus Review uninstall action without changing `.zshrc`.
- E14 | state: observed | source: CUA native app inspection 2026-09-14 | The exact rebuilt app's Updates page exposes an explicit `Check for updates` action and Homebrew-style empty/update states. With no app-managed GitHub checkout in the current user inventory, it correctly reports no updates instead of guessing a remote source.
- E15 | state: observed | source: CUA native app + GitHub read 2026-09-14 | A live anonymous search returned GitHub candidates; the exact app generated an install preview for `syi0808/shellsuggest` at commit `6d083fbe`, showed `No .zshrc change`, disabled hooks/submodules in the detail, and exposed separate Apply/Cancel actions. No external checkout or `.zshrc` write was performed.
- E16 | state: observed | source: CUA native app + keyring status 2026-09-14 | Settings visibly reports `No token configured; anonymous GitHub access is active`; only a status string crosses the Tauri boundary, and the token input remains a password field. Token save/remove and platform keyring behavior with a real user token remain unrun.
