# collection_brief

{{#include ../../marker/s}} v1.0.0`

---

Retrieve some brief metadata about the current [`Collection`](../../common-objects/collection.md).

This method is a subset of the [`collection_full`](collection_full.md) method.

#### Inputs

`None`

#### Outputs

| Field        | Type             | Description |
|--------------|------------------|-------------|
| empty        | boolean          | If the `Collection` does NOT have any `Artist`'s, `Album`'s, or `Song`'s
| timestamp    | unsigned integer | The UNIX timestamp of when this `Collection` was created
| count_artist | unsigned integer | How many unique `Artist`'s there are in this `Collection`
| count_album  | unsigned integer | How many unique `Album`'s there are in this `Collection`
| count_song   | unsigned integer | How many unique `Song`'s there are in this `Collection`
| count_art    | unsigned integer | How much unique `Album` art there are in this `Collection`

#### Example Request
```bash
festival-cli collection_brief
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_brief"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "empty": false,
    "timestamp": 1690410052,
    "count_artist": 195,
    "count_album": 825,
    "count_song": 8543,
    "count_art": 824
  },
  "id": 0
}
```
