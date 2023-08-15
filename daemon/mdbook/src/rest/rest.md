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

## ZIP
For `REST` endpoints that serve large collections of files, the `ZIP` format will be used.

There is no compression applied to the files, they are stored as-is.

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
