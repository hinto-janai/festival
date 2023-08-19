# /playlist/$PLAYLIST_NAME

#### 🟢 Stable
This API is [stable](../api-stability/marker.md) since `festivald v1.0.0`.

---

Download all the [`Song`](../common-objects/song.md)'s in a [`Playlist`](../common-objects/playlist.md).

[`Invalid Entry`](../common-objects/playlist.html#playlist-entry)'s will be ignored.

Each `Song` file will be prefixed with its index in the playlist.

The formatting is as such:
```bash
${INDEX}${SEPARATOR}${ARTIST_NAME}${SEPARATOR}${ALBUM_TITLE}${SEPARATOR}${SONG_TITLE}.${SONG_EXTENSION}
```
For example, if the [`filename_separator`](../config.md) was left as the default ` - `:
```plaintext
11 - Artist Name - Album Title - Song Title.mp3
```

Index starts at `0`.

#### Input
| Input           | Type   |
|-----------------|--------|
| `Playlist` name | string |

#### Output
Playlist (valid `Song` files only) in `ZIP`.

#### Example Input
```http
http://localhost:18425/playlist/My Playlist 2
```

#### Example Output
File:
```plaintext
Playlist - My Playlist 2.zip
```

Extracted:
```plaintext
Playlist - My Playlist 2/
  ├─ 0 - Artist Name - Album Title - Song Name.mp3
  ├─ 1 - Artist Name - Album Title - Song Name.flac
  ├─ 2 - Artist Name - Album Title - Song Name.ogg
```
