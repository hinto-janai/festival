# Common Objects
These often appear in many JSON-RPC call outputs.

The definitions of these object's key/value pairs will be here, instead of everywhere they appear in the documentation.

## Collection
The [`Collection`](/common-objects/collection.md) is the main music library/database in `festivald`.

## The 3 Common Objects
The 3 common "objects" that have a set structure are:
- [`Artist`](/common-objects/artist.md)
- [`Album`](/common-objects/album.md)
- [`Song`](/common-objects/song.md)

## Keys
And the number [`key`](/common-objects/key.md) that acts as the identifier for them:
- `Artist` key
- `Album` key
- `Song` key

## Entry
[`Entry`](/common-objects/entry.md) is an "absolute" key, as it holds all integer `Key`'s and all `string` key's relating to a [`Song`](song.md).

## Playlist
[`Playlist`](/common-objects/playlist.md)'s are regular lists of `Song`'s.

The "entries" within a `Playlist` aren't `Song` objects themselves, but `Entry`-like objects called [`Playlist Entry`](/common-objects/playlist.md)'s.

## Generic Response
This is the generic "success" response object, just an empty `result`:
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0         // the `result` field existing in JSON-RPC means success,
}                 // so the value in this case (null) doesn't matter.
```
This is the response to many JSON-RPC as well, typically the [`Playback Control`](../json-rpc/playback/playback.md) methods.

If an error occurred in this situation, the JSON-RPC defined `error` would be used:
```json
{
  "jsonrpc": "2.0",
  "error": {          // <--- something went wrong.
    "code": -1,       // <--- the error code.
    "message": "...", // <--- short message of the error.
  },
  "id": 0
}
```
