#!/usr/bin/env bash

# prepare new [festival] version in:
# 1. README.md
# 2. CHANGELOG.md
# 3. Cargo.toml

# $1 = new_version
set -ex
sudo -v
[[ $1 = v* ]]
[[ $PWD = */festival/utils ]] && cd ..
[[ $PWD = */festival ]]

# get old FESTIVAL_VER
OLD_VER="v$(grep -m1 "version" Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"
OLD_VER_NUM="$(grep -m1 "version" Cargo.toml | grep -o "[0-9].[0-9].[0-9]")"

# sed change
sed -i "s/$OLD_VER/$1/g" README.md
sed -i "s/^version = \"$OLD_VER_NUM\"/version = \"$1\"/" Cargo.toml

# changelog
cat << EOM > CHANGELOG.md.new
# $1
## Updates
*

## Fixes
*


---


EOM
cat CHANGELOG.md >> CHANGELOG.md.new
mv -f CHANGELOG.md.new CHANGELOG.md

# commit
git add CHANGELOG.md README.md
git commit -m "prepare $1"
