# /art/$ARTIST_NAME

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download all the `Album` art owned by this `Artist`, wrapped in an archive format, using the [`Artist`](/common-objects/artist.md)'s name.

#### Input
| Input       | Type   |
|-------------|--------|
| Artist name | string |

#### Output
Archive of all the art owned by this `Artist`.

#### Example Input
```http
http://localhost:18425/art/Artist Name
```

#### Example Output
File:
```plaintext
Art - Artist Name.zip
```

Extracted:
```plaintext
Art - Artist Name/
    ├─ Album Name 1.jpg
    │─ Album Name 2.png
    ├─ Album Name 3.webp
```
