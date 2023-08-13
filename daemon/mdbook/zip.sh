#!/usr/bin/env bash

# Build the documentation, and zip it.

set -ex

[[ -e docs.zip ]] && rm docs.zip

mdbook build && zip -r9 docs.zip docs
