# queue_add_rand_album

#### 🟡 Incomplete
This API may have [additions](/api-stability/marker.md) in the future.

---

Add a random [`Album`](/common-objects/album.md) to the queue.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](/json-rpc/queue/queue.md#append)
| clear  | optional (maybe-null) boolean               | Should the queue be cleared before adding? `null` or no field at all is equal to `false`.
| play   | optional (maybe-null) boolean               | Should we start playing? `null` or no field at all is equal to `false`.
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| offset | optional (maybe-null) unsigned integer      | See [`Queue/offset`](/json-rpc/queue/queue.md#offset)

#### Outputs
| Field         | Type                                       | Description |
|---------------|--------------------------------------------|-------------|
| album         | [`Album`](/common-objects/album.md) object | The `Album` that was added to the queue

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_rand_album --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_album","params":{"append":"back"}}'
```

#### Example Request 2
Insert at queue index 4.
```bash
festival-cli queue_add_rand_album --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_album","params":{"append":"index","index":4}}'
```

#### Example Request 3
Clear the queue, add all the `Song`'s in this `Album`, but start at the 5th `Song` (offset 4).
```bash
festival-cli queue_add_rand_album --append front --clear --offset 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_rand_album","params":{"append":"front","clear":true,"offset":4}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "RAINBOW",
      "key": 237,
      "artist": 65,
      "release": "????-??-??",
      "runtime": 1090,
      "song_count": 6,
      "songs": [
        2594,
        2540,
        2600,
        2496,
        2557,
        2500
      ],
      "discs": 0,
      "art": 7753,
      "genre": null
    }
  },
  "id": 0
}
```
