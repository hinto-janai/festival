#!/usr/bin/env bash

set -ex

# Check current directory.
[[ $PWD == */festival ]] && cd utils
[[ $PWD == */festival/utils ]]

if [[ -z $1 ]]; then
	TARGET1="../target/release/festival"
	TARGET2="${HOME}/.cargo/target/release/festival"
	if [[ -e $TARGET1 ]]; then
		BINARY="$TARGET1"
	elif [[ -e $TARGET2 ]]; then
		BINARY="$TARGET2"
	else
		echo "$1 not specified, should be path to festival binary"
		exit 1
	fi
else
	BINARY="$1"
fi

[[ -e "$BINARY" ]] || { echo "$BINARY doesn't exist, should be festival binary"; exit 1; }

# Set variables.
APP_DIR="${PWD}/Festival.AppDir"
VERSION="v$(grep -m1 "version" ../gui/Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"

# Remove old AppImage.
[[ -f "Festival-${VERSION}-x86_64.AppImage" ]] && rm "Festival-${VERSION}-x86_64.AppImage"

# Update icon/binary.
cp -f ../assets/images/icon/512.png "${APP_DIR}/festival.png"
# HACK:
# I cannot figure out how to change the top toolbar
# title from `festival` to `Festival` other than
# changing the underlying binary itself.
#
# This is hacky but should be fine since
# it's an AppImage anyway.
cp -f "$BINARY" "${APP_DIR}/usr/bin/Festival"
chmod +x "${APP_DIR}/usr/bin/Festival"

# Create AppImage.
#if ARCH=x86_64 appimagetool --sign --sign-key "2A8F883E016FED0380287FAFB1C5A64B80691E45" "$APP_DIR"; then
#	mv "Festival-x86_64.AppImage" "Festival-${VERSION}-x86_64.AppImage"
#fi
if ! which appimagetool; then
	wget "https://github.com/AppImage/AppImageKit/releases/download/13/appimagetool-x86_64.AppImage" -O /tmp/appimagetool
	chmod +x /tmp/appimagetool
	APPIMAGETOOL=/tmp/appimagetool
else
	APPIMAGETOOL=appimagetool
fi

ARCH=x86_64 "$APPIMAGETOOL" "$APP_DIR"
chmod +x "Festival-x86_64.AppImage"
mv "Festival-x86_64.AppImage" "Festival-${VERSION}-x86_64.AppImage"

# Wipe icon/binary.
rm "${APP_DIR}/festival.png" "${APP_DIR}/usr/bin/Festival"
touch "${APP_DIR}/festival.png" "${APP_DIR}/usr/bin/Festival"
