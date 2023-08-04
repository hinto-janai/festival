# /key/art/$ALBUM_KEY
Download this `Album`'s art in the image's original format, using an [`Album key`](../../common-objects/key.md).

#### Input
| Input     | Type             |
|-----------|------------------|
| Album key | unsigned integer |

#### Output
Art in original format.

#### Example Input
```http
http://localhost:18425/key/art/123
```

#### Example Output
File:
```plaintext
Artist Name - Album Title.jpg
```
