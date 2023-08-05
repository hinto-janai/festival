# Common Objects
These often appear in many JSON-RPC call outputs.

The definitions of these object's key/value pairs will be here, instead of everywhere they appear in the documentation.

## Collection
The [`Collection`](collection.md) is the main music library/database in `festivald`.

## The 3 Common Objects
The 3 common "objects" that have a set structure are:
- [`Artist`](artist.md)
- [`Album`](album.md)
- [`Song`](song.md)

## Keys
And the number [`key`](key.md) that acts as the identifier for them:
- `Artist` key
- `Album` key
- `Song` key

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
