#!/usr/bin/env bash

# Make `Festival.app` for macOS.

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
APP="${PWD}/Festival.app"

# Remove old `.app`
[[ -d $APP ]] && rm -r "$APP"

# Create new `.app`.
cp -r "Festival.app.skel" "$APP"

# Set `Info.plist` variables.
FESTIVAL_NAME="Festival"
FESTIVAL_IDENT="pm.festival.Festival"
FESTIVAL_VERSION="$(grep -m1 "version" ../gui/Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"

cat << EOM > "${APP}/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>English</string>
  <key>CFBundleDisplayName</key>
  <string>${FESTIVAL_NAME}</string>
  <key>CFBundleExecutable</key>
  <string>${FESTIVAL_NAME}</string>
  <key>CFBundleIconFile</key>
  <string>${FESTIVAL_NAME}.icns</string>
  <key>CFBundleIdentifier</key>
  <string>${FESTIVAL_IDENT}</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleDisplayName</key>
  <string>${FESTIVAL_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${FESTIVAL_VERSION}</string>
  <key>CFBundleVersion</key>
  <string>${FESTIVAL_VERSION}</string>
  <key>CSResourcesFileMapped</key>
  <true/>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.music</string>
  <key>LSRequiresCarbon</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOM

rm "${APP}/Contents/MacOS/festival"
cp -f "${BINARY}" "${APP}/Contents/MacOS/"

echo "${APP}"
