# `festivald`
`Festival` daemon.

`festivald` is a music server that plays on the device it is running on, while allowing remote control via clients.

The 2 APIs `festivald` exposes:
- [`JSON-RPC 2.0`](https://jsonrpc.org) for state retrieval & signal control
- [`REST`](https://en.wikipedia.org/wiki/Representational_state_transfer) endpoints for serving large resources (audio, art, etc)

The transport used is `HTTP(s)`.

To interact with `festivald`, you need a client. The reference client can be found at [`festival-cli`](https://github.com/hinto-janai/festival/tree/main/cli).

For the `JSON-RPC` API, anything that can transmit `JSON-RPC` over `HTTP(s)` can be a client, like `curl`:
```bash
# Toggle playback.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"toggle"}'
```

For the `REST` API, you could use anything that can handle `HTTP(s)`, like a browser:
```bash
# Opening this link in a browser will show a small player for this song.
http://localhost:18425/string/Artist Name/Artist Title/Song Title
```

# Contents
* [Quick Start](#Quick-Start)
* [Configuration](#Configuration)
* [Authorization](#Authorization)
* [Disk](#Disk)
* [Command Line](#Command-Line)
* [REST](#REST)
	- [/key](#key)
		- [/artist/${artist_key}](#artistartist_key)
		- [/album/${album_key}](#albumalbum_key)
		- [/song/${song_key}](#songsong_key)
		- [/art/${album_key}](#artalbum_key)
	- [/string](#string)
		- [/${artist_name}](#artist_name)
		- [/${artist_name}/${album_title}](#artist_namealbum_title)
		- [/${artist_name}/${album_title}/${song_title}](#artist_namealbum_titlesong_title)
	- [/art/${artist_name}/${album_title}](#artartist_namealbum_title)
* [JSON-RPC](#JSON-RPC)
	- [State Retrieval](#State-Retrieval)
		- [state_daemon](#state_daemon)
		- [state_audio](#state_audio)
		- [state_reset](#state_reset)
		- [state_collection](#state_collection)
	- [Playback Control](#Playback-Control)
		- [toggle](#toggle)
		- [play](#play)
		- [pause](#pause)
		- [next](#next)
		- [stop](#stop)
		- [repeat_off](#repeat_off)
		- [repeat_song](#repeat_song)
		- [repeat_queue](#repeat_queue)
		- [shuffle](#shuffle)
		- [previous](#previous)
		- [volume](#volume)
		- [add_queue_song](#add_queue_song)
		- [add_queue_album](#add_queue_album)
		- [add_queue_artist](#add_queue_artist)
		- [clear](#clear)
		- [seek](#seek)
		- [skip](#skip)
		- [back](#back)
		- [set_queue_index](#set_queue_index)
		- [remove_queue_range](#remove_queue_range)
	- [Key](#Key)
		- [key_artist](#key_artist)
		- [key_album](#key_album)
		- [key_song](#key_song)
	- [Map](#Map)
		- [map_artist](#map_artist)
		- [map_album](#map_album)
		- [map_song](#map_song)
	- [Search](#Search)
		- [search](#search)
		- [search_artist](#search_artist)
		- [search_album](#search_album)
		- [search_song](#search_song)
	- [Collection](#Collection)
		- [new_collection](#new_collection)

# Quick Start

# Configuration

# Authorization

# Disk

# Command Line

# REST
`festivald` (by default) exposes REST endpoints for `Collection`-related resources that can be accessed via GET HTTP requests.

A simple way to access these files is via a browser, e.g, opening this link:
```
http://localhost:18425/art/my_artist/my_album
```
This will make the art of the album `my_album`, owned by the artist `my_artist` open directly in the browser (or make it download the image, if the `direct_download` configuration setting is enabled).

Opening something like:
```
http://localhost:18425/string/Artist Name/Album Title/Song Title
```
will directly open that song in the browser and show a simple player, if your browser supports it (all modern browsers do). Again, you can change the behavior so that browsers directly download these resources by changing the `direct_download` configuration option.

If a file is downloaded that is nested, the `filename_separator` config option will control what the separator will be. By default, this is ` - `, so the filename of an archive of an artist will look like:
```
Artist Name - Album Title.zip
```

# /key
Access audio files and/or art via a `key`.

This endpoint expects 2 more endpoints:
- `${object}`
- `${key}`

The `object` must be one of:
- `artist`
- `album`
- `song`
- `art`

The `key` must be the key number associated with the object.

This info can be found in multiple JSON-RPC methods, such as `map_artist`, `search_artist`, etc. A `key` field will be included in the response. It is a number that represents that object.

Keys are unique _per_ object group, meaning there is an `artist 0` AND `album 0`, and so forth.

Note that keys can only be relied upon as long as the `Collection` has not been reset. When the `Collection` is reset, it is not guaranteed that the same key will map to the same object. Using the `/string` endpoint may be more convenient so that artist names, album and song titles can be used as inputs instead.

The main reasons `/key` exists:
- Accessing objects via `/key` is faster than with `/string`
- As long as your `Collection` is stable, the `key`'s are stable

## /artist/${artist_key}
Download all the `Album`'s owned by this `Artist`, 1 directory per album (including art if found), wrapped in an archive format.

| Input      | Type             | Example |
|------------|------------------|---------|
| artist key | unsigned integer | `http://localhost:18425/key/artist/123`

| Output                                                  | Type   | Example |
|---------------------------------------------------------|--------|---------|
| Archive of all artist's albums (including art if found) | `.zip` | `Artist Name.zip`

## /album/${album_key}
Download this `Album` (including art if found), wrapped in an archive format.

| Input     | Type             | Example |
|-----------|------------------|---------|
| album key | unsigned integer | `http://localhost:18425/key/album/123`

| Output                                    | Type   | Example |
|-------------------------------------------|--------|---------|
| Album in archive (including art if found) | `.zip` | `Artist Name - Album Title.zip`

## /song/${song_key}
Download this `Song` in the original format.

| Input    | Type             | Example |
|----------|------------------|---------|
| song key | unsigned integer | `http://localhost:18425/key/song/123`

| Output                  | Type       | Example |
|-------------------------|------------|---------|
| Song in original format | audio file | `Artist Name - Album Title - Song Title.flac`

## /art/${album_key}
Download this `Album`'s art in the image's original format.

| Input     | Type             | Example |
|-----------|------------------|---------|
| album key | unsigned integer | `http://localhost:18425/key/art/123`

| Output                 | Type       | Example |
|------------------------|------------|---------|
| Art in original format | image file | `Artist Name - Album Title.jpg`

# /string
This is the same as the `/key` endpoint, but instead of numbers, you can directly use:
- Artist names
- Album titles
- Song titles

So instead of:
```
http://localhost:18425/key/song/123
```
you can use:
```
http://localhost:18425/string/Artist Name/Artist Title/Song Title
```
Browsers will secretly percent-encode this URL, so it'll actually be:
```
http://localhost:18425/string/Artist%20Name/Artist%20Title/Song%20Title
```
This is fine, `festivald` will decode it, along with any other percent encoding, so you can use spaces or any other UTF-8 characters directly in the URL:
```
http://localhost:18425/string/артист/❤️/ヒント じゃない
```

The reason `Artist` names and `Album` titles have to be specified is to prevent collisions.

If there's 2 songs in your `Collection` called: `Hello World`, which one should `festivald` return?

Since `Artist` names are unique, and `Album` titles within `Artist`'s are unique, they serve as an identifier.

Also note: words are case-sensitive and must be exact.

If you have an `Album` called `Hello World`, none of these inputs will work:
- `Hello world`
- `hello World`
- `HELlo World`
- `HelloWorld`
- `H3ll0 W0rld`

The input must _exactly_ be `Hello World`.

## /${artist_name}
Download all the `Album`'s owned by this `Artist`, 1 directory per album (including art if found), wrapped in an archive format.

| Input       | Type         | Example |
|-------------|--------------|---------|
| artist name | UTF-8 string | `http://localhost:18425/string/Artist Name`

| Output                                                  | Type   | Example |
|---------------------------------------------------------|--------|---------|
| Archive of all artist's albums (including art if found) | `.zip` | `Artist Name.zip`

## /${artist_name}/${album_title}
Download this `Album` (including art if found), wrapped in an archive format.

| Input                    | Type         | Example |
|--------------------------|--------------|---------|
| artist name, album title | UTF-8 string | `http://localhost:18425/string/Artist Name/Album Title`

| Output                                    | Type   | Example |
|-------------------------------------------|--------|---------|
| Album in archive (including art if found) | `.zip` | `Artist Name - Album Title.zip`

## /${artist_name}/${album_title}/${song_title}
Download this `Song` in the original format.

| Input                                | Type         | Example |
|--------------------------------------|--------------|---------|
| artist name, album title, song title | UTF-8 string | `http://localhost:18425/string/Artist Name/Album Title/Song Title`

| Output                  | Type       | Example |
|-------------------------|------------|---------|
| Song in original format | audio file | `Artist Name - Album Title - Song Title.flac`

## /art/${artist_name}/${album_title}
This single `/art` endpoint exists to allow downloading an `Album`'s art individually via a `string` key.

Download this `Album`'s art in the original format.

If no art was found, the response will be a 404 error.

| Input                    | Type         | Example |
|--------------------------|--------------|---------|
| artist name, album title | UTF-8 string | `http://localhost:18425/art/Artist Name/Album Title`

| Output                 | Type       | Example |
|------------------------|------------|---------|
| Art in original format | image file | `Artist Name - Album Title.jpg`

# JSON-RPC
`festivald` exposes a [`JSON-RPC 2.0`](https://jsonrpc.org) API for general state retrieval & signal control.

It can be accessed by sending a POST HTTP request containing a `JSON-RPC 2.0` request in the body, to the root endpoint, `/`.

A quick recap of a `JSON-RPC 2.0` _request_:
```json
{
  "jsonrpc": "2.0",   // JSON-RPC version. MUST be exactly "2.0"
  "method": "METHOD", // A string of the method name
  "param": [],        // Optional parameters needed by the method
  "id": 0,            // An ID, MUST be a String, Number, or NULL value if included
}
```
An example:
```bash
IP=localhost             # ip of festivald
PORT=18425               # port of festivald
METHOD='previous'        # the method to call
PARAMS='{"threshold":3}' # the parameters of the method
ID=0                     # the ID of this request

# Send JSON-RPC request to goto the previous song
# (or reset the current, if more than 3 seconds has passed).
curl \
    http://$IP:$PORT \
    -d '{"jsonrpc":"2.0","id":$ID,"method":"'$METHOD'","params":'$PARAMS'}'
```

A quick recap of a SUCCESSFUL `JSON-RPC 2.0` _response_:
```json
{
  "jsonrpc": "2.0", // JSON-RPC version. Will always be exactly "2.0"
  "result": {       // The field containing the result of the SUCCESSFUL response
    // This can contain fields that
    // are nested arbitrarily deep.
    // Although, most times they
    // will be simple "key": value
    // pairs.
  },
  "id": 0, // The ID associated with the client
}
```
A quick recap of a FAILED `JSON-RPC 2.0` _response_:
```json
{
  "jsonrpc": "2.0", // JSON-RPC version. Will always be exactly "2.0"
  "error": {        // The field containing the result of the FAILED response
    "code": -32601, // A number that indicates the error type that occurred
    "message": "",  // A string providing a short description of the error
    "data": null,   // An OPTIONAL field containing extra data about the error
  },
  "id": 0,          // The ID associated with the client
}
```

For methods without parameters, the field can be omitted:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"toggle"}'
```

Each method below will document what inputs it needs, what output to expect, and examples.

The title of the section itself is the method name, for example below, `state_daemon` _is_ the method name.

# State Retrieval
These methods are for retrieving state, and do not mutate any part of the system.

## state_daemon
Retrieve state about the status of `festivald` itself.

| Inputs |
|--------|
| None   |

| Outputs         | Type             | Description |
|-----------------|------------------|-------------|
| uptime          | unsigned integer | Uptime of `festivald` in seconds
| rest            | boolean          | If this `festivald`'s `REST` API is enabled
| direct_download | boolean          | If this `festivald`'s `REST` API has `direct_download` enabled
| authorization   | boolean          | If this `festivald` has authorization enabled
| version         | string           | Semantic version of this `festivald`
| commit          | string           | Git commit of this `festivald`
| os              | string           | The OS this `festivald` was built for

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_daemon"}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "uptime": 15,
    "rest": true,
    "direct_download": false,
    "authorization": false,
    "version": "festivald v0.0.0",
    "commit": "5e54b8ecd6fd505ff8c9ef1a5fbbef26e7f1bd86",
    "os": "Linux x64"
  },
  "id": 0
}
```

## state_audio
## state_reset
## state_collection

# Playback Control
## toggle
## play
## pause
## next
## stop
## repeat_off
## repeat_song
## repeat_queue
## shuffle
## previous
## volume
## add_queue_song
## add_queue_album
## add_queue_artist
## clear
## seek
## skip
## back
## set_queue_index
## remove_queue_range

# Key
## key_artist
## key_album
## key_song

# Map
## map_artist
## map_album
## map_song

# Search
## search
## search_artist
## search_album
## search_song

# Collection
## new_collection
