# map_artist

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Input an [`Artist`](../../common-objects/artist.md) name, retrieve an `Artist` object.

#### Inputs

| Field  | Type   | Description |
|--------|--------|-------------|
| artist | string | `Artist` name

#### Outputs

| Field  | Type            | Description |
|--------|-----------------|-------------|
| artist | `Artist` object | See [`Artist`](../../common-objects/artist.md)

#### Example Request
```bash
festival-cli map_artist --artist "Rex Orange County"
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"map_artist","params":{"artist":"Rex Orange County"}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "Rex Orange County",
      "key": 65,
      "runtime": 7583,
      "albums": [
        237
      ],
      "songs": [
        2800,
        2803,
        2809
      ]
    }
  },
  "id": 0
}
```
