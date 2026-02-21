# App (Tauri 2 Native Application)

## Overview

Cross-platform native application for https://mobidevices.com built with Tauri 2.

It is a WebView-based app with additional native behavior for:

- external link handling
- localized native menu/navigation actions
- dynamic window title sync
- persisted desktop window state

## Scope and Precedence

- This file defines app-specific rules and overrides root defaults for app tasks.
- In conflicts: user request -> this file -> root `AGENTS.md`.

## Path Policy

Allowed by default:

- `app/src-tauri/src/`
- `app/src-tauri/capabilities/`
- `app/src-tauri/i18n/`
- `app/src-tauri/tauri.conf.json`
- `app/src-tauri/Cargo.toml`
- `app/src-tauri/build.rs`
- `app/scripts/`
- `app/package.json`
- `app/flatpak/`, `app/snap/`, `app/aur/` (when packaging work is requested)

Do not edit unless explicitly requested:

- `app/node_modules/`
- `app/src-tauri/target/`
- `app/src-tauri/gen/` (generated platform project files)
- Binary artifacts (`*.apk`, `*.idsig`, `*.dmg`, etc.)

## Code Style

- For JS/TS files: single quotes
- For JS/TS files: no semicolons

## Project Structure

```text
app/
├── package.json
├── mobidevices.keystore
├── scripts/
│   └── clean-dmg-mounts.sh
├── flatpak/
├── snap/
├── aur/
└── src-tauri/
    ├── Cargo.toml
    ├── rust-toolchain.toml
    ├── tauri.conf.json
    ├── build.rs
    ├── capabilities/
    │   └── default.json
    ├── i18n/
    │   ├── en/translate.yml
    │   └── ru/translate.yml
    ├── icons/
    └── src/
        ├── main.rs
        ├── lib.rs
        ├── app.rs
        ├── features/
        └── shared/
```

## Requirements

- Rust toolchain pinned in `app/src-tauri/rust-toolchain.toml` (currently `1.93.1`)
- Node.js 18+
- For Android: Java 17, Android SDK, NDK

Typical shell setup:

```bash
source "$HOME/.cargo/env"
export ANDROID_HOME="/opt/homebrew/share/android-commandlinetools"
export NDK_HOME="$ANDROID_HOME/ndk/27.0.12077973"
export JAVA_HOME="/Library/Java/JavaVirtualMachines/temurin-17.jdk/Contents/Home"
```

## Development Commands

```bash
cd app
npm install
npm run dev
npm run build
npm run build:dmg
```

Script mapping from `package.json`:

- `npm run dev` -> `tauri dev`
- `npm run build` -> `tauri build --bundles app`
- `npm run build:dmg` -> cleanup script + `tauri build --bundles dmg`

## Build Outputs

Common outputs:

- macOS app bundle: `app/src-tauri/target/release/bundle/macos/`
- macOS DMG: `app/src-tauri/target/release/bundle/dmg/`
- Generic Tauri bundle output root: `app/src-tauri/target/release/bundle/`

Android build flow:

```bash
npx tauri android init
npx tauri android build --apk true
```

Unsigned APK is produced under:

- `app/src-tauri/gen/android/app/build/outputs/apk/universal/release/`

Signing is a separate manual step (keystore + zipalign + apksigner).

## Runtime Architecture Notes

Main window setup is in `app/src-tauri/src/app.rs`:

- app creates `main` WebView window manually
- injects JS for external-link intercept and title sync
- handles `on_new_window` and `on_navigation` for external URL routing

Native modules:

- `app/src-tauri/src/features/`: feature modules for navigation/window/UI integration
- `app/src-tauri/src/shared/`: shared cross-cutting runtime modules
- `tauri-plugin-window-state`: persists desktop window state

## Configuration

Prefer referencing real config files:

- `app/src-tauri/tauri.conf.json`
- `app/src-tauri/capabilities/default.json`
- `app/src-tauri/Cargo.toml`

## External Links (Release Builds)

Observed behavior: links can work in `npm run dev` but fail in release bundle if website JS intercepts clicks before Tauri hooks.

Current fix in this repo:

1. Inject click-capture script in WebView.
2. Route external links through `window.open(..., "_blank")`.
3. Handle in Rust (`on_new_window` / `on_navigation`) and open via shell.

Diagnostics:

- Logging is disabled in release by default.
- Enable temporary logs with `MOBIDEVICES_LOG_EXTERNAL=1`.
- Log file: `/tmp/mobidevices-external-links.log` (macOS).

## DMG Bundling Flakiness

If DMG build fails with `Resource busy`, cleanup stale mounts:

- script: `app/scripts/clean-dmg-mounts.sh`
- already wired into `npm run build:dmg`

## Linux Distribution Manifests

- `app/flatpak/`
- `app/snap/`
- `app/aur/`

## Android Keystore

`app/mobidevices.keystore` is critical for Play Store updates.

- Keep it safe.
- Do not commit secrets/passwords.

## Definition of Done

For Rust/Tauri source or config changes:

1. Run `cd app && cargo check --manifest-path src-tauri/Cargo.toml`
2. Run `cd app && npm run build` when changes can affect runtime/bundling
3. Update `app/README.md` if app architecture changed
4. Report check results and any skipped/failed validation

For docs-only or manifest-only changes that do not affect runtime behavior, build may be skipped with a short reason.

## Reporting and Fallback

- For final response structure, follow root `AGENTS.md` -> `Final Report Format`.
- For environment/dependency failures, follow root `AGENTS.md` -> `Fallback Policy (Missing Environment/Deps)`.
