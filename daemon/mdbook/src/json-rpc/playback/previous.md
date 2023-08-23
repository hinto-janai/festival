# previous

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Set the current [`Song`](../../common-objects/song.md) to the previous in the [queue](../queue/queue.md).

If `threshold` is not specified, the [`previous_threshold`](../../config.md) config option will be used.

#### Inputs
| Field     | Type                                   | Description |
|-----------|----------------------------------------|-------------|
| threshold | optional (maybe-null) unsigned integer | If the current `Song` runtime (seconds) has passed this number, this method will reset the current `Song` instead of skipping backwards. Setting this to `0` will make this method always go to the previous `Song`.

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
