# toggle

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Toggle playback.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli toggle
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"toggle"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
