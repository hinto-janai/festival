#!/usr/bin/env bash

set -ex

# Check current directory.
[[ $PWD == */festival ]] && cd utils
[[ $PWD == */festival/utils ]]

# Set variables.
APP_DIR="${PWD}/Festival.AppDir"
VERSION="v$(grep -m1 "version" ../Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"

# Remove old AppImage.
[[ -f "Festival-x86_64.AppImage" ]] && rm "Festival-x86_64.AppImage"

# Update icon/binary.
cp -f ../assets/images/icon/256.png "${APP_DIR}/256.png"
cp -f ~/.cargo/target/release/festival "${APP_DIR}/usr/bin/festival"

# Create AppImage.
if ARCH=x86_64 appimagetool --sign --sign-key "2A8F883E016FED0380287FAFB1C5A64B80691E45" "$APP_DIR"; then
	mv "Festival-x86_64.AppImage" "Festival-${VERSION}-x86_64.AppImage"
fi

# Wipe icon/binary.
rm "${APP_DIR}/256.png" "${APP_DIR}/usr/bin/festival"
