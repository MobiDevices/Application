#!/bin/sh
set -eu

if ! command -v snapcraft >/dev/null 2>&1; then
    echo "snapcraft is required" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
output_path=${1:-}
snap_project_dir="$app_root/snap"

sh "$script_dir/prepare-build-inputs.sh" "$app_root"
find "$snap_project_dir" -maxdepth 1 -type f -name '*.snap' -delete

(
    cd "$snap_project_dir"
    snapcraft --destructive-mode
)

built_snap=$(find "$snap_project_dir" -maxdepth 1 -type f -name '*.snap' | sort | head -n 1)

if [ -z "$built_snap" ]; then
    echo "snap build did not produce a .snap file" >&2
    exit 1
fi

if [ -n "$output_path" ]; then
    mkdir -p -- "$(dirname -- "$output_path")"
    rm -f -- "$output_path"
    cp "$built_snap" "$output_path"
    built_snap="$output_path"
fi

printf '%s\n' "$built_snap"
