# REST
The [`no_auth_rest`](/config.md) config option or [`--no-auth-rest`](/command-line/command-line.md) command-line flag will allow specified `REST` resources without authorization, while still requiring authorization for everything else.

If a [REST resource](/rest/rest.md) is listed in these options `festivald` will allow any client to access it, regardless of authorization.

This allows you to have `authorization` enabled across the board, but allow specific `REST` resources for public usage.

## Usage
The specified `REST` resources must be one of these `string`'s:

| `REST` resource | Allows                                       | Example endpoint |
|-----------------|----------------------------------------------|------------------|
| `collection`    | Access to downloading the whole `Collection` | [`/collection`](/rest/collection.md)
| `playlist`      | Access to downloading `Playlist` ZIPs        | [`/playlist`](/rest/playlist.md)
| `artist`        | Access to downloading `Artist` ZIPs          | [`/current/artist`](/rest/current/artist.md), [`/map/artist`](/rest/map/artist.md)
| `album`         | Access to downloading `Album` ZIPs           | [`/current/album`](/rest/current/album.md), [`/map/album`](/rest/map/album.md)
| `song`          | Access to downloading `Song` files           | [`/current/song`](/rest/current/song.md), [`/map/song`](/rest/map/song.md)
| `art`           | Access to downloading `Art` ZIPs & files     | [`/current/art`](/rest/current/art.md), [`/art/artist`](/rest/art/artist.md)

If a specified `REST` resource name is incorrect, `festivald` will not start.

## Example
For example, if the value is `["art", "song"]`, ALL clients will be allowed to use the `art` and `song`-related endpoints, for all other endpoints, they must authenticate.

`festivald.toml`:
```toml
authorization = "user:pass"
no_auth_rest  = ["art", "song"]
```

Unauthorized client:
```bash
# Even though we didn't specify `-u user:pass`,
# `festivald` will let us download some art.
curl https://localhost:18425/rand/art

# And some song files.
curl https://localhost:18425/rand/song

# BUT it will not let us download whole albums.
curl https://localhost:18425/rand/album

# Or playlists.
curl https://localhost:18425/playlist/sorted/Playlist 1
```

Authorized client:
```bash
# This _does_ have authentication,
# so it can do whatever it wants.
curl https://localhost:18425/rand/album -u user:pass
curl https://localhost:18425/playlist/sorted/Playlist 1 -u user:pass
```
