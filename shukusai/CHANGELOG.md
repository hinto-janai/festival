# shukusai CHANGELOG
The `cargo` version of `shukusai` is mostly meaningless.

Practically, it acts as a "general" marker for when new things get implemented.

This is mostly for personal reference so I can see when important internal changes get made.

Types of changes:
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities
- `Other` other note-worthy details

---

# shukusai v0.0.5
## Added
- `Song` field: `mime: Arc<str>`, the `MIME` type of this `Song`
- `Song` field: `extension: Arc<str>`, the file extension of this `Song`
- Public `JSON` definitions for various internal structures
- `shukusai::collection::Entry`, similar to `shukusai::state::Entry` for playlists but includes `path: PathBuf`
- Various new `Kernel` calls
- `Search` for `Sim80`, `Sim60`, `Top5`

## Changed
- `Collection v2` -> `Collection v3`

## Other
- `festivald v1.0.0` & `festival-cli v1.0.0` start here, with `Collection v3`, `Audio v0`, `Playlists v0`

---
