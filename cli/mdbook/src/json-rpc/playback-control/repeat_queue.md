# repeat_queue
Turn on queue repeating.

If at the last `Song` in the queue, it will wrap around and start again at the first `Song` in the queue.

#### Inputs
`None`

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"repeat_queue"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
