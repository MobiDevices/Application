# MobiDevices (App)

## Overview

Standalone repository for the cross-platform native MobiDevices application built with Tauri 2.

- Main scope: `.`
- Rust/Tauri code: `src-tauri/`
- Website target: https://mobidevices.com

## Structure

```text
.
├── README.md
├── AGENTS.md
├── LICENSE
├── package.json
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── build.yml
├── macos/
├── src-tauri/
│   ├── src/
│   ├── capabilities/
│   ├── i18n/
│   └── tauri.conf.json
├── linux/
├── debian/
├── flatpak/
├── snap/
└── aur/
```

## Working Rules

- Edit source/config files, not generated build output.
- Avoid editing generated paths unless explicitly requested (`src-tauri/target/`, `src-tauri/gen/`).
- Keep this README high-level and stable.

## Quick Start

```bash
npm install
```

## Common Commands

```bash
npm run dev
npm run build
npm run build:dmg
npm run generate:flatpak-sources
npm run build:flatpak
npm run validate:linux-assets
```

## Scripts

- Packaging helpers live next to their platform or packaging format under `linux/`, `flatpak/`, `snap/`, and `macos/`.
- Linux packaging assets are centralized in `linux/install-assets.sh` and reused by Flatpak, Snap, Debian, AUR, and metadata validation.

## Flathub

- Flatpak manifest now builds the app from source in [flatpak/com.mobidevices.desktop.yml](flatpak/com.mobidevices.desktop.yml) using [flatpak/cargo-sources.json](flatpak/cargo-sources.json).
- Refresh vendored Rust sources after any Cargo dependency change with `npm run generate:flatpak-sources`.
- Local Flatpak build uses `npm run build:flatpak`, writes the final `mobidevices.flatpak` bundle into a temporary directory outside the repository by default, and prints the resulting path.
- Local Flatpak build requires `flatpak-builder`, `org.gnome.Sdk//50`, `org.gnome.Platform//50`, and `org.freedesktop.Sdk.Extension.rust-stable//25.08`.
- Hosted screenshots are configured in [linux/com.mobidevices.desktop.metainfo.xml](linux/com.mobidevices.desktop.metainfo.xml) and should stay available at stable public HTTPS URLs.

## Documentation

- Workflow and constraints: `AGENTS.md`
- CI validation: `.github/workflows/ci.yml`
- Release workflow: `.github/workflows/build.yml`
