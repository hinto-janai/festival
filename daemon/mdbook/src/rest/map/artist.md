# /map/$ARTIST_NAME

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download an `Artist`, using the [`Artist`](/common-objects/artist.md)'s name.

#### Input
| Input         | Type   |
|---------------|--------|
| `Artist` name | string |

#### Output
`ZIP` of all artist's albums (including art if found).

#### Example Input
```http
http://localhost:18425/map/Artist Name
```

#### Example Output
File:
```plaintext
Artist Name.zip
```

Extracted:
```plaintext
Artist Name/
    ├─ Album Title 1/
    │    ├─ Album Title 1.jpg
    │    ├─ Song Title 1.mp3
    │    ├─ Song Title 2.flac
    │    ├─ Song Title 3.ogg
    │
    │─ Album Title 2/
    │    ├─ Album Title 2.png
    │    ├─ Song Title 4.mp3
    │    ├─ Song Title 5.flac
    │    ├─ Song Title 6.ogg
    │
    ├─ Album Title 3/
         ├─ Song Title 7.mp3
         ├─ Song Title 8.flac
         ├─ Song Title 9.ogg
```
