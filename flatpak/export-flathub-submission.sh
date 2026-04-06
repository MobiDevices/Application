#!/bin/sh
set -eu

if ! command -v git >/dev/null 2>&1; then
    echo "git is required" >&2
    exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
template_path="$script_dir/com.mobidevices.desktop.flathub.yml.in"
cargo_sources_path="$script_dir/cargo-sources.json"

if [ ! -f "$template_path" ]; then
    echo "Flathub manifest template is missing: $template_path" >&2
    exit 1
fi

if [ ! -f "$cargo_sources_path" ]; then
    echo "Flatpak cargo sources file is missing: $cargo_sources_path" >&2
    exit 1
fi

tag=${1:-}
out_dir=${2:-}

if [ -z "$tag" ]; then
    tag=$(git -C "$app_root" describe --tags --abbrev=0)
fi

if ! git -C "$app_root" rev-parse -q --verify "refs/tags/$tag" >/dev/null 2>&1; then
    echo "git tag not found: $tag" >&2
    exit 1
fi

commit=$(git -C "$app_root" rev-list -n 1 "$tag")

if [ -z "$out_dir" ]; then
    out_dir=$(mktemp -d "${TMPDIR:-/tmp}/mobidevices-flathub-submission.XXXXXX")
fi

mkdir -p "$out_dir"
cp "$cargo_sources_path" "$out_dir/cargo-sources.json"

sed \
    -e "s|@UPSTREAM_TAG@|$tag|g" \
    -e "s|@UPSTREAM_COMMIT@|$commit|g" \
    "$template_path" > "$out_dir/com.mobidevices.desktop.yml"

printf '%s\n' "$out_dir"
