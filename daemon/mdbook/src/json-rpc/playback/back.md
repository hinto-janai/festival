# back
Go backwards a variable amount of `Song`'s in the current queue.

#### Inputs
| Field | Type             | Description |
|-------|------------------|-------------|
| back  | unsigned integer | How many `Song`'s to go backwards. This will _not_ wrap around if we hit the 1st `Song`.

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli back --back 10
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"back","params":{"back":10}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
