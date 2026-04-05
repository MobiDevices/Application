#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <binary-src>" >&2
    exit 1
fi

if ! command -v desktop-file-validate >/dev/null 2>&1; then
    echo "desktop-file-validate is required" >&2
    exit 1
fi

if ! command -v appstreamcli >/dev/null 2>&1; then
    echo "appstreamcli is required" >&2
    exit 1
fi

binary_src=$1
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/mobidevices-linux-assets.XXXXXX")
trap 'rm -rf "$tmp_root"' EXIT INT TERM

"$script_dir/install-linux-assets.sh" \
    "$binary_src" \
    "$tmp_root/usr/bin" \
    mobidevices \
    "$app_root/linux/mobidevices.desktop.in" \
    "$tmp_root/usr/share" \
    com.mobidevices.desktop.desktop \
    mobidevices \
    com.mobidevices.desktop \
    "$app_root/src-tauri/icons" \
    com.mobidevices.desktop \
    "$app_root/linux/com.mobidevices.desktop.metainfo.xml" \
    com.mobidevices.desktop.metainfo.xml

desktop_file="$tmp_root/usr/share/applications/com.mobidevices.desktop.desktop"
metainfo_file="$tmp_root/usr/share/metainfo/com.mobidevices.desktop.metainfo.xml"

desktop-file-validate "$desktop_file"
appstreamcli validate --no-net "$metainfo_file"
