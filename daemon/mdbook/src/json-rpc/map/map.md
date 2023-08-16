# Map
These methods are the same as the [`key_*`](../key/key.md) methods, but instead of [`key`](/common-objects/key.md)'s, you can directly use:

- [`Artist`](/common-objects/artist.md) name
- [`Album`](/common-objects/album.md) title
- [`Song`](/common-objects/song.md) title

`strings` to lookup [`Common Objects`](/common-objects/common-objects.md).

So instead of:
```bash
festival-cli key_artist --key 100
```
you can use:
```bash
festival-cli map_artist --artist 'カネコアヤノ'
```

If the input is invalid, a JSON-RPC [`error`](/json-rpc/json-rpc.md#example-json-rpc-20-failed-response) will be returned.

### Warning
Inputs are case-sensitive and must be exactly correct.

If you have an `Album` called `Hello World`, none of these inputs will work:
- `Hello world`
- `hello World`
- `HELlo World`
- `HelloWorld`
- `H3ll0 W0rld`

The input must be _exactly_ `Hello World`.

### Collisions
When using [`map_album`](/json-rpc/map/map_album.md) you must specify:
- `Artist` name
- `Album` title

and when using [`map_song`](/json-rpc/map/map_song.md) you must specify:
- `Artist` name
- `Album` title
- `Song` title

The reason these have to be specified is to prevent collisions.

If there's 2 `Song`'s in your [`Collection`](/common-objects/collection.md) called: `Hello World`, `festivald` cannot know which one you want.

Since `Artist` names are unique, and `Album` titles within `Artist`'s are unique, they serve as an identifier.

### Duplicate `Song` Names
An uncommon "bug" you may encounter when using:
- `Artist` names
- `Album` titles
- `Song` titles

as the key for searching objects is that:
1. if multiple `Song`'s with the same name exist
2. in the same `Album`
3. by the same `Artist`

it is impossible to specify which one you want.

This may occur in `Album`'s that have "interlude" tracks, which may be different, but have the same name, e.g:
```bash
Artist Name/
  │
  ├─ Album Name/
      │
      ├─ Song 1
      ├─ Interlude # <------------- same name
      ├─ Song 3.flac
      ├─ Interlude # <------------- same name
      ├─ Song 5.flac
```
The workaround is by using [`Song key`](/common-objects/key.md)'s to retrieve these instead, as they are unique per _file_, regardless of metadata.
