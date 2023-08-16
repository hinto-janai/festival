# key_song_album

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Input a [`Song`](/common-objects/song.md) [key](/common-objects/key.md), retrieve its [`Album`](/common-objects/album.md).

#### Inputs

| Field | Type                                         | Description |
|-------|----------------------------------------------|-------------|
| key   | `Song` key (unsigned integer)                | See [`Key`](/common-objects/key.md)

#### Outputs

| Field | Type           | Description |
|-------|----------------|-------------|
| album | `Album` object | See [`Album`](/common-objects/album.md)

#### Example Request
```bash
festival-cli key_song_album --key 2540
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_song_album","params":{"key":2540}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "album": {
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
    }
  },
  "id": 0
}
```
