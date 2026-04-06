# MobiDevices (Tauri 2 Native Application)

## Overview

Cross-platform native application for https://mobidevices.com built with Tauri 2.

It is a WebView-based app with additional native behavior for:

- external link handling
- localized native menu/navigation actions
- dynamic window title sync
- persisted desktop window state

## Path Policy

Allowed by default:

- `src-tauri/src/`
- `src-tauri/capabilities/`
- `src-tauri/i18n/`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/build.rs`
- `package.json`
- `flatpak/`, `snap/`, `aur/`, `linux/`, `debian/`, `macos/`

Do not edit unless explicitly requested:

- `node_modules/`
- `src-tauri/target/`
- `src-tauri/gen/`
- Binary artifacts such as `*.apk`, `*.idsig`, `*.dmg`

## Code Style

- Use 4 spaces for indentation when writing or editing files
- For JS/TS files: single quotes
- For JS/TS files: no semicolons

## Requirements

- Rust toolchain pinned in `src-tauri/rust-toolchain.toml`
- Node.js 18+
- For Android: Java 17, Android SDK, NDK

## Development Commands

```bash
npm install
npm run dev
npm run build
npm run build:dmg
```

## Build Outputs

Common outputs:

- macOS app bundle: `src-tauri/target/release/bundle/macos/`
- macOS DMG: `src-tauri/target/release/bundle/dmg/`
- Generic Tauri bundle output root: `src-tauri/target/release/bundle/`

Unsigned Android APK output:

- `src-tauri/gen/android/app/build/outputs/apk/universal/release/`

## Runtime Architecture Notes

Main window setup is in `src-tauri/src/app.rs`:

- app creates `main` WebView window manually
- injects JS for external-link intercept and title sync
- handles `on_new_window` and `on_navigation` for external URL routing

Native modules:

- `src-tauri/src/features/` for feature modules
- `src-tauri/src/shared/` for shared cross-cutting runtime modules
- `tauri-plugin-window-state` persists desktop window state

## External Links

Observed behavior: links can work in `npm run dev` but fail in release bundle if website JS intercepts clicks before Tauri hooks.

Current fix in this repo:

1. Inject click-capture script in WebView
2. Route external links through `window.open(..., "_blank")`
3. Handle in Rust (`on_new_window` / `on_navigation`) and open via shell

## CI/CD

- CI workflow: `.github/workflows/ci.yml`
- Release workflow: `.github/workflows/build.yml`

## Definition of Done

For Rust/Tauri source or config changes:

1. Run `cargo check --manifest-path src-tauri/Cargo.toml`
2. Run `npm run build` when changes can affect runtime or bundling
3. Update `README.md` if app architecture changed
4. Report check results and any skipped validation

For docs-only or packaging-only changes that do not affect runtime behavior, build may be skipped with a short reason.

## Final Report Format

For substantial tasks, the final message should include:

1. Files changed
2. Commands/checks run and result
3. Risks, assumptions, or anything not validated
4. Next action, only if a natural next step exists

## Fallback Policy

If a required command fails because of missing environment, dependency, or tooling:

1. Stop the failing flow and keep existing changes intact
2. Report the exact failing command and short error reason
3. Continue with the best safe alternative checks available locally
4. Ask for user guidance only when blocked from meaningful validation
