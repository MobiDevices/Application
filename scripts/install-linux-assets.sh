#!/bin/sh
set -eu

if [ "$#" -ne 10 ] && [ "$#" -ne 12 ]; then
    echo "usage: $0 <binary-src> <binary-root> <binary-name> <desktop-template> <share-root> <desktop-name> <desktop-exec> <desktop-icon> <icon-src-dir> <icon-name> [<metainfo-src> <metainfo-name>]" >&2
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
metainfo_src=
metainfo_name=

if [ "$#" -eq 12 ]; then
    metainfo_src=${11}
    metainfo_name=${12}
fi

install_file() {
    src=$1
    dest=$2
    mode=$3

    mkdir -p "$(dirname "$dest")"
    install -m "$mode" "$src" "$dest"
}

mkdir -p "$binary_root" "$share_root/applications"
install -m 755 "$binary_src" "$binary_root/$binary_name"
sed \
    -e "s|@EXEC@|$desktop_exec|g" \
    -e "s|@ICON@|$desktop_icon|g" \
    "$desktop_template" > "$share_root/applications/$desktop_name"
chmod 644 "$share_root/applications/$desktop_name"

if [ -n "$metainfo_src" ] && [ -n "$metainfo_name" ]; then
    install_file "$metainfo_src" "$share_root/metainfo/$metainfo_name" 644
fi

for icon_path in "$icon_src_dir"/*.png; do
    icon_file=$(basename "$icon_path")

    case "$icon_file" in
        icon.png)
            install_file \
                "$icon_path" \
                "$share_root/icons/hicolor/512x512/apps/${icon_name}.png" \
                644
            ;;
        *@2x.png)
            base_name=${icon_file%@2x.png}
            case "$base_name" in
                [0-9]*x[0-9]*)
                    width=${base_name%x*}
                    height=${base_name#*x}
                    width=$((width * 2))
                    height=$((height * 2))
                    install_file \
                        "$icon_path" \
                        "$share_root/icons/hicolor/${width}x${height}/apps/${icon_name}.png" \
                        644
                    ;;
            esac
            ;;
        [0-9]*x[0-9]*.png)
            size=${icon_file%.png}
            install_file \
                "$icon_path" \
                "$share_root/icons/hicolor/${size}/apps/${icon_name}.png" \
                644
            ;;
    esac
done
