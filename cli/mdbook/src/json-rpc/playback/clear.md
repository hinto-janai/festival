# clear
Clear the queue.

#### Inputs
| Field    | Type    | Description |
|----------|---------|-------------|
| playback | boolean | If there is a `Song` currently set and playing, should we continue playback?

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"clear","params":{"playback":false}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
