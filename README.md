# MobiDevices — App

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
├── scripts/
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
npm run validate:linux-assets
```

## Documentation

- Workflow and constraints: `AGENTS.md`
- CI validation: `.github/workflows/ci.yml`
- Release workflow: `.github/workflows/build.yml`
