# map_song
Input an `Artist` name, `Album` title, and `Song` title, retrieve a [`Song`](../../common-objects/song.md).

#### Inputs

| Field  | Type   | Description |
|--------|--------|-------------|
| artist | string | `Artist` name
| album  | string | `Album` title
| song   | string | `Song` title

#### Outputs

| Field | Type          | Description |
|-------|---------------|-------------|
| song  | `Song` object | See [`Song`](../../common-objects/song.md)

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_song","params":{"artist":"Rex Orange County","album":"RAINBOW","song":"SUNFLOWER"}}'
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