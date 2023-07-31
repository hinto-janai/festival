# /key/song/$SONG_KEY
Download this `Song` in the original format, using a [`Song key`](../../common-objects/key.md).

#### Input
| Input    | Type             |
|----------|------------------|
| Song key | unsigned integer |

#### Output
Song in original format.

#### Example Input
```http
http://localhost:18425/key/song/123
```

#### Example Output
```plaintext
Artist Name - Album Title - Song Title.flac
```
