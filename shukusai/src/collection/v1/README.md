# Collection1
This is version 1 of the `Collection`.

This code and data definitions exist here solely for backwards compatibility.

Things added in `v2` that need conversion from `v1`:
	- `key: ArtistKey` in `Artist`
	- `key: AlbumKey` in `Album`
	- `key: SongKey` in `Song`
	- `genre: Option<Arc<str>>` in `Album`
	- `mime: Arc<str>` in `Song`
	- `extension: Arc<str>` in `Song`

`festivald` & `festival-cli` & `rpc/` all also started on `Collection2`.

This means the public JSON API of those things are defined in `Collection2`.
