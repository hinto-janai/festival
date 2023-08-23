# daemon_save

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Save `festivald` data to disk.

Currently this saves:
- [Audio state](../state/state_audio.md)
- [Playlists](../../common-objects/playlist.md)

The [`Collection`](../../common-objects/collection.md) is always saved automatically after creation.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli daemon_save
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_save"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
