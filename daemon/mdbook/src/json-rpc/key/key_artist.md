# key_artist

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Input an `Artist` [key](../../common-objects/key.md), retrieve an [`Artist`](../../common-objects/artist.md).

#### Inputs

| Field | Type                                           | Description |
|-------|------------------------------------------------|-------------|
| key   | `Artist` key (unsigned integer)                | See [`Key`](../../common-objects/key.md)

#### Outputs

| Field  | Type            | Description |
|--------|-----------------|-------------|
| artist | `Artist` object | See [`Artist`](../../common-objects/artist.md)

#### Example Request
```bash
festival-cli key_artist --key 65
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"key_artist","params":{"key":65}}'
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
