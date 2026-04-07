#!/bin/sh
set -eu

if ! command -v flatpak-builder >/dev/null 2>&1; then
    echo "flatpak-builder is required" >&2
    exit 1
fi

if ! command -v flatpak >/dev/null 2>&1; then
    echo "flatpak is required" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
flatpak_root="$app_root/flatpak"
temp_root=${RUNNER_TEMP:-${TMPDIR:-/tmp}}
flatpak_bundle_path=${1:-${FLATPAK_BUNDLE_PATH:-}}

if [ -z "$flatpak_bundle_path" ]; then
    flatpak_bundle_path="$app_root/dist/flatpak/mobidevices.flatpak"
fi

state_dir=$(mktemp -d "$temp_root/mobidevices-flatpak-state.XXXXXX")
build_dir=$(mktemp -d "$temp_root/mobidevices-flatpak-build.XXXXXX")
repo_dir=$(mktemp -d "$temp_root/mobidevices-flatpak-repo.XXXXXX")
trap 'rm -rf "$state_dir" "$build_dir" "$repo_dir"' EXIT INT TERM

flatpak_builder_manifest="$flatpak_root/com.mobidevices.desktop.yml"

mkdir -p -- "$(dirname -- "$flatpak_bundle_path")"
rm -f "$flatpak_bundle_path"

flatpak-builder \
    --user \
    --install-deps-from=flathub \
    --state-dir="$state_dir" \
    --repo="$repo_dir" \
    --force-clean \
    "$build_dir" \
    "$flatpak_builder_manifest"

flatpak build-bundle "$repo_dir" "$flatpak_bundle_path" com.mobidevices.desktop

printf '%s\n' "$flatpak_bundle_path"