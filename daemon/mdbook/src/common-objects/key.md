# Key

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

[`Common Objects`](common-objects.md) all have an unsigned integer associated with them:

- [`Artist`](artist.md) key
- [`Album`](album.md) key 
- [`Song`](song.md) key

These keys directly map to a given object, and can be used to retrieve them.

In the case of `Album` art, the `Album` key doubles as the key (since art belongs to an album), so to access the art of `Album` 123 in the `REST` API, you would use:
```http
http://localhost:18425/key/art/123
```

`Key`'s start at `0` and are unique _per_ object group, meaning there is an `Artist` 0 key AND `Album` 0 key AND `Song` 0 key.

The actual number value of `Key`'s aren't significant, and should be treated as random.

Thus, the `songs` field in [`Album`](/common-objects/album.md) won't necessarily be incrementing by 1, e.g, `[0, 1, 2, ...]`. It could be `[0, 6, 112, 3, ...]`. Same with `Album` keys within `Artist`'s.

## Where
`Key`'s can be found in multiple `JSON-RPC` methods, such as [`map_artist`](../json-rpc/map/map_artist.md), [`search_album`](../json-rpc/search/search_album.md), etc.

Each object contains its own key within itself as well, and there are links in-between them:
```plaintext
                 Artist_0                       Artist_1
              ______|______                  ______|_________________
             /             \                /             \          \
          Album_13       Album_5         Album_0         Album_4    Album_8
          __|__           __|__           __|__          __|__         \___        
         /     \         /     \         /     \        /     \            \
[Song_5, Song_0, ...] [Song_3, ...]  [Song_76, ...] [Song_45, ...]   [Song_41, ...]
```
1. An `Artist` contains keys leading to `Album`'s
2. `Album`'s have a key pointing back to the owning `Artist`, and keys leading to `Song`'s
3. `Song`'s have a key pointing back to the owning `Album`

For convenience, all `Artist`'s also have an array of all their `Song`'s.

## Why
`Key`'s are a number that represent direct access to a unique `Artist`/`Album`/`Song`.

Think of it as an index into an array (that's what they really are, implemented).

Reasons why key's exist:
- Accessing objects via a key is faster than with `string` inputs
- Storing/sending/parsing integers is faster & cheaper than `string`'s
- As long as your `Collection` is stable, the key's are stable
- Accessing `Song`'s with the _same_ title in the same `Album` by the same `Artist` is impossible with `string`'s, however, each one of those `Song`'s will have a unique `Key`, which it makes it possible to access them that way

## Why NOT
`Key`'s can only be relied upon as long as the [`Collection`](/common-objects/collection.md) has not been [reset](/json-rpc/collection/collection_new.md).

When the `Collection` is reset, it is not guaranteed that the same key will map to the same object. Using [`map_*`](/json-rpc/map/map.md) and [`search_*`](/json-rpc/search/index.md) methods as the main way to retrieve information may be more convenient so that `Artist` names, `Album` and `Song` titles can be used as inputs instead.

## Example
Let's search for a song that has a title similar to: "hello"
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_song","params":{"input":"hello","kind":"top1"}}'
```
The output gives us back a `Song` object, which contains some keys:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "songs": [
      {
        "title": "HELLO",
        "key": 15850,   // <--- This is the `Song`'s key
        "album": 737,   // <--- This is the key of the `Album` this `Song` belongs too
        "runtime": 243,
        "sample_rate": 44100,
        "track": 8,
        "disc": 1,
        "mime": "audio/x-flac",
        "extension": "flac"
      }
    ]
  },
  "id": 0
}
```
We can now use that `Album` key to get the `Album`:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_album","params":{"key":"737"}}'
```
And we get the `Album` object back:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "title": "Hero",
    "key": 737,    // <--- This is the `Album`'s key
    "artist": 196, // <--- This is the key of the `Artist` this `Album` belongs too
    "release": "2001-12-29",
    "runtime": 4676,
    "song_count": 3,
    "songs": [
      15850, // <--- Hey look, it's the original `Song` key
      9771,
      15853
    ],
    "discs": 0,
    "art": 8348685,
    "genre": null
  },
  "id": 0
}
```
We can now use that `Artist` key to get the `Artist`:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_artist","params":{"key":"196"}}'
```
And we get the `Artist` object back:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "name": "Tac",
    "key": 196, // <--- This is the `Artist`' key
    "runtime": 15636,
    "albums": [
      737, // <--- Hey look, it's the original `Album` key
    ],
    "songs": [
      15850, // <--- Hey look, it's the original `Song` key
      9771,
      15853
    ]
  },
  "id": 0
}
```
With these keys, you traverse up and down the relational graph.
