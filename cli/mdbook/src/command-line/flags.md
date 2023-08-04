# Top-level flags
Top-level command flags.

```plaintext
Usage: festivald [OPTIONS] [COMMAND + OPTIONS] [ARGS...]

Options:
      --ip <IP>
          The IPv4 address `festivald` will bind to [default: 127.0.0.1]

      --port <PORT>
          The port `festivald` will bind to [default: 18425]

      --max-connections <NUMBER>
          Max amount of connections [default: unlimited]
          
          The max amount of connections `festivald`
          will serve at any given moment.
          `0` means unlimited.
          
          Note that 1 client doesn't necessarily mean
          1 connection. A single web browser client for
          example can make many multiple connections
          to `festivald`.

      --exclusive-ip <IP>
          Only accept connections from these IPs
          
          `festivald` will only serve connections coming
          from these IPs. If there's no value given or
          any of the values is "0.0.0.0", `festivald`
          will serve all IP ranges.
          
          To allow multiple IPs, use this flag per IP.
          
          Example: `festivald --exclusive-ip 127.0.0.1 --exclusive-ip 192.168.2.1`

      --tls
          Enable HTTPS
          
          You must also provide a PEM-formatted X509 certificate
          and key in the below options for this to work.
          
          Example: `festivald --tls --certificate /path/to/cert.pem --key /path/to/key.pem`

      --certificate <FILE>
          The PEM-formatted X509 certificate file used for TLS

      --key <FILE>
          The PEM-formatted key file used for TLS

      --authorization <USER:PASS or FILE>
          Enforce a `username` and `password` for connections to `festivald`
          
          Only process connections to `festivald` that have a
          "authorization" HTTP header with this username and password.
          
          If either the `--no-auth-rpc` or `--no-auth-rest` options are
          used, then every RPC call/REST endpoint _NOT_ in those lists
          will require this authorization.
          
          TLS must be enabled for this feature to work
          or `festivald` will refuse to start.
          
          This value must be:
            1. The "username"
            2. Followed by a single colon ":"
            3. Then the "password", e.g:
          ```
          festivald --authorization my_user:my_pass
          ```
          An empty string disables this feature.
          
          Alternatively, you can input an absolute PATH to a file
          `festivald` can access, containing the string, e.g:
          ```
          festivald --authorization "/path/to/user_and_pass.txt"
          ```
          In this case, `festivald` will read the file and attempt
          to parse it with the same syntax, i.e, the file should contain:
          ```
          my_user:my_pass
          ```

      --no-auth-rpc <METHOD>
          Allow specified JSON-RPC calls without authorization
          
          If a JSON-RPC method is listed in this array,
          `festivald` will allow any client to use it,
          regardless of authorization.
          
          This allows you to have `authorization` enabled
          across the board, but allow specific JSON-RPC
          calls for public usage.
          
          For example, if only `toggle` is listed, then
          clients WITHOUT authorization will only be
          allowed to use the `toggle` method, for every
          other method, they must authenticate.
          
          The method names listed here must match the
          exact names when using them, or shown in the
          documentation, see here:
          
          <https://docs.festival.pm/daemon/json-rpc/json-rpc.html>
          
          OR WITH
          
          ```
          festivald data --docs
          ```
          
          To allow multiple methods, use this flag per method.
          
          Example: `festivald --no-auth-rpc toggle --no-auth-rpc volume`

      --no-auth-rest <RESOURCE>
          Allow specified REST resources without authorization,
          
          REST resources, from most expensive to least:
            - `collection`
            - `artist`
            - `album`
            - `song`
            - `art`
          
          If a REST resource is listed in this array,
          `festivald` will allow any client to use it,
          regardless of authorization.
          
          For example, if only `art` is listed, then
          clients WITHOUT authorization will only be
          allowed to use the `art` related endpoints
          (/rand/art, /current/art, etc). For every
          other endpoint (/rand/song, /collection, etc),
          they must authenticate.
          
          To allow multiple methods, use this flag per method.
          
          Example: `festivald --no-auth-rest art --no-auth-rest song`

      --sleep-on-fail <MILLI>
          Sleep before responding to (potentially malicious) failed connections
          
          Upon a failed, potentially malicious request, instead of
          immediately responding, `festivald` will randomly sleep
          up to this many milliseconds before responding to the connection.
          
          This includes:
            - Authentication failure
            - IPs not in the `exclusive_ips` list
            - IPv6 connections
          
          If 0, `festivald` will immediately respond. This may
          not be wanted to due potential DoS and timing attacks.
          
          If you're hosting locally (127.0.0.1), you can set this
          to 0 (unless you don't trust your local network?).

      --collection-path <PATH>
          Default PATHs to use for the `Collection`
          
          Upon a `collection_new` JSON-RPC method call, if the
          `paths` parameter is empty, these PATHs will be scanned
          instead.
          
          If this is not set, the default OS `Music` directory will be scanned.
          
          To set multiple PATHs, use this flag per PATH.
          
          Example: `festivald --collection-path /my/path/1 --collection-path /my/path/2`

      --direct-download
          Enable direct downloads via the REST API for browsers
          
          By default, accessing the REST API via a browser
          will open the resource in the browser (audio player,
          image viewer, etc)
          
          Using this flag will make browsers download
          the file directly, without opening it.

      --filename-separator <SEPARATOR>
          When files are downloaded via the REST API, and the
          file is a nested object referencing multiple things
          (e.g, an _album_ owned by an _artist_), we must include
          that information, but what string should separate them?
          
          The default separator is " - ", e.g:
          ```
          Artist Name - Album Title.zip
          ```
          it can be changed to any string, like "/":
          ```
          Artist Name/Album Title.zip
          ```
          or left empty "" for no separator at all.

      --cache-time <SECONDS>
          Set the `REST` API cache time limit
          
          When serving `ZIP` files via the REST API, `festivald`
          will first write them to disk, then serve those files
          instead of directly storing everything in memory,
          as to not get OOM-killed on more than a few requests.
          
          This option sets the time limit on how many seconds
          `festivald` will hold onto this cache for.
          
          Once the time limit is up, `festivald` will remove the
          file. This cache is also reset on startup and shutdown.
          
          Default is 3600 seconds (1 hour).

      --disable-watch
          Disable watching the filesystem for signals
          
          The way a newly launched Festival communicates to
          an already existing one (e.g, `festivald --play`) is
          by creating a file in Festival's `signal` directory.
          
          `festivald --FLAG` just creates a file in that directory,
          which an existing Festival will notice and do the appropriate task.
          
          Using `--disable-watch` will disable that part of the system so that
          filesystem signals won't work, e.g, `festivald --play` will not work.

      --disable-media-controls
          Disable OS media controls
          
          Festival plugs into the native OS's media controls so that signals
          like `play/pause/stop` and/or keyboard controls can be processed.
          
          `--disable-media-controls` disables this.

      --disable-rest
          Disable the REST API
          
          This is responsible for the `/rest` API that
          serves image, audio, and other heavy resource data.
          
          `--disable-rest` will disable this part of the system,
          and will only leave the JSON-RPC API available.

      --disable-docs
          Enable/disable serving documentation
          
          By default, `festivald` serves a markdown book
          of it's own documentation, accessible at the
          root `/` endpoint, e.g:
          ```
          http://localhost:18425/
          ```
          
          `--disable-docs` will disable that.

      --log-level <OFF|ERROR|INFO|WARN|DEBUG|TRACE>
          Set filter level for console logs

  -v, --version
          Print version

  -h, --help
          Print help (see a summary with '-h')
```
