# /map/$ARTIST_NAME/$ALBUM_TITLE
Download an `Album` (including art if found), wrapped in an archive format, using the [`Artist`](../../common-objects/artist.md)'s name, and [`Album`](../../common-objects/album.md) title.
#### Input
| Input       | Type   |
|-------------|--------|
| Artist name | string |
| Album title | string |

#### Output
Album in archive (including art if found).

#### Example Input
```http
http://localhost:18425/map/Artist Name/Album Title
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
