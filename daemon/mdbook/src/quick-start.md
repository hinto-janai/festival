# Quick Start
A quick start to using `festivald` and its `JSON-RPC 2.0` & `REST` APIs.

More JSON-RPC specific examples [here](json-rpc/quick-start.md), and more REST specific examples [here](rest/quick-start.md).

Important things to know:
1. The main music library/database in `festivald` is called the [`Collection`](common-objects/collection.md)
2. The `Collection` is created by scanning the filesystem `festivald` is running on
3. Everything revolves around the `Collection`, you should create one before doing anything
4. In `festivald`, there are "objects" that appear often, see [`Common Objects`](common-objects/common-objects.md) for more info

## Launch `festivald`
Start `festivald` with no options:
```bash
./festivald
```
`festivald` will use a default [`configuration`](config.md) and open up on `http://localhost:18425`.

To view this documentation locally after starting `festivald`, you can open it in a web browser:
```http
http://localhost:18425
```
Or you can open the files locally with:
```bash
./festivald --docs
```

## Create a `Collection` with the `JSON-RPC` method [`collection_new`](json-rpc/collection/collection_new.md)
This scans the default `Music` directory on `festivald`'s filesystem and creates a `Collection` from it.
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":null}}'
```

## Download a random `Artist` with the `REST` endpoint [`/rand/artist`](/rest/rand/artist.md)
Opening this link in a web browser will cause `festivald` to collect, organize, and archive all the `Album`'s of a random `Artist`, and send it over.
```http
http://localhost:18425/rand/artist
```

## View metadata about an `Album` with the `JSON-RPC` method [`map_artist`](json-rpc/map/map_artist.md)
This will lookup the `Album` "Cigarette & Alcohol" by the `Artist` "LUCKY TAPES".
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_album","params":{"artist":"LUCKY TAPES","album":"Cigarette & Alcohol"}}'
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

## Search for an `Artist` with the `JSON-RPC` method [`search_artist`](json-rpc/search/search_artist.md)
This will look up _all_ `Artist`'s in the `Collection`, and return the one that is the most similar (lexicographically) to the input "lUcKee TaPeZ":
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_artist","params":{"input":"lUcKee TaPeZ","kind":"top1"}}'
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
