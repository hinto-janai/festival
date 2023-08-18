# state_runtime

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve the elapsed runtime & total runtime of the currently set [`Song`](/common-objects/key.md).

This will return `0` values (`0:00`) if there is no `Song` set.

#### Inputs

`None`

#### Outputs

| Field            | Type             | Description |
|------------------|------------------|-------------|
| elapsed          | unsigned integer | Elapsed runtime of current `Song` in seconds
| runtime          | unsigned integer | Total runtime of current `Song` in seconds
| elapsed_readable | string           | Human-readable version of `elapsed` (`5` would be `0:05`)
| runtime_readable | string           | Human-readable version of `runtime` (`60` would be `1:00`)

#### Example Request
```bash
festival-cli state_runtime
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_runtime"}'
```

#### Example Response 1
Song is set:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "elapsed": 12,
    "runtime": 125,
    "elapsed_readable": "0:12",
    "runtime_readable": "2:05"
  },
  "id": 0
}
```

#### Example Response 2
Song is not set:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "elapsed": 0,
    "runtime": 0,
    "elapsed_readable": "0:00",
    "runtime_readable": "0:00"
  },
  "id": 0
}
```
