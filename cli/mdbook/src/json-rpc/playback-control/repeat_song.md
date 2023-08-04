# repeat_song
Turn on song repeating.

After the end of a `Song`, play it again, forever.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"repeat_song"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
