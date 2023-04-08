#!/usr/bin/env bash

# Finds the longest PATH and file with
# the most amount of lines in this repo.
# This is used for left-padding the filename
# in the `src/logger.rs` file.

# Exit on failure.
set -e

# `cd` to root.
[[ $PWD == */festival/utils ]] && cd ..
[[ $PWD == */festival ]]

# Use `fd` if found.
if [[ -f /usr/bin/fd ]]; then
	FIND=$(fd .*.rs "festival"* "src")
else
	FIND=$(find "festival"* src -type f -iname *.rs)
fi

# PATH.
echo "Longest PATH"
echo "$FIND" | awk '{ print length(), $0 | "sort -n" }' | tail -n 1

# Lines.
echo
echo "Most lines"
wc -l $FIND | sort -h | tail -n 2 | head -n 1
