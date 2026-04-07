#!/bin/sh
set -eu

if ! command -v snapcraft >/dev/null 2>&1; then
    echo "snapcraft is required" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
output_path=${1:-${SNAP_OUTPUT_PATH:-}}
snap_project_dir="$app_root/snap"
build_provider=${SNAPCRAFT_BUILD_PROVIDER:-destructive-mode}

if [ -z "$output_path" ]; then
    output_path="$app_root/dist/snap/mobidevices.snap"
fi

sh "$script_dir/prepare-build-inputs.sh" "$app_root"
find "$snap_project_dir" -maxdepth 1 -type f -name '*.snap' -delete

case "$build_provider" in
    destructive-mode)
        snapcraft_command="snapcraft --destructive-mode"
        ;;
    lxd)
        export SNAPCRAFT_BUILD_ENVIRONMENT=lxd
        snapcraft_command="snapcraft"
        ;;
    *)
        echo "unsupported snapcraft build provider: $build_provider" >&2
        exit 1
        ;;
esac

(
    cd "$snap_project_dir"
    sh -c "$snapcraft_command"
)

built_snap=$(find "$snap_project_dir" -maxdepth 1 -type f -name '*.snap' | sort | head -n 1)

if [ -z "$built_snap" ]; then
    echo "snap build did not produce a .snap file" >&2
    exit 1
fi

mkdir -p -- "$(dirname -- "$output_path")"
rm -f -- "$output_path"
mv "$built_snap" "$output_path"
built_snap="$output_path"

printf '%s\n' "$built_snap"
