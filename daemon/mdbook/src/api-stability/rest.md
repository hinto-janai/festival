# REST
[`REST`](/rest/rest.md)-specific API stability notes.

## File Format & Names
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

## ZIP File Hierarchy
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
