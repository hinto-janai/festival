# queue_add_key_song

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Add a [`Song`](/common-objects/song.md) to the queue with a [`Song` key](/common-objects/key.md).

#### Inputs

| Field  | Type                                        | Description |
|--------|---------------------------------------------|-------------|
| key    | `Song` key (unsigned integer)               | See [`Key`](/common-objects/key.md)
| append | `string`, one of `front`, `back` or `index` | See [`Queue/Append`](/json-rpc/queue/queue.md#append)
| clear  | optional (maybe-null) boolean               | Should the queue be cleared before adding? `null` or no field at all is equal to `false`.
| play   | optional (maybe-null) boolean               | Should we start playing? `null` or no field at all is equal to `false`.
| index  | optional (maybe-null) unsigned integer      | If the `index` append is chosen, this will be the index used

#### Outputs
`result: null` if everything went ok.

`error: ...` if there was an index/offset error.

#### Example Request 1
Add to back of the queue.
```bash
festival-cli queue_add_key_song --key 123 --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_key_song","params":{"key":123,"append":"back"}'
```

#### Example Request 2
Insert at queue index 4.
```bash
festival-cli queue_add_key_song --key 123 --append index --index 4
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_key_song","params":{"key":123,"append":"index","index":4}'
```

#### Example Request 3
Clear the queue, add `Song` 123.
```bash
festival-cli queue_add_key_song --key 123 --append front --clear
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_key_song","params":{"key":123,"append":"front","clear":true}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": null, // <--- everything went ok.
  "id": 0
}
```
