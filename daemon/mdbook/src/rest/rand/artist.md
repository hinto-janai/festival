# /rand/artist

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download all the `Album`'s by a random `Artist`, 1 directory per album (including art if found), wrapped in an archive format.

#### Input
`None`

#### Output
Archive of all artist's albums (including art if found).

#### Example Input
```http
http://localhost:18425/rand/artist
```

#### Example Output
File:
```plaintext
Artist Name.zip
```

Extracted:
```plaintext
Artist Name/
    ├─ Album Name 1/
    │    ├─ Album Name 1.jpg
    │    ├─ Song Name 1.mp3
    │    ├─ Song Name 2.flac
    │    ├─ Song Name 3.ogg
    │
    │─ Album Name 2/
    │    ├─ Album Name 2.png
    │    ├─ Song Name 4.mp3
    │    ├─ Song Name 5.flac
    │    ├─ Song Name 6.ogg
    │
    ├─ Album Name 3/
         ├─ Song Name 7.mp3
         ├─ Song Name 8.flac
         ├─ Song Name 9.ogg
```
