# /key/album/$ALBUM_KEY
Download an `Album` (including art if found), wrapped in an archive format, using an [`Album key`](../../common-objects/key.md).

#### Input
| Input     | Type             |
|-----------|------------------|
| Album key | unsigned integer |

#### Output
Album in archive (including art if found).

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
  ├─ Album Name.jpg
  ├─ Song Name 1.mp3
  ├─ Song Name 2.flac
  ├─ Song Name 3.ogg
```
