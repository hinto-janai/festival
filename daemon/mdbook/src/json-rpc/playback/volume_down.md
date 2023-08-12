# volume_down
Lower the playback volume.

#### Inputs
| Field | Type             | Description |
|-------|------------------|-------------|
| down  | unsigned integer | The number to lower the volume by. Must be in-between 0..100.

#### Outputs
| Field    | Type             | Description |
|----------|------------------|-------------|
| previous | unsigned integer | What the volume was set to previously
| current  | unsigned integer | What the volume is now set at

#### Example Request
```bash
festival-cli volume_down --down 15
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume_down","params":{"down":15}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "previous": 15,
    "current": 0
  },
  "id": "festival-cli"
}
```
