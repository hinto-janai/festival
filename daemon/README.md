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
* [JSON-RPC](#JSON-RPC)
	- [Common Objects](#Common-Objects)
		- [Artist](#Artist)
		- [Album](#Album)
		- [Song](#Song)
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

# Quick Start

# Configuration

# Authorization

# Disk

# Command Line

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

All method names and field names are in `lower_case_snake_case`.

The title of the section itself is the method name, for example, `state_daemon` _is_ the method name.

# Common Objects
These are 3 common "objects" that have a set structure, and appear in many JSON-RPC calls:
- `Artist`
- `Album`
- `Song`

The definitions of these object's key/value pairs will be here, instead of everywhere they appear in the documentation.

## Artist
`Artist` object:
| Field   | Type                                      | Description |
|---------|-------------------------------------------|-------------|
| name    | string                                    | The `Artist`'s name
| key     | `Artist` key (unsigned integer)           | The `Artist` key associated with this `Artist`
| runtime | unsigned integer                          | The total runtime of all songs owned by this `Artist`
| albums  | array of `Album` keys (unsigned integers) | Keys to all `Album`'s owned by this `Artist`, in release order
| songs   | array of `Song` keys (unsigned integers)  | Keys to all `Songs`'s owned by this `Artist`, in `Album` release order, then `Song` track order

Example:
```json
{
  "name": "Artist Name",
  "key": 65,
  "runtime": 7583,
  "albums": [
    255,
    263
  ],
  "songs": [
    2829,
    2832,
    2835,
    2841
  ]
}
```

## Album
`Album` object:
| Field      | Type                                      | Description |
|------------|-------------------------------------------|-------------|
| title      | string                                    | The title of this `Album`
| key        | `Album` key (unsigned integer)            | The `Album` key associated with this `Album`
| artist     | `Artist` key (unsigned integer)           | The `Artist` key of the `Artist` that owns this `Album`
| release    | string                                    | Release date of this `Album` in `YYYY-MM-DD`/`YYYY-MM`/`YYYY` format, `????-??-??` if unknown
| runtime    | unsigned integer                          | The total runtime of this `Album`
| song_count | unsigned integer                          | How many `Song`'s are in this `Album`
| songs      | array of `Song` keys (unsigned integers)  | Keys to all of the `Song`'s in this `Album`, in track order
| discs      | unsigned integer                          | Count of how many "discs" are in this `Album`, most will be `0`
| art        | Optional (maybe-null) unsigned integer    | Size of this `Album`'s art in bytes, `null` if not found
| genre      | Optional (maybe-null) string              | Genre of this `Album`, `null` if not found

Example:
```json
{
  "title": "Album Title",
  "key": 100,
  "artist": 16,
  "release": "2011-07-13",
  "runtime": 2942,
  "song_count": 3,
  "songs": [
    972,
    1024,
    1051,
  ],
  "discs": 0,
  "art": 306410,
  "genre": null
}
```

## Song
`Song` object:
| Field       | Type                                   | Description |
|-------------|----------------------------------------|-------------|
| title       | string                                 | The title of this `Song`
| key         | `Song` key (unsigned integer)          | The `Song` key associated with this `Song`
| album       | `Album` key (unsigned integer)         | The `Album` key of the `Album` this `Song` is from
| runtime     | unsigned integer                       | The total runtime of this `Song`
| sample_rate | unsigned integer                       | The sample rate of this `Song` in hertz, e.g: `44100`
| track       | Optional (maybe-null) unsigned integer | Track number of this `Song`, `null` if not found
| disc        | Optional (maybe-null) unsigned integer | Disc number this `Song` belongs to, `null` if not found

Example:
```json
{
  "title": "Song Title",
  "key": 401,
  "album": 42,
  "runtime": 132,
  "sample_rate": 44100,
  "track": 5,
  "disc": null
}
```

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
    "version": "v0.0.0",
    "commit": "5e54b8ecd6fd505ff8c9ef1a5fbbef26e7f1bd86",
    "os": "Linux x64"
  },
  "id": 0
}
```

## state_audio
## state_reset
## state_collection
Retrieve some brief metadata about the current `Collection`.

This method is a subset of `state_collection_full`.

| Inputs |
|--------|
| None   |

| Outputs      | Type             | Description |
|--------------|------------------|-------------|
| empty        | boolean          | If the `Collection` does NOT have any `Artist`'s, `Album`'s, or `Song`'s
| timestamp    | unsigned integer | The UNIX timestamp of when this `Collection` was created
| count_artist | unsigned integer | How many unique `Artist`'s there are in this `Collection`
| count_album  | unsigned integer | How many unique `Album`'s there are in this `Collection`
| count_song   | unsigned integer | How many unique `Song`'s there are in this `Collection`
| count_art    | unsigned integer | How much unique `Album` art there are in this `Collection`

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_collection"}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "empty": false,
    "timestamp": 1690410052,
    "count_artist": 195,
    "count_album": 825,
    "count_song": 8543,
    "count_art": 824
  },
  "id": 0
}
```

## state_collection_full
Retrieve full metadata about the current `Collection`.

This method is much heavier superset of `state_collection`.

| Inputs |
|--------|
| None   |

| Outputs                                     | Type                                       | Description |
|---------------------------------------------|--------------------------------------------|-------------|
| empty                                       | boolean                                    | If the `Collection` does NOT have any `Artist`'s, `Album`'s, or `Song`'s
| timestamp                                   | unsigned integer                           | The UNIX timestamp of when this `Collection` was created
| count_artist                                | unsigned integer                           | How many unique `Artist`'s there are in this `Collection`
| count_album                                 | unsigned integer                           | How many unique `Album`'s there are in this `Collection`
| count_song                                  | unsigned integer                           | How many unique `Song`'s there are in this `Collection`
| count_art                                   | unsigned integer                           | How much unique `Album` art there are in this `Collection`
| artists                                     | array of `Artist` objects (see above)      | An array of `Artist` objects
| albums                                      | array of `Album` objects (see above)       | An array of `Album` objects
| songs                                       | array of `Song` objects (see above)        | An array of `Song` objects
| sort_artist_lexi                            | array of `Artist` keys (unsigned integers) | `Artists A-Z`
| sort_artist_lexi_rev                        | array of `Artist` keys (unsigned integers) | `Artists Z-A`
| sort_artist_album_count                     | array of `Artist` keys (unsigned integers) | `Artists per album count (least to most)`
| sort_artist_album_count_rev                 | array of `Artist` keys (unsigned integers) | `Artists per album count (most to least)`
| sort_artist_song_count                      | array of `Artist` keys (unsigned integers) | `Artists per song count (least to most)`
| sort_artist_song_count_rev                  | array of `Artist` keys (unsigned integers) | `Artists per song count (most to least)`
| sort_artist_runtime                         | array of `Artist` keys (unsigned integers) | `Artists runtime shortest-longest`
| sort_artist_runtime_rev                     | array of `Artist` keys (unsigned integers) | `Artists runtime longest-shortest`
| sort_artist_name                            | array of `Artist` keys (unsigned integers) | `Artist name shortest-longest`
| sort_artist_name_rev                        | array of `Artist` keys (unsigned integers) | `Artist name longest-shortest`
| sort_album_release_artist_lexi              | array of `Album` keys (unsigned integers)  | `Artists A-Z, albums oldest-latest`
| sort_album_release_artist_lexi_rev          | array of `Album` keys (unsigned integers)  | `Artists Z-A, albums oldest-latest`
| sort_album_release_rev_artist_lexi          | array of `Album` keys (unsigned integers)  | `Artists A-Z, albums latest-oldest`
| sort_album_release_rev_artist_lexi_rev      | array of `Album` keys (unsigned integers)  | `Artists Z-A, albums latest-oldest`
| sort_album_lexi_artist_lexi                 | array of `Album` keys (unsigned integers)  | `Artists A-Z, albums A-Z`
| sort_album_lexi_artist_lexi_rev             | array of `Album` keys (unsigned integers)  | `Artists Z-A, albums A-Z`
| sort_album_lexi_rev_artist_lexi             | array of `Album` keys (unsigned integers)  | `Artists A-Z, albums Z-A`
| sort_album_lexi_rev_artist_lexi_rev         | array of `Album` keys (unsigned integers)  | `Artists Z-A, albums Z-A`
| sort_album_lexi                             | array of `Album` keys (unsigned integers)  | `Albums A-Z`
| sort_album_lexi_rev                         | array of `Album` keys (unsigned integers)  | `Albums Z-A`
| sort_album_release                          | array of `Album` keys (unsigned integers)  | `Albums oldest-latest`
| sort_album_release_rev                      | array of `Album` keys (unsigned integers)  | `Albums latest-oldest`
| sort_album_runtime                          | array of `Album` keys (unsigned integers)  | `Albums shortest-longest`
| sort_album_runtime_rev                      | array of `Album` keys (unsigned integers)  | `Albums longest-shortest`
| sort_album_title                            | array of `Album` keys (unsigned integers)  | `Album title shortest-longest`
| sort_album_title_rev                        | array of `Album` keys (unsigned integers)  | `Album title longest-shortest`
| sort_song_album_release_artist_lexi         | array of `Song` keys (unsigned integers)   | `Artists A-Z, albums oldest-latest, songs in track order`
| sort_song_album_release_artist_lexi_rev     | array of `Song` keys (unsigned integers)   | `Artists Z-A, albums oldest-latest, songs in track order`
| sort_song_album_release_rev_artist_lexi     | array of `Song` keys (unsigned integers)   | `Artists A-Z, albums latest-oldest, songs in track order`
| sort_song_album_release_rev_artist_lexi_rev | array of `Song` keys (unsigned integers)   | `Artists Z-A, albums latest-oldest, songs in track order`
| sort_song_album_lexi_artist_lexi            | array of `Song` keys (unsigned integers)   | `Artists A-Z, albums A-Z, songs in track order`
| sort_song_album_lexi_artist_lexi_rev        | array of `Song` keys (unsigned integers)   | `Artists Z-A, albums A-Z, songs in track order`
| sort_song_album_lexi_rev_artist_lexi        | array of `Song` keys (unsigned integers)   | `Artists A-Z, albums Z-A, songs in track order`
| sort_song_album_lexi_rev_artist_lexi_rev    | array of `Song` keys (unsigned integers)   | `Artists Z-A, albums Z-A, songs in track order`
| sort_song_lexi                              | array of `Song` keys (unsigned integers)   | `Songs A-Z`
| sort_song_lexi_rev                          | array of `Song` keys (unsigned integers)   | `Songs Z-A`
| sort_song_release                           | array of `Song` keys (unsigned integers)   | `Songs oldest-latest`
| sort_song_release_rev                       | array of `Song` keys (unsigned integers)   | `Songs latest-oldest`
| sort_song_runtime                           | array of `Song` keys (unsigned integers)   | `Songs shortest-longest`
| sort_song_runtime_rev                       | array of `Song` keys (unsigned integers)   | `Songs longest-shortest`
| sort_song_title                             | array of `Song` keys (unsigned integers)   | `Song title shortest-longest`
| sort_song_title_rev                         | array of `Song` keys (unsigned integers)   | `Song title longest-shortest`

The output of this method contains 3 more nested objects:
- `Artist`
- `Album`
- `Song`
See the [`Objects`](#objects) section for more information about their key/values.

The `sort_*` fields are a bunch of keys that represent an ordering.

For example, the `sort_artist_lexi` array contains `Artist` keys that are in `Artist` name `A-Z` ordering, so the first `Artist` in that array will be something like `ArtistStartingWithA` and the last will probably be something like `ZArtist`. 

String sorting is done lexicographically as per [Rust's string ordering implementation](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord). UTF-8 strings, so non-English characters and emojis will work, although it is unclear which languages come first/last, or whether 👺 is before/after 🤡, but regardless, the implementation linked handles that.

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_collection_full"}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "empty": false,
    "timestamp": 1690481104,
    "count_artist": 3,
    "count_album": 4,
    "count_song": 7,
    "count_art": 4,
    "artists": [
      {
        "name": "artist_1",
        "key": 0,
        "runtime": 4,
        "albums": [
          0,
          1
        ],
        "songs": [
          0,
          1,
          2,
          3
        ]
      },
      {
        "name": "artist_2",
        "key": 1,
        "runtime": 2,
        "albums": [
          2
        ],
        "songs": [
          4,
          5
        ]
      },
      {
        "name": "artist_3",
        "key": 2,
        "runtime": 1,
        "albums": [
          3
        ],
        "songs": [
          6
        ]
      }
    ],
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": 10239,
        "genre": null
      },
      {
        "title": "album_2",
        "key": 1,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          2,
          3
        ],
        "discs": 0,
        "art": 10239,
        "genre": null
      },
      {
        "title": "album_3",
        "key": 2,
        "artist": 1,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          4,
          5
        ],
        "discs": 1,
        "art": 10239,
        "genre": null
      },
      {
        "title": "album_4",
        "key": 3,
        "artist": 2,
        "release": "2018-04-25",
        "runtime": 1,
        "song_count": 1,
        "songs": [
          6
        ],
        "discs": 0,
        "art": 10239,
        "genre": null
      }
    ],
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": 2
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": 2
      },
      {
        "title": "mp3",
        "key": 2,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": 2
      },
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": 2
      },
      {
        "title": "m4a",
        "key": 4,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null
      },
      {
        "title": "song_6",
        "key": 5,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": 2
      },
      {
        "title": "mp3",
        "key": 6,
        "album": 3,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": 2
      }
    ],
    "sort_artist_lexi": [
      0,
      1,
      2
    ],
    "sort_artist_lexi_rev": [
      2,
      1,
      0
    ],
    "sort_artist_album_count": [
      1,
      2,
      0
    ],
    "sort_artist_album_count_rev": [
      0,
      2,
      1
    ],
    "sort_artist_song_count": [
      2,
      1,
      0
    ],
    "sort_artist_song_count_rev": [
      0,
      1,
      2
    ],
    "sort_artist_runtime": [
      2,
      1,
      0
    ],
    "sort_artist_runtime_rev": [
      0,
      1,
      2
    ],
    "sort_artist_name": [
      0,
      1,
      2
    ],
    "sort_artist_name_rev": [
      2,
      1,
      0
    ],
    "sort_album_release_artist_lexi": [
      0,
      1,
      2,
      3
    ],
    "sort_album_release_artist_lexi_rev": [
      3,
      2,
      0,
      1
    ],
    "sort_album_release_rev_artist_lexi": [
      1,
      0,
      2,
      3
    ],
    "sort_album_release_rev_artist_lexi_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_lexi_artist_lexi": [
      0,
      1,
      2,
      3
    ],
    "sort_album_lexi_artist_lexi_rev": [
      3,
      2,
      0,
      1
    ],
    "sort_album_lexi_rev_artist_lexi": [
      1,
      0,
      2,
      3
    ],
    "sort_album_lexi_rev_artist_lexi_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_lexi": [
      0,
      1,
      2,
      3
    ],
    "sort_album_lexi_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_release": [
      0,
      1,
      2,
      3
    ],
    "sort_album_release_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_runtime": [
      3,
      0,
      1,
      2
    ],
    "sort_album_runtime_rev": [
      2,
      1,
      0,
      3
    ],
    "sort_album_title": [
      0,
      1,
      2,
      3
    ],
    "sort_album_title_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_song_album_release_artist_lexi": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_album_release_artist_lexi_rev": [
      6,
      4,
      5,
      0,
      1,
      2,
      3
    ],
    "sort_song_album_release_rev_artist_lexi": [
      2,
      3,
      0,
      1,
      4,
      5,
      6
    ],
    "sort_song_album_release_rev_artist_lexi_rev": [
      6,
      4,
      5,
      2,
      3,
      0,
      1
    ],
    "sort_song_album_lexi_artist_lexi": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_album_lexi_artist_lexi_rev": [
      6,
      4,
      5,
      0,
      1,
      2,
      3
    ],
    "sort_song_album_lexi_rev_artist_lexi": [
      2,
      3,
      0,
      1,
      4,
      5,
      6
    ],
    "sort_song_album_lexi_rev_artist_lexi_rev": [
      6,
      4,
      5,
      2,
      3,
      0,
      1
    ],
    "sort_song_lexi": [
      3,
      4,
      0,
      1,
      2,
      6,
      5
    ],
    "sort_song_lexi_rev": [
      5,
      6,
      2,
      1,
      0,
      4,
      3
    ],
    "sort_song_release": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_release_rev": [
      6,
      5,
      4,
      3,
      2,
      1,
      0
    ],
    "sort_song_runtime": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_runtime_rev": [
      6,
      5,
      4,
      3,
      2,
      1,
      0
    ],
    "sort_song_title": [
      0,
      1,
      2,
      4,
      6,
      3,
      5
    ],
    "sort_song_title_rev": [
      5,
      3,
      6,
      4,
      2,
      1,
      0
    ]
  },
  "id": 0
}
```

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
Fuzzy similarity searches on the `Collection`.

In general: input a string, receive some `Artist`/`Album`/`Song`'s that are similar to the input.

## search
Input a `string`, retrieve arrays of `Artist`, `Album`, and `Song` objects, sorted by how similar their names/titles are to the input.

| Inputs | Type                                   | Description |
|--------|----------------------------------------|-------------|
| input  | string                                 | The string to match against, to use as input
| kind   | string, one of `all`, `sim70`, `top25` | This dictates how many objects back you will receive. `all` means ALL objects in the `Collection` will be returned. `sim70` means only objects that are `70%` similar will be returned. `top25` means only the top 25 results will be returned (per object group, so total 75).

| Outputs | Type                      | Description |
|---------|---------------------------|-------------|
| artists | array of `Artist` objects | An array of `Artist` objects, sorted by most similar name first
| albums  | array of `Album` objects  | An array of `Album` objects, sorted by most similar title first
| songs   | array of `Song` objects   | An array of `Song` objects, sorted by most similar title first

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search","params":{"input":"twice","kind":"sim70"}}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artists": [
      {
        "name": "TWICE",
        "key": 106,
        "runtime": 343,
        "albums": [
          598
        ],
        "songs": [
          5411
        ]
      },
    ],
    "albums": [
      {
        "title": "TIME",
        "key": 271,
        "artist": 42,
        "release": "2014-01-21",
        "runtime": 2904,
        "song_count": 3,
        "songs": [
          3058,
          3095,
          3121
        ],
        "discs": 0,
        "art": 1264656,
        "genre": null
      }
    ],
    "songs": [
      {
        "title": "TIME",
        "key": 5560,
        "album": 538,
        "runtime": 249,
        "sample_rate": 44100,
        "track": 5,
        "disc": 1
      }
    ]
  },
  "id": 0
}
```

## search_artist
Input a `string`, retrieve an array of `Artist` objects, sorted by how similar their names are to the input.

| Inputs | Type                                   | Description |
|--------|----------------------------------------|-------------|
| input  | string                                 | The string to match against, to use as input
| kind   | string, one of `all`, `sim70`, `top25` | This dictates how many objects back you will receive. `all` means ALL `Artist`'s will be returned. `sim70` means only `Artist`'s that are `70%` similar will be returned. `top25` means only the top 25 results will be returned.

| Outputs | Type                      | Description |
|---------|---------------------------|-------------|
| artists | array of `Artist` objects | An array of `Artist` objects, sorted by most similar name first

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_artist","params":{"input":"twice","kind":"sim70"}}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "artists": [
      {
        "name": "TWICE",
        "key": 106,
        "runtime": 343,
        "albums": [
          598
        ],
        "songs": [
          5411
        ]
      }
    ]
  },
  "id": 0
}
```

## search_album
Input a `string`, retrieve an array of `Album` objects, sorted by how similar their titles are to the input.

| Inputs | Type                                   | Description |
|--------|----------------------------------------|-------------|
| input  | string                                 | The string to match against, to use as input
| kind   | string, one of `all`, `sim70`, `top25` | This dictates how many objects back you will receive. `all` means ALL `Album`'s will be returned. `sim70` means only `Album`'s that are `70%` similar will be returned. `top25` means only the top 25 results will be returned.

| Outputs | Type                     | Description |
|---------|--------------------------|-------------|
| albums  | array of `Album` objects | An array of `Album` objects, sorted by most similar title first

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_album","params":{"input":"time","kind":"sim70"}}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "albums": [
      {
        "title": "TIME",
        "key": 271,
        "artist": 42,
        "release": "2014-01-21",
        "runtime": 2904,
        "song_count": 3,
        "songs": [
          3058,
          3095,
          3121
        ],
        "discs": 0,
        "art": 1264656,
        "genre": null
      }
    ]
  },
  "id": 0
}
```

## search_song
Input a `string`, retrieve an array of `Song` objects, sorted by how similar their titles are to the input.

| Inputs | Type                                   | Description |
|--------|----------------------------------------|-------------|
| input  | string                                 | The string to match against, to use as input
| kind   | string, one of `all`, `sim70`, `top25` | This dictates how many objects back you will receive. `all` means ALL `Song`'s will be returned. `sim70` means only `Song`'s that are `70%` similar will be returned. `top25` means only the top 25 results will be returned.

| Outputs | Type                    | Description |
|---------|-------------------------|-------------|
| albums  | array of `Song` objects | An array of `Song` objects, sorted by most similar title first

Example Request:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"search_song","params":{"input":"time","kind":"sim70"}}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "songs": [
      {
        "title": "TIME",
        "key": 5560,
        "album": 538,
        "runtime": 249,
        "sample_rate": 44100,
        "track": 5,
        "disc": 1
      }
    ]
  },
  "id": 0
}
```

# Collection
Methods related to the `Collection`.

## new_collection
Reset the current `Collection`.

| Inputs | Type                                 | Description |
|--------|--------------------------------------|-------------|
| paths  | Optional (maybe-null) array of PATHs | An array of filesystem PATHs to scan for the new `Collection`. These must be absolute PATHs **on the system `festivald` is running on**, not PATHs on the client. If `null` is provided, the default `Music` directory will be used.

| Outputs | Type    | Description |
|---------|---------|-------------|
| ok      | boolean | `true` if the request was successfully received, parsed, and started, `false` otherwise

Example Request:
```bash
# Use default Music PATH.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"new_collection","params":{"paths":null}}'
# Use the PATH `/home/user/Music/collection` on `festivald`'s filesystem.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"new_collection","params":{"paths":["/home/user/Music/collection"]}}'
# Windows PATH works too if `\` is escaped (and if `festivald` is running on Windows).
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"new_collection","params":{"paths":["C:\\Users\\User\\Music\\collection"]}}'
```
Example Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "ok": true
  },
  "id": 0
}
```

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
