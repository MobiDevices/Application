# MobiDevices — App

## Overview

Cross-platform native application built with Tauri 2 for MobiDevices.

- Main app scope: `app/`
- Rust/Tauri code: `app/src-tauri/`
- Website target: https://mobidevices.com

## Structure

```text
app/
├── README.md
├── AGENTS.md
├── package.json
├── scripts/
├── src-tauri/
│   ├── src/
│   ├── capabilities/
│   ├── i18n/
│   └── tauri.conf.json
├── flatpak/
├── snap/
└── aur/
```

## Working Rules

- Edit source/config files, not generated build output.
- Avoid editing generated paths unless explicitly requested (`app/src-tauri/target/`, `app/src-tauri/gen/`).
- Keep this README high-level and stable (avoid implementation-level details).

## Quick Start

```bash
cd app
npm install
```

## Common Commands

```bash
cd app
npm run dev
npm run build
npm run build:dmg
```

## Documentation

- App workflow and constraints: `app/AGENTS.md`
- Root conventions: `AGENTS.md`
- If app architecture changes (structure, runtime responsibilities, major integration boundaries), update this README in the same change.
