# daemon_state

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Retrieve state about the status of `festivald` itself.

#### Inputs

`None`

#### Outputs

| Field               | Type             | Description |
|---------------------|------------------|-------------|
| uptime              | unsigned integer | Uptime of `festivald` in seconds
| uptime_readable     | string           | Uptime of `festivald` in human-readable time (60 is `1 minute`)
| saving              | boolean          | If `festivald` is currently saving a recently created [`Collection`](../../common-objects/collection.md) to disk
| total_requests      | unsigned integer | Total amount of requests `festivald` has received since starting
| total_connections   | unsigned integer | Total amount of connections `festivald` has received since starting
| current_connections | unsigned integer | Amount of open connections `festivald` currently has
| rest                | boolean          | If this `festivald`'s `REST` API is enabled
| docs                | boolean          | If this `festivald`'s documentation serving is enabled
| direct_download     | boolean          | If this `festivald`'s `REST` API has `direct_download` enabled
| authorization       | boolean          | If this `festivald` has authorization enabled
| version             | string           | Semantic version of this `festivald`
| commit              | string           | Git commit of this `festivald`
| os                  | string           | The OS this `festivald` was built for

#### Example Request
```bash
festival-cli search_daemon
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_state"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "uptime": 43,
    "uptime_readable": "43 seconds",
    "saving": false,
    "total_requests": 1447,
    "total_connections": 297,
    "current_connections": 3,
    "rest": true,
    "docs": true,
    "direct_download": false,
    "authorization": false,
    "version": "v1.0.0",
    "commit": "ae086becca9a603b53ded80de870794858c272d0",
    "os": "Linux x64"
  },
  "id": 0
}
```
