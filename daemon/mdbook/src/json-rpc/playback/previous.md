# previous

{{#include ../../marker/s}} v1.0.0`

---

Set the current `Song` to the previous in the queue.

#### Inputs
| Field     | Type             | Description |
|-----------|------------------|-------------|
| threshold | unsigned integer | If the current `Song` runtime (seconds) has passed this number, this method will reset the current `Song` instead of skipping backwards. Setting this to `0` will make this method always go to the previous `Song`.

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli previous --threshold 0
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"previous","params":{"threshold":0}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
