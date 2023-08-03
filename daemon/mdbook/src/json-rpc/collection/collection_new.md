# collection_new
Create a new [`Collection`](../../common-objects/collection.md) (and replace the current one).

`festivald` will respond with some stats when the `Collection` reset has finished.

The output is a superset of the [`collection_brief`](collection_brief.md) method.

#### Inputs

| Field | Type                                 | Description |
|-------|--------------------------------------|-------------|
| paths | optional (maybe null) array of PATHs | An array of filesystem PATHs to scan for the new `Collection`. These must be absolute PATHs **on the system `festivald` is running on**, not PATHs on the client. If `null` is provided, the default `Music` directory will be used.

#### Outputs

| Field        | Type             | Description |
|--------------|------------------|-------------|
| time         | float            | How many seconds the reset took
| empty        | boolean          | If the `Collection` does NOT have any `Artist`'s, `Album`'s, or `Song`'s
| timestamp    | unsigned integer | The UNIX timestamp of when this `Collection` was created
| count_artist | unsigned integer | How many unique `Artist`'s there are in this `Collection`
| count_album  | unsigned integer | How many unique `Album`'s there are in this `Collection`
| count_song   | unsigned integer | How many unique `Song`'s there are in this `Collection`
| count_art    | unsigned integer | How much unique `Album` art there are in this `Collection`

#### Example Request
```bash
# Use default Music PATH.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":null}}'

# Use the PATH `/home/user/Music/collection` on `festivald`'s filesystem.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":["/home/user/Music/collection"]}}'

# Windows PATH works too if `\` is escaped (and if `festivald` is running on Windows).
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":["C:\\Users\\User\\Music\\collection"]}}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "time": 0.462621988,
    "empty": false,
    "timestamp": 1690579397,
    "count_artist": 195,
    "count_album": 825,
    "count_song": 8543,
    "count_art": 824
  },
  "id": 0
}
```
