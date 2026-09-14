# ohmyzsh-gui

A personal cross-platform desktop MVP for inspecting and safely managing `.zshrc` and Oh My Zsh plugins.

## Technology

The app uses Tauri 2 with a Rust core and a lightweight HTML/CSS/JavaScript frontend. Rust owns file access, syntax validation, backups, Git operations, and token storage; the frontend provides an Apple-inspired sidebar, list/detail marketplace, update review, badges, toolbar, keyboard-friendly controls, and light/dark system appearance.

The layout follows the Apple Human Interface Guidelines for [sidebars](https://developer.apple.com/design/human-interface-guidelines/sidebars), [settings](https://developer.apple.com/design/human-interface-guidelines/settings), [toolbars](https://developer.apple.com/design/human-interface-guidelines/toolbars), and [feedback](https://developer.apple.com/design/human-interface-guidelines/feedback), with the supplied Homebrew window as a practical reference.

## Run

Install the Tauri CLI, then run `cargo tauri dev` from the repository root. For a browser-only visual preview, run `npm run dev` and open `http://localhost:1420`; preview mode uses a safe in-memory `.zshrc` fixture and explicitly refuses Apply, token storage, and GitHub requests.

## Safety boundary

The app never changes `.zshrc` without an explicit Apply action. It creates a backup, writes a temporary file, validates with `zsh -n`, and replaces the original only after validation succeeds. GitHub plugin sources are restricted to owner/repository components; install/update/uninstall require confirmation. Installed checkouts record their repository and exact commit SHA, and the current SHA is shown when the checkout is rediscovered. GitHub Token is optional and stored through the platform credential store (Keychain on macOS, Credential Manager on Windows, Secret Service on Linux); anonymous search remains available when that store is unavailable. Plugin code is never executed by the manager.

## Validation

`node --check src/app.js` and `cargo test --manifest-path src-tauri/Cargo.toml` validate the frontend syntax and Rust parsing/safety core. `cargo check --manifest-path src-tauri/Cargo.toml` validates the Tauri backend. The current Rust suite has 9 tests; `cargo tauri build --debug --bundles app` produces the local macOS `.app` bundle. Full signing and DMG distribution require additional platform SDKs and signing setup.
