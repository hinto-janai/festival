# rand_artist

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Access a random [`Artist`](/common-objects/artist.md) in your [`Collection`](/common-objects/collection.md).

#### Inputs

`None`

#### Outputs

| Field  | Type            | Description |
|--------|-----------------|-------------|
| artist | `Artist` object | See [`Artist`](/common-objects/artist.md)

#### Example Request
```bash
festival-cli rand_artist
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"rand_artist"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "カネコアヤノ",
      "key": 65,
      "runtime": 4709,
      "albums": [
        276,
        256
      ],
      "songs": [
        2883,
        2504,
        2859,
        2863,
        2866,
        2869,
        2873,
        2874,
        2693,
        2694
      ]
    }
  },
  "id": 0
}
```
