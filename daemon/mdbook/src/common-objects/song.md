# Song

| Field       | Type                                   | Description |
|-------------|----------------------------------------|-------------|
| title       | string                                 | The title of this `Song`
| key         | `Song` key (unsigned integer)          | The `Song` key associated with this `Song`
| album       | `Album` key (unsigned integer)         | The `Album` key of the `Album` this `Song` is from
| runtime     | unsigned integer                       | The total runtime of this `Song` in seconds
| sample_rate | unsigned integer                       | The sample rate of this `Song` in hertz, e.g: `44100`
| track       | optional (maybe null) unsigned integer | Track number of this `Song`, `null` if not found
| disc        | optional (maybe null) unsigned integer | Disc number this `Song` belongs to, `null` if not found

#### Example
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