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

[[ $1 ]] && OUTPUT="$1" || OUTPUT="Festival-${VERSION}.dmg"

# Remove old `.dmg`
[[ -e $OUTPUT ]] && rm -r "$OUTPUT"

# Create `.dmg`
hdiutil create -fs HFS+ -srcfolder Festival.app -volname "Festival-${VERSION}" "$OUTPUT"

echo "${OUTPUT}"
