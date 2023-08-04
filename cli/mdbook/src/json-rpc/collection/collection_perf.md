# collection_perf
View some performance stats about the latest [`Collection`](../../common-objects/collection.md) construction.

#### Inputs

| Field | Type                                 | Description |
|-------|--------------------------------------|-------------|
| paths | optional (maybe null) array of PATHs | An array of filesystem PATHs to scan for the new `Collection`. These must be absolute PATHs **on the system `festivald` is running on**, not PATHs on the client. If `null` is provided, the default `Music` directory will be used.

#### Outputs

| Field | Type             | Description |
|-------|------------------|-------------|
| bytes | unsigned integer | Total size of the `Collection` (`collection.bin` only, not audio/art) in bytes
| user  | float            | `Collection` creation time in seconds, for the user
| sys   | float            | `Collection` creation time in seconds, for the system `festivald` is running on

#### Example Request
```bash
# Windows PATH works too if `\` is escaped (and if `festivald` is running on Windows).
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_perf"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "bytes": 2649405,
    "user": 0.3863234519958496,
    "sys": 0.4135190546512604
  },
  "id": 0
}
```
