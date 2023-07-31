# Collection
This is the main music "library" or "database" that `festivald` creates and uses.

It is _the_ central component, and almost all other methods/endpoints use it in some way.

The `Collection` contains many nested objects, including the common 3:
- [`Artist`](artist.md)
- [`Album`](album.md)
- [`Song`](song.md)

The `sort_*` fields are a bunch of [keys](key.md) that represent an ordering.

For example, the `sort_artist_lexi` array contains `Artist` keys that are in `Artist` name `A-Z` ordering, so the first `Artist` in that array will be something like `ArtistStartingWithA` and the last will probably be something like `ZArtist`. 

String sorting is done lexicographically as per [Rust's string ordering implementation](https://doc.rust-lang.org/std/primitive.str.html#impl-Ord). UTF-8 strings are accepted, so non-English characters and emojis will work, although it is unclear which languages come first/last, or whether 👺 is before/after 🤡. Regardless, the implementation linked handles that.

This full `Collection` object can be received via the [`state_collection_full`](../json-rpc/state-retrieval/state_collection_full.md) method.

| Field                                       | Type                                       | Description |
|---------------------------------------------|--------------------------------------------|-------------|
| empty                                       | boolean                                    | If the `Collection` does NOT have any `Artist`'s, `Album`'s, or `Song`'s
| timestamp                                   | unsigned integer                           | The UNIX timestamp of when this `Collection` was created
| count_artist                                | unsigned integer                           | How many unique `Artist`'s there are in this `Collection`
| count_album                                 | unsigned integer                           | How many unique `Album`'s there are in this `Collection`
| count_song                                  | unsigned integer                           | How many unique `Song`'s there are in this `Collection`
| count_art                                   | unsigned integer                           | How much unique `Album` art there are in this `Collection`
| artists                                     | array of `Artist` objects                  | An array of [`Artist` objects](artist.md)
| albums                                      | array of `Album` objects                   | An array of [`Album` objects](album.md)
| songs                                       | array of `Song` objects                    | An array of [`Song` objects](song.md)
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

#### Example
```json
{
  "empty": false,
  "timestamp": 1690665271,
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
}
```
