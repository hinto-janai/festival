# state_reset
Retrieve the current state of a [`Collection`](../../common-objects/collection.md) reset.

#### Inputs

`None`

#### Outputs

| Field     | Type    | Description |
|-----------|---------|-------------|
| resetting | boolean | Whether `festivald` is currently in the process of resetting the `Collection`
| saving    | boolean | Whether `festivald` is currently saving a newly created `Collection` to disk

#### Example Request
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_reset"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "resetting": false,
    "saving": true
  },
  "id": 0
}
```
