# state_config

#### 🔴 Unstable
This API may be [changed](/api-stability/marker.md) in the future.

---

Retrieve the active configuration of `festivald`.

The output is almost a mirror of the configuration file.

See the [`Config`](/config.md) chapter to see the full description of these fields.

The reason why this method is `🔴 Unstable` is because it will output _all_ `JSON-RPC` methods & `REST` resources, even `🔴 Unstable` ones, which may not exist in the future.

#### Inputs

`None`

#### Outputs

| Field               | Type             |
|---------------------|------------------|
| ip                  | string (IPv4 address)
| port                | unsigned integer
| max_connections     | optional (maybe-null) unsigned integer
| exclusive_ips       | optional (maybe-null) array of strings (IPv4 addresses)
| sleep_on_fail       | optional (maybe-null) unsigned integer
| collection_paths    | array of strings (PATHs)
| tls                 | boolean
| certificate         | optional (maybe-null) string (PATH)
| key                 | optional (maybe-null) string (PATH)
| rest                | boolean
| docs                | boolean
| direct_download     | boolean
| filename_separator  | string
| log_level           | string, one of `OFF`, `ERROR`, `WARN`, `INFO`, `DEBUG`, `ERROR`
| watch               | boolean
| cache_clean         | boolean
| cache_time          | unsigned integer
| restore_audio_state | boolean
| media_controls      | boolean
| authorization       | boolean
| confirm_no_tls_auth | boolean
| no_auth_rpc         | optional (maybe-null) array of [`JSON-RPC Method`](/json-rpc/json-rpc.md) names
| no_auth_rest        | optional (maybe-null) array of [`REST Resource`](/authorization/rest.md) names
| no_auth_docs        | boolean

#### Example Request
```bash
festival-cli state_config
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_config"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ip": "127.0.0.1",
    "port": 18425,
    "max_connections": null,
    "exclusive_ips": null,
    "sleep_on_fail": 3000,
    "collection_paths": [
      "/home/hinto/Music"
    ],
    "tls": true,
    "certificate": "/home/hinto/festival/assets/tls/cert.pem",
    "key": "/home/hinto/festival/assets/tls/key.pem",
    "rest": true,
    "docs": true,
    "direct_download": false,
    "filename_separator": " - ",
    "log_level": "TRACE",
    "watch": true,
    "cache_clean": true,
    "cache_time": 3600,
    "restore_audio_state": true,
    "media_controls": true,
    "authorization": true,
    "confirm_no_tls_auth": false,
    "no_auth_rpc": [
      "toggle"
    ],
    "no_auth_rest": [
      "song"
    ],
    "no_auth_docs": true
  },
  "id": 0
}
```
