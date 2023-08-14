# /playlist/unsorted/$PLAYLIST_NAME

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download all the `Song`'s (valid entries only) from a `Playlist`. [`Invalid Entry`](/common-objects/playlist.md)'s will be ignored.

`Song` files will not be numbered in any way.

#### Input
| Input         | Type   |
|---------------|--------|
| Playlist name | string |

#### Output
Playlist (valid `Song` files) in archive.

#### Example Input
```http
http://localhost:18425/playlist/unsorted/My Playlist 1
```

#### Example Output
File:
```plaintext
Playlist - My Playlist 1.zip
```

Extracted:
```plaintext
Playlist - My Playlist 1/
  ├─ Song Name.mp3
  ├─ Song Name.flac
  ├─ Song Name.ogg
```
