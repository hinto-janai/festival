# /key/artist/$ARTIST_KEY
Download all the `Album`'s owned by this `Artist`, 1 directory per album (including art if found), wrapped in an archive format, using an [`Artist key`](../../common-objects/key.md).

#### Input
| Input      | Type             |
|------------|------------------|
| Artist key | unsigned integer |

#### Output
Archive of all artist's albums (including art if found).

#### Example Input
```http
http://localhost:18425/key/artist/123
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
