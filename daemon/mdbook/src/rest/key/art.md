# /key/art/$ALBUM_KEY

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download this `Album`'s art in the image's original format, using an [`Album key`/common-objects/key.md).

#### Input
| Input     | Type             |
|-----------|------------------|
| Album key | unsigned integer |

#### Output
Art in original format.

#### Example Input
```http
http://localhost:18425/key/art/123
```

#### Example Output
File:
```plaintext
Artist Name - Album Title.jpg
```
