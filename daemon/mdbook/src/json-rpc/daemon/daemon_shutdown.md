# daemon_shutdown

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Send a shutdown signal to `festivald`.

`festivald` will:
- Reject future connections
- Wait for a [`Collection`](../../common-objects/collection.md) [reset](../collection/collection_new.md) to finish if occurring
- [Clean cache](../daemon/daemon_remove_cache.md)
- [Save](../daemon/daemon_save.md) state to disk

Then shutdown.

#### Inputs
`None`

#### Outputs
| Field               | Type             | Description |
|---------------------|------------------|-------------|
| uptime              | unsigned integer | Uptime of `festivald` in seconds
| uptime_readable     | string           | Uptime of `festivald` in human-readable time (60 is `1 minute`)
| total_requests      | unsigned integer | Total amount of requests `festivald` has received since starting
| total_connections   | unsigned integer | Total amount of connections `festivald` has received since starting

#### Example Request
```bash
festival-cli daemon_shutdown
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_shutdown"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "uptime": 4,
    "uptime_readable": "4 seconds",
    "total_requests": 1,
    "total_connections": 1
  },
  "id": "festival-cli"
}
```
