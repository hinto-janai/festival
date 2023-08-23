# key_song

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Input a `Song` [key](../../common-objects/key.md), retrieve a [`Song`](../../common-objects/song.md).

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| key   | `Song` key (unsigned integer)                  | See [`Key`](../key.md)

#### Outputs

| Field | Type          | Description |
|-------|---------------|-------------|
| song  | `Song` object | See [`Song`](../../common-objects/song.md)

#### Example Request
```bash
festival-cli key_song --key 2594
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_song","params":{"key":2594}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "song": {
      "title": "SUNFLOWER",
      "key": 2594,
      "album": 237,
      "runtime": 252,
      "sample_rate": 44100,
      "track": 1,
      "disc": null,
      "mime": "audio/mpeg",
      "extension": "mp3"
    }
  },
  "id": 0
}
```
