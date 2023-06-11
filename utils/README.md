## Utilities
Some utility scripts.

| File/Folder        | Purpose |
|--------------------|---------|
| Festival.AppDir    | Skeleton `.AppDir` for creating an `.AppImage` for Linux
| skel               | A skeleton directory with the proper naming scheme + folder structure for packaging for all OS's
| mk_appimage.sh     | Create a `Festival.AppImage` from the `Festival.AppDir` 
| mk_tmpenv.sh       | Copy `skel/` to `/tmp` with the packaging scripts
| package.sh         | Package the contents of `skel`, sign, etc. Checks if all files exist and have the proper naming schemes
| prepare.sh         | Changes version across repo, commits README.md + CHANGELOG.md
| `pgp/`             | PGP key used to sign releases

## AUR
The `aur/` directory has `PKGBUILD`'s for the [`AUR`](https://aur.archlinux.org).

| Directory          | Link |
|--------------------|------|
| `festival-gui-bin` | https://aur.archlinux.org/packages/festival-gui-bin
| `festival-web-bin` | https://aur.archlinux.org/packages/festival-web-bin
| `festival-cli-bin` | https://aur.archlinux.org/packages/festival-cli-bin
| `festivald-bin` | https://aur.archlinux.org/packages/festivald-bin
