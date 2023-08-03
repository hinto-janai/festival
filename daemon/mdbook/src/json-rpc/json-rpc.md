# JSON-RPC
`festivald` exposes a [`JSON-RPC 2.0`](https://www.jsonrpc.org/specification) API for general state retrieval & signal control.

It can be accessed by sending a POST HTTP request containing a `JSON-RPC 2.0` request in the body, to the root endpoint, `/`.

For a quick start on using it, see the next section: [Quick Start](json-rpc/quick-start.md).

### Missing resource
If a `JSON-RPC` method is interacting with an underlying resource and that resource is missing from the filesystem, `festivald` will _not_ respond to the client with an error, however, it will log an error message on the machine it is running on.

For example, if a [`add_queue_key_song`](json-rpc/playback-control/add_queue_key_song.md) method is sent, and _that_ `Song`'s underlying PATH is missing/moved/renamed from when the `Collection` was created:
```bash
mv "Artist/Album/Song Title" "Artist/Album/Song_Title"
```

`festivald` will now have a reference to a non-existent PATH and will not be able to find the file, so it will log an error that looks something like:
```plaintext
Audio - PATH error: No such file or directory (os error 2) ... /path/to/unknown/song.mp3
```

You can re-create the `Collection` with [`collection_new`](json-rpc/collection/collection_new.md) to re-link these PATHs.

### Example `JSON-RPC 2.0` _request_:
```json
{
  "jsonrpc": "2.0",   // JSON-RPC version. MUST be exactly "2.0"
  "method": "method", // A string of the method name
  "param": null,      // Optional parameters needed by the method
  "id": 0,            // An ID, MUST be a String, Number, or NULL value if included
}
```

### Example Shell script for sending a request:
```bash
IP=localhost             # ip of festivald
PORT=18425               # port of festivald
METHOD='previous'        # the method to call
PARAMS='{"threshold":3}' # the parameters of the method
ID=0                     # the ID of this request

# Send JSON-RPC request to goto the previous song
# (or reset the current, if more than 3 seconds has passed).
curl \
    http://$IP:$PORT \
    -d '{"jsonrpc":"2.0","id":$ID,"method":"'$METHOD'","params":'$PARAMS'}'
```

### Example `JSON-RPC 2.0` SUCCESSFUL _response_:
```json
{
  "jsonrpc": "2.0", // JSON-RPC version. Will always be exactly "2.0"
  "result": {       // The field containing the result of the SUCCESSFUL response
    // This can contain fields that
    // are nested arbitrarily deep.
    // Although, most times they
    // will be simple "key": value
    // pairs.
  },
  "id": 0, // The ID associated with the client
}
```

### Example `JSON-RPC 2.0` FAILED _response_:
```json
{
  "jsonrpc": "2.0", // JSON-RPC version. Will always be exactly "2.0"
  "error": {        // The field containing the result of the FAILED response
    "code": -32601, // A number that indicates the error type that occurred
    "message": "",  // A string providing a short description of the error
    "data": null,   // An OPTIONAL field containing extra data about the error
  },
  "id": 0, // The ID associated with the client
}
```

### Parameters
For methods without parameters, the field can be omitted:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"toggle"}'
```

All method documentation will include what inputs it needs, what output to expect, and examples.

All method names, parameter names, and field names are in `lower_case_snake_case`.

The title of the section itself is the method name, for example, [`collection_new`](json-rpc/collection/collection_new.md) _is_ the method name.
