# /art/$ARTIST_NAME/$ALBUM_TITLE
Download an `Album`'s art, using the [`Artist`](../../common-objects/artist.md)'s name, and [`Album`](../../common-objects/album.md) title

#### Input
| Input       | Type   |
|-------------|--------|
| Artist name | string |
| Album title | string |

#### Output
Album art in original format.

#### Example Input
```http
http://localhost:18425/art/Artist Name/Album Title
```

#### Example Output
```plaintext
Art - Artist Name - Album Title.png
```
