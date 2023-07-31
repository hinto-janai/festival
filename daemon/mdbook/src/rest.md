# REST
`festivald` (by default) exposes REST endpoints for `Collection`-related resources that can be accessed via GET HTTP requests.

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
