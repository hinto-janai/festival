# /map/$ARTIST_NAME/$ALBUM_TITLE/$SONG_TITLE

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download this `Song` in the original format, using the [`Artist`](/common-objects/artist.md)'s name, [`Album`](/common-objects/album.md) title, and [`Song`](/common-objects/song.md) title.

#### Input
| Input       | Type   |
|-------------|--------|
| Artist name | string |
| Album title | string |
| Song title  | string |

#### Output
| Output                  | Type       |
|-------------------------|------------|
| Song in original format | audio file |

#### Example Input
```http
http://localhost:18425/map/Artist Name/Album Title/Song Title
```

#### Example Output
```plaintext
Artist Name - Album Title - Song Title.flac
```
