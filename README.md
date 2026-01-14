# MobiDevices — App (Tauri 2)

Cross-platform native app (Tauri 2). Essentially a WebView wrapper around https://mobidevices.com.

## Commands

```bash
cd app

npm install
npm run dev

# Build only the .app bundle (faster, no DMG)
npm run build

# Build DMG (macOS)
npm run build:dmg
```

## External links (release builds)

If external links work in `npm run dev` but fail in `npm run build`, the reason is usually not allowlist/permissions.

Typical root cause: the website intercepts clicks in JavaScript (`preventDefault`, custom router, etc.), so WebView events (`on_navigation` / `on_new_window`) never receive the external URL — and Rust handlers never run.

Fix used in this repo:
- Inject a click-capture interceptor using `WebviewWindowBuilder::initialization_script(...)`.
- For external URLs, force `window.open(url, '_blank')`.
- In Rust, handle `on_new_window` and open through `tauri_plugin_shell` (with fallback).

Diagnostics:
- Logging is disabled by default in release.
- To enable temporarily: run with `MOBIDEVICES_LOG_EXTERNAL=1`. Log file: `/tmp/mobidevices-external-links.log`.

## DMG: "Resource busy" during build

Sometimes `bundle_dmg.sh` fails with `hdiutil: couldn't unmount ... - Resource busy`.

Mitigation used in this repo:
- `app/scripts/clean-dmg-mounts.sh` detaches stale `/Volumes/dmg.*` mounts before bundling.
- `npm run build:dmg` calls this script automatically.

## Linux distribution (Flatpak / Snap / AUR)

Packaging manifests live in the repo:
- `app/flatpak/` (Flatpak)
- `app/snap/` (Snap)
- `app/aur/` (AUR)

### Flathub (outline)

1. Fork https://github.com/flathub/flathub
2. Copy `app/flatpak/com.mobidevices.desktop.yml` into a new app repo
3. Update the SHA256 to your release archive
4. Open a PR

### Snap Store (outline)

```bash
cd app/snap
snapcraft

snapcraft login
snapcraft upload mobidevices_1.0.0_amd64.snap
snapcraft release mobidevices 1.0.0 stable
```

### AUR (outline)

```bash
git clone ssh://aur@aur.archlinux.org/mobidevices.git

cp app/aur/PKGBUILD mobidevices/
cp app/aur/mobidevices.desktop mobidevices/

cd mobidevices
updpkgsums
makepkg --printsrcinfo > .SRCINFO

git add PKGBUILD .SRCINFO mobidevices.desktop
git commit -m "Update"
git push
```
