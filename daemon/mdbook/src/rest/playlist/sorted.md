# /playlist/sorted/$PLAYLIST_NAME

#### 🟢 Stable
This API is [stable](/api-stability/marker.md) since `festivald v1.0.0`.

---

Download all the `Song`'s from a `Playlist`. [`Invalid Entry`](/common-objects/playlist.md)'s will be ignored.

Unlike [`/playlist/sorted`](/rest/playlist/playlist/unsorted.md), this endpoint will prefix each song with its index in the playlist.

The formatting is as such:
```plaintext
${INDEX}${FILENAME_SEPARATOR}${SONG_TITLE}.${SONG_EXTENSION}
```
For example, if the filename separator were ` - `:
```plaintext
11 - song_file.mp3
```

Index starts at `0`.

#### Input
| Input         | Type   |
|---------------|--------|
| Playlist name | string |

#### Output
Playlist (valid `Song` files) in archive.

#### Example Input
```http
http://localhost:18425/playlist/sorted/My Playlist 2
```

#### Example Output
File:
```plaintext
Playlist - My Playlist 2.zip
```

Extracted:
```plaintext
Playlist - My Playlist 2/
  ├─ 0 - Song Name.mp3
  ├─ 1 - Song Name.flac
  ├─ 2 - Song Name.ogg
```
