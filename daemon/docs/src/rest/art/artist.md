# /art/$ARTIST_NAME
Download all the `Album` art owned by this `Artist`, wrapped in an archive format, using the [`Artist`](../../common-objects/artist.md)'s name.

#### Input
| Input       | Type   |
|-------------|--------|
| Artist name | string |

#### Output
Archive of all the art owned by this `Artist`.

#### Example Input
```http
http://localhost:18425/art/Artist Name
```

#### Example Output
File:
```plaintext
Artist Name - Art.zip
```

Extracted:
```plaintext
Artist Name - Art/
    ├─ Album Name 1.jpg
    │─ Album Name 2.png
    ├─ Album Name 3.webp
```
