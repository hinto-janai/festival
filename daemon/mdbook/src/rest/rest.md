# REST
`festivald` (by default) exposes `REST` endpoints for [`Collection`](/common-objects/collection.md)-related resources that can be accessed via GET HTTP requests.

A simple way to access these files is via a browser, e.g, opening this link:

```http
http://localhost:18425/art/Artist Name/Album Title
```

This will make the art of the album `Album Title`, owned by the artist `Artist Name` open directly in the browser (or make it download the image, if the `direct_download` configuration setting is enabled).

Opening something like:

```http
http://localhost:18425/map/Artist Name/Album Title/Song Title
```

will directly open that song in the browser and show a simple player, if your browser supports it (all modern browsers do). Again, you can change the behavior so that browsers directly download these resources by changing the `direct_download` [config](config.md) option.

If a file is downloaded that is nested, the `filename_separator` [config](config.md) option will control what the separator will be. By default, this is ` - `, so the filename of an archive of an artist will look like:

```plaintext
Artist Name - Album Title.zip
```

## Resource
The `REST` endpoint resources are in loosely grouped based on what type of data they are and what format they are outputted as.

These "groups" are referenced in [`authorization`](/authorization/rest.md) & [`API Stability`](/api-stability/rest.md).

| `REST` resource | File Format Type                           | Description             | Example Endpoint                           | Filename Formatting |
|-----------------|--------------------------------------------|-------------------------|--------------------------------------------|---------------------|
| `collection`    | `zip`                                      | The whole `Collection`  | [`/collection`](/rest/collection.md)       | `Collection - ${CREATION_UNIX_TIMESTAMP}.zip`
| `playlist`      | `zip`                                      | Individual `Playlist`'s | [`/playlist`](/rest/playlist.md)           | `Playlist - ${PLAYLIST_NAME}.zip`
| `artist`        | `zip`                                      | Individual `Artist`'s   | [`/map/artist`](/rest/map/artist.md)       | `${ARTIST_NAME}.zip`
| `album`         | `zip`                                      | Individual `Album`'s    | [`/current/album`](/rest/current/album.md) | `${ARTIST_NAME} - ${ALBUM_TITLE}.zip`
| `song`          | Original audio format (`flac`, `mp3`, etc) | Individual `Song`'s     | [`/rand/song`](/rest/rand/song.md)         | `${ARTIST_NAME} - ${ALBUM_TITLE} - ${SONG_TITLE}.${AUDIO_FORMAT}`
| `art`           | Original image format (`png`, `jpg`, etc)  | Individual `Album` art  | [`/current/art`](/rest/current/art.md)     | `${ARTIST_NAME} - ${ALBUM_TITLE}.${IMAGE_FORMAT}`

## Missing Resource
If the underlying file for a resource is missing from the filesystem, `festivald` will respond to `REST` requests with an error.

For example, if an `Artist` is requested:
```http
http://localhost:18425/map/Artist
```
And _any_ underlying PATH is missing/moved/renamed from when the `Collection` was created:
```bash
mv "Song Title" "Song_Title"
```
`festivald` will now have a reference to a non-existent PATH and will not be able to find the file(s), so it will respond with:
```plaintext
File not found
```

You can re-create the `Collection` with [`collection_new`](json-rpc/collection/collection_new.md) to re-link these PATHs.

## Duplicate Files
Within an `Album`, if there a multiple `Song`'s with the same name and an additional one is found, it will be suffixed with an incrementing number in parentheses starting from `1`.

For example:
```plaintext
My Song.flac
My Song (1).flac
My Song (2).flac
My Song (3).flac
```

## ZIP
For `REST` endpoints that serve large collections of files, the `ZIP` format will be used.

There is no compression applied to the files, they are stored as-is.

## Disk Cache
`festivald` exclusively uses the [`Disk`](/disk.md) when operating with potentially heavy data.

All heavy files are both streamed and [`memory-mapped`](https://en.wikipedia.org/wiki/Mmap) from disk, so none of them take up full space in physical memory.

Practically, what this means is that when serving files, `festivald` can get by with very little memory, although the [`Cache`](/disk.md) folder may get quite heavy.

The cache behavior can be configured in the [`config`](/config.md) or via [`command-line`](/command-line/command-line.md) options.
