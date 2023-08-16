# seek

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Seek forwards/backwards or to an absolute second in the current [`Song`](/common-objects/song.md).

Seeking forwards pass the remaining `Song` length will finish the `Song`.

Seeking to an absolute position longer than the `Song` length will also finish the `Song`.

Seeking backwards more than already-played `Song` length will reset the `Song`.

#### Inputs
| Field  | Type                                             | Description |
|--------|--------------------------------------------------|-------------|
| kind   | string, one of `forward`, `backward`, `absolute` | The "type" of seeking we should do. `forward` means advance the current `Song` by the provided `second`. `backward` means go back in the current `Song` by the provided `second`. `absolute` means skip to the exact `second` in the `Song`, e.g, to skip to the 1 minute mark in the current `Song`, you would use `absolute` + `60`.
| second | unsigned integer                                 | The `second` to seek forward/backwards/to.

#### Outputs
`null` if everything went ok.

#### Example Request
```bash
festival-cli seek --kind absolute --second 60
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"seek","params":{"kind":"absolute","second":60}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
