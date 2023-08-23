# queue_add_map_song

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Add a [`Song`](../../common-objects/song.md) to the queue with an [`Artist`](../../common-objects/artist.md) name [`Album`](../../common-objects/album.md) title, and `Song` title.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| artist | `string`                                    | `Artist` name
| album  | `string`                                    | `Album` title
| song   | `string`                                    | `Song` title
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](../queue/queue.md#append)
| clear  | boolean                                     | Should the queue be cleared before adding?
| play   | boolean                                     | Should we start playing?
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used

#### Outputs
`result: null` if everything went ok.

`error: ...` if there was an index/offset error.

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_map_song --artist TWICE --album "PAGE TWO" --song "CHEER UP" --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_song","params":{"artist":"TWICE","album":"PAGE TWO","song":"CHEER UP","append":"back","clear":false,"play":false}}'
```

#### Example Request 2
Insert at queue index 4, start from `Song` 3 (offset 2).
```bash
festival-cli queue_add_map_song --artist TWICE --album "PAGE TWO"  --song "CHEER UP" --append index --index 4 
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_song","params":{"artist":"TWICE","album":"PAGE TWO","song":"CHEER UP","append":"index","clear":false,"play":false,"index":4}}'
```

#### Example Request 3
Clear the queue, add the `Song` "CHEER UP".
```bash
festival-cli queue_add_map_song --artist TWICE --album "PAGE TWO" --song "CHEER UP" --append front --clear --play
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_song","params":{"artist":"TWICE","album":"PAGE TWO","song":"CHEER UP","append":"front","clear":true,"play":true}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
