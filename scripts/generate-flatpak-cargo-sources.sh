#!/bin/sh
set -eu

if ! command -v curl >/dev/null 2>&1; then
    echo "curl is required" >&2
    exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
    echo "python3 is required" >&2
    exit 1
fi

if ! python3 -c 'import aiohttp, tomlkit, yaml' >/dev/null 2>&1; then
    echo "python modules aiohttp, tomlkit, and PyYAML are required" >&2
    echo "install them with: python3 -m pip install --user aiohttp tomlkit PyYAML" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
generator_url=https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
tmp_script=$(mktemp "${TMPDIR:-/tmp}/flatpak-cargo-generator.XXXXXX.py")
trap 'rm -f "$tmp_script"' EXIT INT TERM

curl -fsSL "$generator_url" -o "$tmp_script"
python3 "$tmp_script" "$app_root/src-tauri/Cargo.lock" -o "$app_root/flatpak/cargo-sources.json"
