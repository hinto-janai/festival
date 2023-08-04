# repeat_off
Turn off queue repeating.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"repeat_off"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
