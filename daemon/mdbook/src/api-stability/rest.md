# REST
[`REST`](/rest/rest.md)-specific API stability notes.

### ZIP
1. For `REST` endpoints that serve large collections of files, the `ZIP` format will be used
2. There is no compression applied to the files, they are stored as-is
3. These ZIPs can be found the in [`Cache`](/disk.md) folder with the exact same name as sent to the client

These points are all [`🟢 Stable`](/api-stability/marker.md).

### ZIP `/`
ZIP files cannot have internal filenames with `/`, as that is the legal PATH separator, so all:
- File names
- `Playlist` names
- `Artist` names
- `Album` titles
- `Song` titles

inside ZIPs that contain `/` will always be replaced with a single dash with no spaces: `-`

This also applies to the [`filename_separator`](/config.md), it cannot include `/`.

Example:
```bash
Artist/With/Slash.zip # This is fine, the actual ZIP filename will be quoted.
  │
  ├─Artist/With/Slash # This will be turned into `Artist-With-Slash`
      │
      ├─ Album With//Slashes # This will be turned into `Album With--Slashes`
           │
           ├─ Song With/A Slash # This will be turned into `Song With-A Slash`
```

This is [`🟢 Stable`](/api-stability/marker.md).

### Duplicate Filenames
When packaging `Song`'s into ZIPs, if there a multiple `Song`'s with the same name in the same `Album`, it will be suffixed with an incrementing number in parentheses starting from `1`.

For example:
```plaintext
My Song.flac
My Song (1).flac
My Song (2).flac
My Song (3).flac
```

This is [`🟢 Stable`](/api-stability/marker.md).

### File Format & Names
The file format type and names of those files[^1] in the `REST` API are both [`🟢 Stable`](/api-stability/marker.md).

([^1] The filenames depend on the user-specified [`filename_separator`](/config.md), but the general formatting of them is stable)

The below example uses the default ` - ` as the `filename_separator`.

| `REST` resource | File Format Type                           | Filename Formatting |
|-----------------|--------------------------------------------|---------------------|
| `collection`    | `zip`                                      | `Collection - ${CREATION_UNIX_TIMESTAMP}.zip`
| `playlist`      | `zip`                                      | `Playlist - ${PLAYLIST_NAME}.zip`
| `artist`        | `zip`                                      | `${ARTIST_NAME}.zip`
| `album`         | `zip`                                      | `${ARTIST_NAME} - ${ALBUM_TITLE}.zip`
| `song`          | Original audio format (`flac`, `mp3`, etc) | `${ARTIST_NAME} - ${ALBUM_TITLE} - ${SONG_TITLE}.${AUDIO_FORMAT}`
| `art`           | Original image format (`png`, `jpg`, etc)  | `${ARTIST_NAME} - ${ALBUM_TITLE}.${IMAGE_FORMAT}`

### ZIP File Hierarchy
The file hierarchy within ZIPs, and the internal file/folder names are both [`🟢 Stable`](/api-stability/marker.md).

This below example will be the same for the rest of `festivald v1.x.x`:
```plaintext
# Input
https://localhost:18425/collection

# Output
Collection - ${CREATION_UNIX_TIMESTAMP}.zip

# Extracted
Collection - ${CREATION_UNIX_TIMESTAMP}/
    │
    ├─ ${ARTIST_NAME}/
    │    │
    │    ├─ ${ALBUM_TITLE}/
    │         │
    │         ├─ ${ALBUM_TITLE}.${IMAGE_FORMAT}
    │         ├─ ${SONG_TITLE}.${AUDIO_FORMAT}
    │
    │
    ├─ ${ARTIST_NAME}/
    │    │
    │    ├─ ${ALBUM_TITLE}/
    │         │
    │         ├─ ${ALBUM_TITLE}.${IMAGE_FORMAT}
    │         ├─ ${SONG_TITLE}.${AUDIO_FORMAT}
    │
    │
	[ ... etc ... ]
```
