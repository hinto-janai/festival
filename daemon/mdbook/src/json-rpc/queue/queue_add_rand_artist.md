# queue_add_rand_artist

#### 🟡 Incomplete
This API may have [additions](../../api-stability/marker.md) in the future.

---

Add a random [`Artist`](../../common-objects/artist.md) to the queue.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](../queue/queue.md#append)
| clear  | optional (maybe-null) boolean               | Should the queue be cleared before adding? `null` or no field at all is equal to `false`.
| play   | optional (maybe-null) boolean               | Should we start playing? `null` or no field at all is equal to `false`.
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| offset | optional (maybe-null) unsigned integer      | See [`Queue/offset`](../queue/queue.md#offset)

#### Outputs
| Field         | Type                                              | Description |
|---------------|---------------------------------------------------|-------------|
| artist        | [`Artist`](../../common-objects/artist.md) object | The `Artist` that was added to the queue

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_rand_artist --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_artist","params":{"append":"back"}}'
```

#### Example Request 2
Insert at queue index 4.
```bash
festival-cli queue_add_rand_artist --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_artist","params":{"append":"index","index":4}}'
```

#### Example Request 3
Clear the queue, add all the `Song`'s by this `Artist`, but start at the 5th `Song` (offset 4).
```bash
festival-cli queue_add_rand_artist --append front --clear --offset 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_artist","params":{"append":"front","clear":true,"offset":4}}'
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
