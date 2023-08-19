# daemon_remove_cache

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Remove all the cached [`REST resource`](../../rest/rest.md) files on disk.

Warning: If there is a connection currently using this cache, it will error.

#### Inputs
`None`

#### Outputs
The output is an un-named array containing:

| Field | Type             | Description |
|-------|------------------|-------------|
| path  | string (PATH)    | A file that was removed
| bytes | unsigned integer | How many bytes that file was

#### Example Request
```bash
festival-cli daemon_remove_cache
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_remove_cache"}'
```

#### Example Response
```
{
  "jsonrpc": "2.0",
  "result": [
    {
      "path": "/home/hinto/.cache/festival/daemon/zip/artist/Artist Name.zip"
      "bytes": 345673
    },
    {
      "path": "/home/hinto/.cache/festival/daemon/zip/album/Album Title.zip"
      "bytes": 345673
    }
  ],
  "id": 0
}
```
