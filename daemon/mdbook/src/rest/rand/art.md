# /rand/art

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download a random `Album` art.

If no art was found, the response will be an `HTTP` error.

#### Input
`None`

#### Output
Art in original format.

#### Example Input
```http
http://localhost:18425/rand/art
```

#### Example Output
```plaintext
Artist Name - Album Title.jpg
```
