# daemon_methods

#### 🔴 Unstable
This API may be [changed](../../api-stability/marker.md) in the future.

---

Retrieve all [`JSON-RPC` methods](../json-rpc.md) this `festivald` knows about.

The reason why this method is `🔴 Unstable` is because it will output _all_ methods, even `🔴 Unstable` ones, which may not exist in the future.

Ordering of the method names [should not be relied upon](../../api-stability/json-rpc.md).

#### Inputs
`None`

#### Outputs
| Field   | Type                | Description |
|---------|---------------------|-------------|
| len     | unsigned integer    | Total amount of methods
| methods | array of `string`'s | The names of all the methods this `festivald` knows about

#### Example Request
```bash
festival-cli daemon_methods
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_methods"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "len": 109,
    "methods": [
      "collection_new",
      "collection_brief",
      "collection_full",
      "collection_brief_artists",
      "collection_brief_albums",
      "collection_brief_songs",
      "collection_full_artists",
      "collection_full_albums",
      "collection_full_songs",
      "collection_entries",
      "collection_perf",
      "collection_health",
      "collection_resource_size",
      "daemon_save",
      "daemon_remove_cache",
      "daemon_shutdown",
      "daemon_methods",
      "daemon_no_auth_rpc",
      "daemon_no_auth_rest",
      "state_audio",
      "state_config",
      "state_daemon",
      "state_ip",
      "state_queue_key",
      "state_queue_song",
      "state_queue_entry",
      "state_playing",
      "state_repeat",
      "state_runtime",
      "state_volume",
      "key_artist",
      "key_album",
      "key_song",
      "key_entry",
      "key_artist_albums",
      "key_artist_songs",
      "key_artist_entries",
      "key_album_artist",
      "key_album_songs",
      "key_album_entries",
      "key_song_artist",
      "key_song_album",
      "key_other_albums",
      "key_other_songs",
      "key_other_entries",
      "map_artist",
      "map_album",
      "map_song",
      "map_entry",
      "map_artist_albums",
      "map_artist_songs",
      "map_artist_entries",
      "map_album_songs",
      "map_album_entries",
      "current_artist",
      "current_album",
      "current_song",
      "current_entry",
      "rand_artist",
      "rand_album",
      "rand_song",
      "rand_entry",
      "search",
      "search_artist",
      "search_album",
      "search_song",
      "search_entry",
      "toggle",
      "play",
      "pause",
      "next",
      "stop",
      "previous",
      "clear",
      "seek",
      "skip",
      "back",
      "shuffle",
      "repeat",
      "volume",
      "volume_up",
      "volume_down",
      "queue_add_key_artist",
      "queue_add_key_album",
      "queue_add_key_song",
      "queue_add_map_artist",
      "queue_add_map_album",
      "queue_add_map_song",
      "queue_add_rand_artist",
      "queue_add_rand_album",
      "queue_add_rand_song",
      "queue_add_rand_entry",
      "queue_add_playlist",
      "queue_set_index",
      "queue_remove_range",
      "playlist_new",
      "playlist_remove",
      "playlist_clone",
      "playlist_get_index",
      "playlist_remove_index",
      "playlist_add_key_artist",
      "playlist_add_key_album",
      "playlist_add_key_song",
      "playlist_add_map_artist",
      "playlist_add_map_album",
      "playlist_add_map_song",
      "playlist_single",
      "playlist_brief",
      "playlist_full"
    ]
  },
  "id": 0
}
```
