# play

{{#include ../../marker/s}} v1.0.0`

---

Start playback.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli play
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"play"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
