# collection_full

#### 🟡 Incomplete
This API's output may have [additions](../../api-stability/marker.md) in the future.

---

Retrieve full metadata about the current [`Collection`](../../common-objects/collection.md).

#### Inputs
`None`

#### Outputs
The output of this method will be a full [`Collection`](../../common-objects/collection.md) object.

#### Example Request
```bash
festival-cli collection_full
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_full"}'
```

#### Example Response
```json
{
  "jsonrpc": "2.0",
  "result": {
    "empty": false,
    "timestamp": 1691018515,
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
        "disc": 2,
        "mime": "audio/mpeg",
        "extension": "mp3"
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": 2,
        "mime": "audio/mpeg",
        "extension": "mp3"
      },
      {
        "title": "mp3",
        "key": 2,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": 2,
        "mime": "audio/mpeg",
        "extension": "mp3"
      },
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": 2,
        "mime": "audio/x-flac",
        "extension": "flac"
      },
      {
        "title": "m4a",
        "key": 4,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "audio/m4a",
        "extension": "m4a"
      },
      {
        "title": "song_6",
        "key": 5,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": 2,
        "mime": "audio/ogg",
        "extension": "ogg"
      },
      {
        "title": "mp3",
        "key": 6,
        "album": 3,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": 2,
        "mime": "audio/mpeg",
        "extension": "mp3"
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
