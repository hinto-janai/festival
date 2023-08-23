# key_artist_albums

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Input an [`Artist`](../../common-objects/artist.md) [key](../../common-objects/key.md), retrieve all their [`Album`](../../common-objects/album.md)'s.

The `Album`'s are sorted by `Release date`.

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| key   | `Artist` key (unsigned integer)                | See [`Key`](../../common-objects/key.md)

#### Outputs

| Field  | Type                     | Description |
|--------|--------------------------|-------------|
| len    | unsigned integer         | How many `Album`'s there are
| albums | array of `Album` objects | See [`Album`](../../common-objects/album.md)

#### Example Request
```bash
festival-cli key_artist_albums --key 62
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_artist_albums","params":{"key":62}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "albums": [
      {
        "title": "Apricot Princess",
        "key": 234,
        "artist": 62,
        "release": "2017",
        "runtime": 2370,
        "song_count": 10,
        "songs": [
          2463,
          2471,
          2483,
          2492,
          2498,
          2504,
          2514,
          2522,
          2530,
          2536
        ],
        "discs": 0,
        "art": 307745,
        "genre": "Pop"
      },
      {
        "title": "Pony",
        "key": 241,
        "artist": 62,
        "release": "2019-09-19",
        "runtime": 2032,
        "song_count": 10,
        "songs": [
          2540,
          2545,
          2548,
          2553,
          2558,
          2567,
          2573,
          2578,
          2581,
          2587
        ],
        "discs": 0,
        "art": 190830,
        "genre": "Alternative & Indie"
      },
      {
        "title": "WHO CARES?",
        "key": 247,
        "artist": 62,
        "release": "2022",
        "runtime": 2091,
        "song_count": 11,
        "songs": [
          2590,
          2592,
          2596,
          2598,
          2602,
          2606,
          2607,
          2610,
          2614,
          2618,
          2622
        ],
        "discs": 0,
        "art": 80994,
        "genre": "Alternative & Indie"
      }
    ]
  },
  "id": 0
}
```
