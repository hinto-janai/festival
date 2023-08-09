# state_audio
Retrieve audio state.

#### Inputs

`None`

#### Outputs

| Field     | Type                                                | Description |
|-----------|-----------------------------------------------------|-------------|
| queue     | array of `Song` keys (unsigned integers)            | Array of `Song` keys that are in the queue, in order of what will be played next
| queue_idx | optional (maybe null) unsigned integer              | The queue index `festivald` is currently on, `null` if no `Song` is set
| playing   | boolean                                             | If `festivald` is currently playing
| song_key  | optional (maybe null) `Song` key (unsigned integer) | The key of current `Song`, `null` if no `Song` is set
| elapsed   | unsigned integer                                    | Elapsed runtime of current `Song` in seconds
| runtime   | unsigned integer                                    | Total runtime of current `Song` in seconds
| repeat    | string, one of `song`, `queue`, or `off`            | Audio repeat behavior. `song` means the `Song` will repeat after ending, `queue` means the whole queue will repeat after ending, `off` means the queue will be cleared and playback will stop when ending
| volume    | unsigned integer in between `0..100`                | The current volume level
| song      | optional (maybe null) `Song` object                 | The current `Song` as an object, `null` if no `Song` is set


#### Example Request
```bash
festival-cli state_audio
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_audio"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "queue": [
      44
    ],
    "queue_len": 1,
    "queue_idx": 0,
    "playing": true,
    "song_key": 44,
    "elapsed": 4,
    "runtime": 349,
    "repeat": "off",
    "volume": 25,
    "song": {
      "title": "いつか",
      "key": 44,
      "album": 10,
      "runtime": 349,
      "sample_rate": 44100,
      "track": 1,
      "disc": 1,
      "mime": "audio/x-flac",
      "extension": "flac"
    }
  },
  "id": 0
}
```
