#!/usr/bin/env bash
set -euo pipefail

# Detach any leftover Tauri/createdmg mounts like /Volumes/dmg.*
# This prevents "Resource busy" errors during subsequent DMG builds.

devices="$(
    hdiutil info 2>/dev/null \
        | awk 'BEGIN{dev=""} /^\/dev\//{dev=$1} /\/Volumes\/dmg\./{if(dev!=""){print dev; dev=""}}' \
        | sort -u
)"

if [[ -z "$devices" ]]; then
    echo "No stale dmg.* volumes found"
    exit 0
fi

echo "Detaching stale dmg volumes:"
echo "$devices"

while IFS= read -r dev; do
    [[ -z "$dev" ]] && continue
    hdiutil detach -force "$dev" >/dev/null 2>&1 || true
done <<< "$devices"

exit 0