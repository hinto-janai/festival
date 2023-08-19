# state_queue_entry

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve state about the queue.

This is the same as [`state_queue`](../state/state_queue.md) except it returns [`Entry`](../../common-objects/entry.md)'s instead of [`Song`](../../common-objects/song.md)'s.

Returned `Entry`'s are in order of what will be played next.

#### Inputs
`None`

#### Outputs

| Field   | Type                     | Description |
|---------|--------------------------|-------------|
| len     | unsigned integer         | Length of the queue
| entries | array of `Entry` objects | Array of the queue's `Song`'s, in `Entry` form


#### Example Request
```bash
festival-cli state_queue_entry
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_queue_entry"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "entries": [
      {
        "path": "/home/hinto/Music/Rex Orange County/RAINBOW/SUNFLOWER.mp3",
        "key_artist": 65,
        "key_album": 237,
        "key_song": 2539,
        "artist": "Rex Orange County",
        "album": "RAINBOW",
        "song": "SUNFLOWER"
      },
      {
        "path": "/home/hinto/Music/Rex Orange County/RAINBOW/BEST FRIEND.mp3",
        "key_artist": 65,
        "key_album": 237,
        "key_song": 2517,
        "artist": "Rex Orange County",
        "album": "RAINBOW",
        "song": "BEST FRIEND"
      }
    ]
  },
  "id": 0
}
```
