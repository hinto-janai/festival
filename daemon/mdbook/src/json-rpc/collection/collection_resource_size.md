# collection_resource_size

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

View the size of the current [`Collection`](/common-objects/collection.md)'s _underlying_ resources (audio files and art).

#### Inputs
`None`

#### Outputs

| Field | Type             | Description |
|-------|------------------|-------------|
| audio | unsigned integer | Total size of the `Collection`'s underlying audio files in bytes
| art   | unsigned integer | Total size of the `Collection`'s underlying `Album` art in bytes

#### Example Request
```bash
festival-cli collection_resource_size
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_resource_size"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "audio": 315060491209,
    "art": 877030803
  },
  "id": 0
}
```
