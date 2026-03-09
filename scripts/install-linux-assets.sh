#!/bin/sh
set -eu

if [ "$#" -ne 10 ]; then
  echo "usage: $0 <binary-src> <binary-root> <binary-name> <desktop-template> <share-root> <desktop-name> <desktop-exec> <desktop-icon> <icon-src-dir> <icon-name>" >&2
  exit 1
fi

binary_src=$1
binary_root=$2
binary_name=$3
desktop_template=$4
share_root=$5
desktop_name=$6
desktop_exec=$7
desktop_icon=$8
icon_src_dir=$9
icon_name=${10}

install -Dm755 "$binary_src" "$binary_root/$binary_name"
install -Dm644 /dev/null "$share_root/applications/$desktop_name"
sed \
  -e "s|@EXEC@|$desktop_exec|g" \
  -e "s|@ICON@|$desktop_icon|g" \
  "$desktop_template" > "$share_root/applications/$desktop_name"

for size in 32 128; do
  install -Dm644 \
    "$icon_src_dir/${size}x${size}.png" \
    "$share_root/icons/hicolor/${size}x${size}/apps/${icon_name}.png"
done
