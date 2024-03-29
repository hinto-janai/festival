# /current/album

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Download the `Album` of the currently set `Song` (including art if found).

#### Input
`None`

#### Output
Album in archive (including art if found).

#### Example Input
```http
http://localhost:18425/current/album
```

#### Example Output
File:
```plaintext
Artist Name - Album Title.zip
```

Extracted:
```plaintext
Artist Name - Album Title/
  ├─ Album Name.jpg
  ├─ Song Name 1.mp3
  ├─ Song Name 2.flac
  ├─ Song Name 3.ogg
```
