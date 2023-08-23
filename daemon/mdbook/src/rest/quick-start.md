# `REST` Quick Start
A quick start to using `festivald`'s `REST` API.

### View random [`Album`](../common-objects/album.md) art in a web browser with [`/rand/art`](rand/art.md)
```http
http://localhost:18425/rand/art
```

### Download a random [`Song`](../common-objects/song.md) with [`/rand/song`](rand/song.md)
```http
http://localhost:18425/rand/song
```
### Download a random [`Album`](../common-objects/album.md) in the `ZIP` format with [`/rand/album`](rand/album.md)
```http
http://localhost:18425/rand/album
```

### Download a _specific_ [`Artist`](../common-objects/artist.md) in the `ZIP` format with [`/map/$ARTIST_NAME`](map/artist.md)
```http
http://localhost:18425/map/Artist Name
```

### Download a _specific_ [`Album`](../common-objects/album.md) by _specific_ [`Artist`](../common-objects/artist.md) in the `ZIP` format with [`/map/$ARTIST_NAME`](map/artist.md)
```http
http://localhost:18425/map/Artist Name/Album Title
```

### Download the currently set [`Song`](../common-objects/song.md) with [`/current/song`](current/song.md)
```http
http://localhost:18425/current/song
```

### View the art of the currently set [`Song`](../common-objects/song.md) with [`/current/art`](current/art.md)
```http
http://localhost:18425/current/art
```

### Download _all_ the art belonging to an [`Artist`](../common-objects/artist.md) with [`/art/$ARTIST_NAME`](art/artist.md)
```http
http://localhost:18425/art/Artist Name
```

### Download _all_ the [`Song`](../common-objects/song.md)'s in a [`Playlist`](../common-objects/playlist.md) in the `ZIP` format with [`/playlist/$PLAYLIST_NAME`](playlist.md)
```http
http://localhost:18425/playlist/My Playlist 1
```
