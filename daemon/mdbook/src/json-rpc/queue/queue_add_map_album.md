# queue_add_map_album

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Add an [`Album`](../../common-objects/album.md) to the queue with an [`Artist`](../../common-objects/artist.md) name and `Album` title.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| artist | `string`                                    | `Artist` name
| album  | `string`                                    | `Album` title
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](../queue/queue.md#append)
| clear  | boolean                                     | Should the queue be cleared before adding?
| play   | boolean                                     | Should we start playing?
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| offset | optional (maybe-null) unsigned integer      | See [`Queue/offset`](../queue/queue.md#offset)

#### Outputs
`result: null` if everything went ok.

`error: ...` if there was an index/offset error.

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_map_album --artist TWICE --album "PAGE TWO" --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_album","params":{"artist":"TWICE","album":"PAGE TWO","append":"back","clear":false,"play":false}}'
```

#### Example Request 2
Insert at queue index 4.
```bash
festival-cli queue_add_map_album --artist TWICE --album "PAGE TWO" --append index --index 4 
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_album","params":{"artist":"TWICE","album":"PAGE TWO","append":"index","clear":false,"play":false,"index":4}}'
```

#### Example Request 3
Clear the queue, add all the `Song`'s in this `Album`, but start at the 5th `Song` (offset 4).
```bash
festival-cli queue_add_map_album --artist TWICE --album "PAGE TWO" --append front --clear --play --offset 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_album","params":{"artist":"TWICE","album":"PAGE TWO","append":"front","clear":true,"play":true,"offset":4}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
