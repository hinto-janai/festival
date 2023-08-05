# add_queue_key_song
Add a [`Song`](../../common-objects/song.md) to the queue with a `Song` [key](../../common-objects/key.md).

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| key    | `Song` key (unsigned integer  )             | See [`Key`](key.md)
| append | `string`, one of `front`, `back` or `index` | In which way should we add to the queue? `front` means to the front of the queue. `back` means to the back. `index` means at an exact queue index. Queue index starts at `0`, so to mimic `front`, you would provide `0`.
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used
| clear  | boolean                                     | Should the queue be cleared before adding?

#### Outputs

`null` if everything went okay.

#### Example Request 1
```bash
# Add to back of the queue.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_song","params":{"key":123,"append":"back","clear":false}'
```

#### Example Request 2
```bash
# Append at queue index 4.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_song","params":{"key":123,"append":"index","index":4,"clear":false}'
```

#### Example Request 3
```bash
# Clear the queue, add `Song` 123.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_key_song","params":{"key":123,"append":"front","clear":true}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
