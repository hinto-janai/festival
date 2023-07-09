This directory contains test data for testing and (de)serialization purposes, e.g, the `Collection`.

Folders & files are laid out in the same way as the real files, with this folder serving a `Frontend`'s root, e.g `~/.local/share/festival/gui`.

All `shukusai` data structures (e.g `Collection`) are inside here instead of the `../festival`, since there's no reason to test the same structures for every `Frontend`.
