# collection_perf

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

View some performance stats about the latest [`Collection`](../../common-objects/collection.md) [reset](collection_new.md).

#### Inputs

`None`

#### Outputs

| Field | Type             | Description |
|-------|------------------|-------------|
| bytes | unsigned integer | Total size of the `Collection` (`collection.bin` only, not audio/art) in bytes
| user  | float            | `Collection` creation time in seconds, for the user
| sys   | float            | `Collection` creation time in seconds, for the system `festivald` is running on

#### Example Request
```bash
festival-cli collection_perf
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_perf"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "bytes": 2554823,
    "user": 0.45682073,
    "sys": 0.48661286
  },
  "id": 0
}
```
