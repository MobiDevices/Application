#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <app-dir>" >&2
  exit 1
fi

app_dir=$1
snap_ci_dir="$app_dir/snap/.ci"

rm -rf "$snap_ci_dir"
mkdir -p "$snap_ci_dir/icons"

install -m 755 "$app_dir/src-tauri/target/release/mobidevices-app" "$snap_ci_dir/mobidevices-app"
install -m 755 "$app_dir/scripts/install-linux-assets.sh" "$snap_ci_dir/install-linux-assets.sh"
install -m 644 "$app_dir/linux/mobidevices.desktop.in" "$snap_ci_dir/mobidevices.desktop.in"
install -m 644 "$app_dir/linux/com.mobidevices.desktop.metainfo.xml" "$snap_ci_dir/com.mobidevices.desktop.metainfo.xml"
cp "$app_dir"/src-tauri/icons/*.png "$snap_ci_dir/icons/"
