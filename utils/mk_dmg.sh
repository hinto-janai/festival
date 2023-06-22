#!/usr/bin/env bash

# Make a `Festival-${VERSION}.dmg` out of the `Festival.app` for macOS.
#
# This assumes you're running on macOS.

set -ex

# Check current directory.
[[ $PWD == */festival ]] && cd utils
[[ $PWD == */festival/utils ]]

# Set variables.
VERSION="$(grep -m1 "version" ../gui/Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"
APP="${PWD}/Festival.app"

[[ $1 ]] && DMG="$1" || DMG="Festival-${VERSION}.dmg"

# Remove old `.dmg`
[[ -e $DMG ]] && rm -r "$DMG"

# Create `.dmg`
hdiutil create -fs HFS+ -srcfolder Festival.app -volname "Festival-${VERSION}" "$DMG"

# Wipe binary.
rm "${APP}/Contents/MacOS/festival"
touch "${APP}/Contents/MacOS/festival"
