# key_album

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Input an `Album` [key](../../common-objects/key.md), retrieve an [`Album`](../../common-objects/album.md).

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| key   | `Album` key (unsigned integer)                 | See [`Key`](../../common-objects/key.md)

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| album | `Album` object | See [`Album`](../../common-objects/album.md)

#### Example Request
```bash
festival-cli key_album --key 237
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_album","params":{"key":237}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "RAINBOW",
      "key": 237,
      "artist": 65,
      "release": "????-??-??",
      "runtime": 1090,
      "song_count": 6,
      "songs": [
        2594,
        2540,
        2600,
        2496,
        2557,
        2500
      ],
      "discs": 0,
      "art": 7753,
      "genre": null
    }
  },
  "id": 0
}
```
