# queue_add_playlist

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Add a [`Playlist`](../../common-objects/playlist.md) to the queue.

#### Inputs

| Field    | Type                                        | Description |
|----------|---------------------------------------------|-------------|
| playlist | `string`                                    | The `Playlist`'s name
| append   | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](../queue/queue.md#append)
| clear    | optional (maybe-null) boolean               | Should the queue be cleared before adding?
| play     | optional (maybe-null) boolean               | Should we start playing?
| index    | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| offset   | optional (maybe-null) unsigned integer      | See [`Queue/offset`](../queue/queue.md#offset)

#### Outputs
`result: null` if everything went ok.

`error: ...` if there was an index/offset error or if the playlist didn't exist.

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_playlist --playlist my_playlist --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_playlist","params":{"playlist":"my_playlist","append":"back","clear":false,"play":false}}'
```

#### Example Request 2
Insert at queue index 4, start from `Song` 3 (offset 2).
```bash
festival-cli queue_add_playlist --playlist my_playlist --append index --index 4 --offset 2
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_playlist","params":{"playlist":"my_playlist","append":"index","clear":false,"play":false,"index":4,"offset":2}}'
```

#### Example Request 3
Clear the queue, add starting from `Song` 5 (offset 4).
```bash
festival-cli queue_add_playlist --playlist my_playlist --append front --clear --play --offset 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_playlist","params":{"playlist":"my_playlist","append":"front","clear":true,"play":false,"offset":4}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
