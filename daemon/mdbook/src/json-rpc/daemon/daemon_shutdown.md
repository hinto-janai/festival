# daemon_shutdown

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Send a shutdown signal to `festivald`.

`festivald` will:
- Reject future connections
- Wait for a [`Collection`](/json-rpc/collection/collection_new.md) reset to finish if occuring
- Clean cache
- Save state to disk

Then shutdown.

#### Inputs
`None`

#### Outputs
| Field               | Type             | Description |
|---------------------|------------------|-------------|
| uptime              | unsigned integer | Uptime of `festivald` in seconds
| uptime_string       | string           | Uptime of `festivald` in human-readable time (60 is `1 minute`)
| total_requests      | unsigned integer | Total amount of requests this `festivald` has received since starting
| total_connections   | unsigned integer | Total amount of connections `festivald` has gotten since starting

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
    "uptime_string": "4 seconds",
    "total_requests": 1,
    "total_connections": 1
  },
  "id": "festival-cli"
}
```
