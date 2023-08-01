# add_queue_map_song
Add a [`Song`](../../common-objects/song.md) to the queue with an [`Artist`](../../common-objects/artist.md) name [`Album`](../../common-objects/album.md) title, and `Song` title.

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| artist | `string`                                    | `Artist` name
| album  | `string`                                    | `Album` title
| song   | `string`                                    | `Song` title
| append | `string`, one of `front`, `back` or `index` | In which way should we add to the queue? `front` means to the front of the queue. `back` means to the back. `index` means at an exact queue index. Queue index starts at `0`, so to mimic `front`, you would provide `0`.
| clear  | boolean                                     | Should the queue be cleared before adding?

#### `append`
If using `"append": index`, you must also specific the index like so:
```json
"append":{"index":0},
```
compared to `front` and `back`:
```json
"append":"front",
```
```json
"append":"back",
```

#### Outputs

`null` if everything went okay.

#### Example Request 1
```bash
# Add to back of the queue.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_song","params":{"key":"123","append":"back","clear":false}'
```

#### Example Request 2
```bash
# Append at index 4, start from `Song` 3 (offset 2).
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_song","params":{"key":"123","append":{"index":4},"clear":false}'
```

#### Example Request 3
```bash
# Clear the queue, add starting from `Song` 5 (offset 4).
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_song","params":{"key":"123","append":"front","clear":true}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```