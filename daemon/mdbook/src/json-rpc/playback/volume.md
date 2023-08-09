# volume
Set the playback volume.

#### Inputs
| Field     | Type             | Description |
|-----------|------------------|-------------|
| volume    | unsigned integer | The volume % to set. Must be in-between `0..100`. If greater than `100`, it will set the volume to `100`.

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli volume --volume 15
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume","params":{"volume":15}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
