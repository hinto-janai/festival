# /key/album/$ALBUM_KEY

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download an `Album` using an [`Album key`](/common-objects/key.md).

#### Input
| Input     | Type             |
|-----------|------------------|
| Album key | unsigned integer |

#### Output
Album in `ZIP` (including art if found).

#### Example Input
```http
http://localhost:18425/key/album/123
```

#### Example Output
File:
```plaintext
Artist Name - Album Title.zip
```

Extracted:
```plaintext
Artist Name - Album Title/
  ├─ Album Title.jpg
  ├─ Song Title 1.mp3
  ├─ Song Title 2.flac
  ├─ Song Title 3.ogg
```
