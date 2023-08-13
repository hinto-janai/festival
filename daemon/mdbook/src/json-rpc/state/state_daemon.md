# state_daemon

{{#include ../../marker/i}}

---

Retrieve state about the status of `festivald` itself.

#### Inputs

`None`

#### Outputs

| Field               | Type             | Description |
|---------------------|------------------|-------------|
| uptime              | unsigned integer | Uptime of `festivald` in seconds
| total_requests      | unsigned integer | Total amount of requests this `festivald` has received since starting
| total_connections   | unsigned integer | Total amount of connections `festivald` has gotten since starting
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
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_daemon"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "uptime": 43,
    "total_requests": 1447,
    "total_connections": 297,
    "current_connections": 3,
    "rest": true,
    "direct_download": false,
    "authorization": false,
    "version": "v1.0.0",
    "commit": "ae086becca9a603b53ded80de870794858c272d0",
    "os": "Linux x64"
  },
  "id": 0
}
```
