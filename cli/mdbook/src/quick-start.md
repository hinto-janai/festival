# Quick Start
A quick start to using `festival-cli`.

Important things to know:
1. The main music library/database in `festivald` is called the [`Collection`](common-objects/collection.md)
2. The `Collection` is created by scanning the filesystem `festivald` is running on
3. Everything revolves around the `Collection`, you should create one before doing anything
4. In `festivald`, there are "objects" that appear often, see [`Common Objects`](common-objects/common-objects.md) for more info

## Launch `festivald` & create a new `Collection` with `festival-cli`
Start `festivald` with no options:
```bash
./festivald
```
`festivald` will use a default [`configuration`](config.md) and open up on `http://localhost:18425`.

You can then tell `festivald` to create a new `Collection` with:
```bash
festival-cli collection_new /path/to/collection/on/festivald
```
The PATH used _must_ be a PATH on the machine `festivald` is running on, not the machine `festival-cli` is ran on.

If no PATH is specified, 4 things can happen:
- Your `festival-cli`'s custom [config](config.md) PATH is used
- `festival-cli`'s default PATH is used (OS Music directory)
- `festivald`'s custom config PATH is used
- `festivald`'s default PATH is used (OS Music directory)

If you have a custom PATH set in [`festival-cli.toml`](config.md), that PATH will be used if no PATH is specified on the command line.

## View metadata about an `Album` with [`map_artist`](json-rpc/map/map_artist.md)
This will lookup the `Album` "Cigarette & Alcohol" by the `Artist` "LUCKY TAPES".
```bash
festival-cli map_artist --artist "LUCKY TAPES" --album "Cigarette & Alcohol"
```
The response looks like:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "title": "Cigarette & Alcohol",
    "key": 752,
    "artist": 169,
    "release": "2016-07-06",
    "runtime": 2593,
    "song_count": 10,
    "songs": [
      7611,
      7616,
      7618,
      7619,
      7620,
      7621,
      7622,
      7623,
      7624,
      7626
    ],
    "discs": 0,
    "art": 1947006,
    "genre": null
  },
  "id": 0
}
```

## Search for an `Artist` with [`search_artist`](json-rpc/search/search_artist.md)
This will look up _all_ `Artist`'s in the `Collection`, and return the one that is the most similar (lexicographically) to the input "lUcKee TaPeZ".
```bash
festival-cli search_artist --input "lUcKee TaPeZ" --kind "top1"
```
We found "LUCKY TAPES":
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artists": [
      {
        "name": "LUCKY TAPES",
        "key": 169,
        "runtime": 2593,
        "albums": [
          752
        ],
        "songs": [
          7611,
          7616,
          7618,
          7619,
          7620,
          7621,
          7622,
          7623,
          7624,
          7626
        ]
      }
    ]
  },
  "id": 0
}
```

## Add an `Artist` to the queue with [`add_queue_map_artist`](json-rpc/queue/add_queue_map_artist)
This will add all `Song`'s by the `Artist` "LUCKY TAPES" to the front of the queue.
```bash
festival-cli add_queue_map_artist --artist "LUCKY TAPES" --append front
```
