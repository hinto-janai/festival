# Command Line
`festival-cli` takes in `--flags` for many purposes.

Arguments passed to `festival-cli` will always take priority over the same [configuration](../config.md) options read from disk.

See [`JSON-RPC`](json-rpc.md) for method command-line syntax.

```plaintext
Usage: festival-cli [OPTIONS] [METHOD] [--PARAM <ARG>]

Arguments passed to `festival-cli` will always take
priority over configuration options read from disk.

Commands:
  collection_new            Create a new Collection (and replace the current one)
  collection_brief          Retrieve some brief metadata about the current Collection
  collection_full           Retrieve full metadata about the current Collection
  collection_relation       Retrieve an array of every Song in the current Collection, with its relational data
  collection_relation_full  Retrieve an array of every Song in the current Collection, with its relational data
  collection_perf           View some performance stats about the latest Collection construction
  collection_resource_size  View the size of the current Collection's underlying resources (audio files and art)
  state_ip                  Retrieve an array of the IP addresses festivald has seen
  state_config              Retrieve the active configuration of festivald
  state_daemon              Retrieve state about the status of festivald itself
  state_audio               Retrieve audio state
  state_reset               Retrieve the current state of a Collection reset
  key_artist                Input an Artist key, retrieve an Artist
  key_album                 Input an Album key, retrieve an Album
  key_song                  Input a Song key, retrieve an Song
  map_artist                Input an Artist name, retrieve an Artist
  map_album                 Input an Artist name and Album title, retrieve an Album
  map_song                  Input an Artist name, Album title, and Song title, retrieve a Song
  current_artist            Access the Artist of the currently set Song
  current_album             Access the Album of the currently set Song
  current_song              Access the currently set Song
  rand_artist               Access a random Artist
  rand_album                Access a random Album
  rand_song                 Access a random Song
  search                    Input a string, retrieve arrays of Artist's, Album's, and Song's, sorted by how similar their names/titles are to the input
  search_artist             Input a string, retrieve an array of Artist's, sorted by how similar their names are to the input
  search_album              Input a string, retrieve an array of Album's, sorted by how similar their titles are to the input
  search_song               Input a string, retrieve an array of Song's, sorted by how similar their titles are to the input
  toggle                    Toggle playback
  play                      Start playback
  pause                     Pause playback
  next                      Skip to the next song in the queue
  stop                      Clear the queue and stop playback
  shuffle                   Shuffle the current queue, then start playing from the 1st Song in the queue
  repeat_off                Turn off repeating
  repeat_song               Turn on song repeating
  repeat_queue              Turn on queue repeating
  previous                  Set the current Song to the previous in the queue
  volume                    Set the playback volume
  clear                     Clear the queue
  seek                      Seek forwards/backwards or to an absolute second in the current Song
  skip                      Skip forwards a variable amount of Song's in the current queue
  back                      Go backwards a variable amount of Song's in the current queue
  queue_add_key_artist      Add an Artist to the queue with an Artist key
  queue_add_key_album       Add an Album to the queue with an Album key
  queue_add_key_song        Add an Song to the queue with an Song key
  queue_add_map_artist      Add an Artist to the queue with an Artist name
  queue_add_map_album       Add an Album to the queue with an Artist name and Album title
  queue_add_map_song        Add a Song to the queue with an Artist name Album title, and Song title
  queue_add_rand_artist     Add a random Artist to the queue
  queue_add_rand_album      Add a random Album to the queue
  queue_add_rand_song       Add a random Song to the queue
  queue_add_playlist        Add a playlist to the queue
  queue_set_index           Set the current Song to a queue index
  queue_remove_range        Remove a range of queue indices
  playlist_new              Create a new empty playlist
  playlist_remove           Remove a playlist
  playlist_clone            Clone a playlist into a new one
  playlist_remove_entry     Remove a song in a playlist
  playlist_add_key_artist   Add an artist to a playlist
  playlist_add_key_album    Add an album to a playlist
  playlist_add_key_song     Add a song to a playlist
  playlist_add_map_artist   Add an artist to a playlist
  playlist_add_map_album    Add an album to a playlist
  playlist_add_map_song     Add a song to a playlist
  playlist_names            Retrieve all playlist names
  playlist_count            Retrieve how many playlists there are
  playlist_single           Retrieve a single playlist
  playlist_all              Retrieve all playlists
  help                      Print this message or the help of the given subcommand(s)

Options:
      --festivald <URL>
          URL of the `festivald` to connect to
          
          The protocol, IPv4 address, and port of the
          `festivald` that `festival-cli` will connect
          to by default.
          
          Protocol must be:
            - http
            - https
          
          IP address must be IPv4.
          
          Default is: `http://127.0.0.1:18425`

      --timeout <SECONDS>
          Set a timeout for a non-responding `festivald`
          
          If `festivald` does not respond with _at least_
          a basic HTTP header within this time (seconds),
          `festival-cli` will disconnect.
          
          0 means never disconnect.

      --id <ID>
          The `JSON-RPC 2.0` ID to send to `festivald`.
          
          See below for more info:
          <https://jsonrpc.org/specification>

      --authorization <USER:PASS or FILE>
          Authorization sent to `festivald`
          
          This matches the `authorization` config
          in `festivald`, see here for more info:
          <https://docs.festival.pm/daemon/authorization/authorization.html>
          
          A `festivald` with HTTPS must be used or `festival-cli`
          will refuse to start.
          
          An empty string disables this feature.
          
          Alternatively, you can input an absolute PATH to a file
          `festival-cli` can access, containing the string, e.g:
          ```
          authorization = "/path/to/user_and_pass.txt"
          ```
          
          In this case, `festival-cli` will read the file and attempt
          to parse it with the same syntax, i.e, the file should contain:
          ```
          my_user:my_pass
          ```

      --dry-run
          Print the options that would be used, but don't actually connect to `festivald`

      --docs
          Open `festival-cli` documentation locally in browser

      --path
          Print the PATHs used by `festival-cli`
          
          All data saved by `festival-cli` is saved in these directories.
          For more information, see: <https://docs.festival.pm/cli/disk.html>

      --reset-config
          Reset the current `festival-cli.toml` config file to the default
          
          Exits with `0` if everything went ok, otherwise shows error.

      --delete
          Delete all `festival-cli` files that are on disk
          
          This deletes all `cli` Festival folders.
          The PATHs deleted will be printed on success.

      --methods
          Print all the methods available

  -v, --version
          Print version

  -h, --help
          Print help (see a summary with '-h')
```
