## Utilities
Some utility scripts.

| File/Folder        | Purpose |
|--------------------|---------|
| Festival.AppDir    | Skeleton `.AppDir` for creating an `.AppImage` for Linux
| longest.sh         | Find the longest PATH and line count file in the repo
| skel               | A skeleton directory with the proper naming scheme + folder structure for packaging for all OS's
| mk_appimage.sh     | Create a `Festival.AppImage` from the `Festival.AppDir` 
| mk_app.sh          | Create a macOS `Festival.app` 
| mk_dmg.sh          | Create a macOS `.dmg` from the above `Festival.app`
| mk_tmpenv.sh       | Copy `skel/` to `/tmp` with the packaging scripts
| package.sh         | Package the contents of `skel`, sign, etc. Checks if all files exist and have the proper naming schemes
| prepare.sh         | Changes version across repo, commits README.md + CHANGELOG.md
