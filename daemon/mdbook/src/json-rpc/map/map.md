# Map
These methods are the same as the [`key_*`](../key/key.md) methods, but instead of [`key`](../../common-objects/key.md)'s, you can directly use:

- [`Artist`](../../common-objects/artist.md) name
- [`Album`](../../common-objects/album.md) title
- [`Song`](../../common-objects/song.md) title

`strings` to lookup [`Common Objects`](../../common-objects/common-objects.md).

So instead of:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_artist","params":{"key":100}}'
```
you can use:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_artist","params":{"artist":"カネコアヤノ"}}'
```

If the input is invalid, a JSON-RPC [`error`](../json-rpc.md#example-json-rpc-20-failed-response) will be returned.

## Warning
Inputs are case-sensitive and must be exactly correct.

If you have an `Album` called `Hello World`, none of these inputs will work:
- `Hello world`
- `hello World`
- `HELlo World`
- `HelloWorld`
- `H3ll0 W0rld`

The input must be _exactly_ `Hello World`.

## Collisions
When using `map_album` you must specify:
- `Artist` name
- `Album` title

and when using `map_song` you must specify:
- `Artist` name
- `Album` title
- `Song` title

The reason these have to be specified is to prevent collisions.

If there's 2 songs in your `Collection` called: `Hello World`, which one should `festivald` return?

Since `Artist` names are unique, and `Album` titles within `Artist`'s are unique, they serve as an identifier.
