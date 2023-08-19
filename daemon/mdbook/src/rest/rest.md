# REST
`festivald` (by default) exposes `REST` endpoints for accessing the underlying resources associated with the [`Collection`](../common-objects/collection.md) via `HTTP GET` requests.

A simple way to access these files is via a browser, e.g, opening this link:

```http
http://localhost:18425/art/Artist Name/Album Title
```

This will make the art of the album `Album Title`, owned by the artist `Artist Name` open directly in the browser (or make it download the image, if the [`direct_download`](../config.md) configuration setting is enabled).

Opening something like:

```http
http://localhost:18425/map/Artist Name/Album Title/Song Title
```

will directly open that song in the browser and show a simple player, if your browser supports it (all modern browsers do). Again, you can change the behavior so that browsers directly download these resources by changing the `direct_download` [config](../config.md) option.

If a file downloaded has a nested filename, the `filename_separator` [config](../config.md) option will control what the separator will be. By default, this is ` - `, so the filename of an archive of an `Artist` will look like:

```plaintext
Artist Name - Album Title.zip
```

### Resource
The `REST` endpoint resources are in loosely grouped based on what type of data they are and what format they are outputted as.

These "groups" are referenced in [`authorization`](../authorization/rest.md) & [`API Stability`](../api-stability/rest.md).

| `REST` resource | File Format Type                           | Description             | Example Endpoint                           | Filename Formatting |
|-----------------|--------------------------------------------|-------------------------|--------------------------------------------|---------------------|
| `collection`    | `zip`                                      | The whole `Collection`  | [`/collection`](collection.md)       | `Collection - ${CREATION_UNIX_TIMESTAMP}.zip`
| `playlist`      | `zip`                                      | Individual `Playlist`'s | [`/playlist`](playlist.md)           | `Playlist - ${PLAYLIST_NAME}.zip`
| `artist`        | `zip`                                      | Individual `Artist`'s   | [`/map/artist`](map/artist.md)       | `${ARTIST_NAME}.zip`
| `album`         | `zip`                                      | Individual `Album`'s    | [`/current/album`](current/album.md) | `${ARTIST_NAME} - ${ALBUM_TITLE}.zip`
| `song`          | Original audio format (`flac`, `mp3`, etc) | Individual `Song`'s     | [`/rand/song`](rand/song.md)         | `${ARTIST_NAME} - ${ALBUM_TITLE} - ${SONG_TITLE}.${AUDIO_FORMAT}`
| `art`           | Original image format (`png`, `jpg`, etc)  | Individual `Album` art  | [`/current/art`](current/art.md)     | `${ARTIST_NAME} - ${ALBUM_TITLE}.${IMAGE_FORMAT}`

### Missing Resource
If the underlying file for a resource is missing from the filesystem, `festivald` will respond to `REST` requests with an `HTTP` error.

For example, if an `Artist` is requested:
```http
http://localhost:18425/map/Artist
```
And _any_ underlying PATH is missing/moved/renamed from when the `Collection` was created:
```bash
mv "Song Title" "Song_Title"
```
`festivald` will now have a reference to a non-existent PATH and will not be able to find the file(s), so it will respond with something along the lines of:
```plaintext
Song file error: ${ARTIST_NAME} - ${ALBUM_TITLE} - Song Title.flac
```

You can re-create the `Collection` with [`collection_new`](../json-rpc/collection/collection_new.md) to re-link these PATHs.

### Disk Cache
`festivald` exclusively uses the [`Disk`](../disk.md) when operating with potentially heavy data.

All heavy files are streamed from disk, so none of them take up full space in physical memory.

Practically, what this means is that when serving files, `festivald` can get by with very little memory, although the [`Cache`](../disk.md) folder may get quite heavy.

The cache behavior can be configured in the [`config`](../config.md) or via [`command-line`](../command-line/command-line.md) options.
