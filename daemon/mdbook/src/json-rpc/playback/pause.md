# pause
Pause playback.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli pause
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"pause"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
