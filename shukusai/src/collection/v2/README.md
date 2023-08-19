# Collection2
This is version 2 of the `Collection`.

This code and data definitions exist here solely for backwards compatibility.

Things added in `v3` that need conversion from `v2`:

- `mime: Arc<str>` in `Song`
- `extension: Arc<str>` in `Song`

`festivald` & `festival-cli` & `rpc/` all also started on `Collection3`.

This means the public JSON API of those things are defined in `Collection3`.
