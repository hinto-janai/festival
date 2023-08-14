# /collection

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

(However, the [`collection_full`](/json-rpc/collection/collection_full.md) output is `🟡 Incomplete`).

---

Download _the entire_ [`Collection`](../common-objects/collection.md).

## Warning
This is the heaviest `REST` endpoint and will cause `festivald` to use at least the same amount of disk space your `Collection`'s `Song` files take up.

For example, if all your `Song`'s combined are `100GB` in size, `festivald` will use at least `100GB` in disk space when receiving this request. Further same requests will not further disk usage, depending on your [`cache_time`](../config.md) configuration.

`festivald` may also take a while to respond to the client, as it is copying, organizing, and archiving the _entire_ `Collection`.

---

#### Input
`None`

#### Output

Archive including:
- A copy of [`collection_full`](../json-rpc/collection/collection_full.md) output in `JSON`
- All [`Artist`](../common-objects/artist.md)'s
- All [`Album`](../common-objects/album.md)'s (including art if found)
- All [`Song`](../common-objects/song.md)'s

#### Example Input
```http
http://localhost:18425/collection
```

#### Example Output
File:
```plaintext
Collection ${FILENAME_SEPARATOR} ${CREATION_UNIX_TIMESTAMP}.zip
```
E.g:
```plaintext
Collection - 1690812809.zip
```

Extracted:
```plaintext
Collection - 1690812809/
  ├─ collection_full.json
  │
  │
  ├─ Artist Name 1/
  │      ├─ Album Name 1/
  │      │    ├─ Album Name 1.jpg
  │      │    ├─ Song Name 1.mp3
  │      │    ├─ Song Name 2.flac
  │      │    ├─ Song Name 3.ogg
  │      │
  │      │─ Album Name 2/
  │      │    ├─ Album Name 2.png
  │      │    ├─ Song Name 4.mp3
  │      │    ├─ Song Name 5.flac
  │      │    ├─ Song Name 6.ogg
  │      │
  │      ├─ Album Name 3/
  │           ├─ Song Name 7.mp3
  │           ├─ Song Name 8.flac
  │           ├─ Song Name 9.ogg
  │
  │
  ├─ Artist Name 2/
  │      ├─ Album Name 4/
  │      │    ├─ Album Name 4.jpg
  │      │    ├─ Song Name 10.mp3
  │      │    ├─ Song Name 12.flac
  │      │    ├─ Song Name 13.ogg
  │      │
  │      │─ Album Name 5/
  │      │    ├─ Album Name 5.png
  │      │    ├─ Song Name 14.mp3
  │      │    ├─ Song Name 15.flac
  │      │    ├─ Song Name 16.ogg
  │      │
  │      ├─ Album Name 6/
  │           ├─ Song Name 17.mp3
  │           ├─ Song Name 18.flac
  │           ├─ Song Name 19.ogg
  │
  │
  ├─ [... etc ...]
```
