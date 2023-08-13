# repeat_song

{{#include ../../marker/s}} v1.0.0`

---

Turn on song repeating.

After the end of a `Song`, play it again, forever.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli repeat_song
```
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
