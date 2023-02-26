## Utilities

| File/Folder        | Purpose |
|--------------------|---------|
| Festival.AppDir    | Skeleton `.AppDir` for creating an `.AppImage` for Linux
| skel               | A skeleton directory with the proper naming scheme + folder structure for packaging for all OS's
| create_appimage.sh | Create a `Festival.AppImage` from the `Festival.AppDir` 
| create_tmp_env.sh  | Copy `skel/` to `/tmp` with the packaging scripts
| package.sh         | Package the contents of `skel`, sign, etc. Checks if all files exist and have the proper naming schemes
| prepare.sh         | Changes version across repo, commits README.md + CHANGELOG.md
