# /current/art

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download the `Album` art of the currently set `Song`.

If no art was found, this will respond with an `HTTP` error.

#### Input
`None`

#### Output
Art in original format.

#### Example Input
```http
http://localhost:18425/current/art
```

#### Example Output
```plaintext
Artist Name - Album Title.jpg
```
