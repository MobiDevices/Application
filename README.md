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

## Build Outputs

- Native Tauri bundles are produced under `src-tauri/target/release/bundle/`.
- Linux `.deb` packages in CI and tagged releases are built from the Debian packaging under `debian/` via `dpkg-buildpackage`, then staged under `dist/release/linux/`.
- Local Flatpak builds now write the final bundle to `dist/flatpak/mobidevices.flatpak` by default.
- Local Snap builds now write the final package to `dist/snap/mobidevices.snap` by default.
- Local Flathub submission export now writes to `dist/flathub/<tag>/` by default.
- CI and release workflows stage downloadable artifacts under `dist/release/linux/` before upload, so artifacts no longer mirror the GitHub Actions workspace or `_temp` paths.

## Scripts

- Packaging helpers live next to their platform or packaging format under `linux/`, `flatpak/`, `snap/`, and `macos/`.
- Linux packaging assets are centralized in `linux/install-assets.sh` and reused by Flatpak, Snap, Debian, AUR, and metadata validation.
- Release `.deb` packages use the Debian source package metadata in `debian/`, which keeps the package name at `mobidevices` and aligns desktop/AppStream assets with the rest of the Linux packaging.

## Flathub

- Flatpak manifest now builds the app from source in [flatpak/com.mobidevices.desktop.yml](flatpak/com.mobidevices.desktop.yml) using [flatpak/cargo-sources.json](flatpak/cargo-sources.json).
- Refresh vendored Rust sources after any Cargo dependency change with `npm run generate:flatpak-sources`.
- Local Flatpak build uses `npm run build:flatpak`, writes the final `mobidevices.flatpak` bundle into `dist/flatpak/` by default, and prints the resulting path.
- `npm run prepare:flathub-submission -- [tag] [output-dir]` exports a minimal Flathub submission directory with a top-level `com.mobidevices.desktop.yml` and `cargo-sources.json`, pinned to an upstream Git tag and commit, and defaults to `dist/flathub/<tag>/`.
- The Flathub submission manifest template lives in [flatpak/com.mobidevices.desktop.flathub.yml.in](flatpak/com.mobidevices.desktop.flathub.yml.in) and keeps AppStream compose enabled for Flathub builds.
- Tagged GitHub releases now attach both `mobidevices.flatpak` and `mobidevices-flathub-submission-<tag>.tar.gz`, so the release already contains the final Flatpak bundle plus the Flathub submission files.
- Local Flatpak build requires `flatpak-builder`, `org.gnome.Sdk//50`, `org.gnome.Platform//50`, and `org.freedesktop.Sdk.Extension.rust-stable//25.08`.
- Hosted screenshots are configured in [linux/com.mobidevices.desktop.metainfo.xml](linux/com.mobidevices.desktop.metainfo.xml) and should stay available at stable public HTTPS URLs.

## Documentation

- Workflow and constraints: `AGENTS.md`
- CI validation: `.github/workflows/ci.yml`
- Release workflow: `.github/workflows/build.yml`
