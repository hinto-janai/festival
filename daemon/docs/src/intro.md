# Intro
`Festival` daemon.

`festivald` is a music server that plays on the device it is running on, while allowing remote access via clients. It can also serve music resources, such as song files, album art, and organized archives of whole artists.

The 3 APIs `festivald` exposes:
- [`JSON-RPC 2.0`](https://www.jsonrpc.org/specification) for state retrieval & signal control
- [`REST`](https://en.wikipedia.org/wiki/Representational_state_transfer) endpoints for serving large resources (audio, art, etc)
- [`Documentation`](intro.md): `festivald` serves the very docs you are reading

The transport used is `HTTP(s)`.

For a general quick start, see the next section: [Quick Start](quick-start.md).

### Clients
To interact with `festivald`, you need a client. The reference client is [`festival-cli`](https://github.com/hinto-janai/festival/tree/main/cli).

`HTTP` clients like `curl`, `wget` or a web browser will also do.

The documentation will use `curl` on the default `http://localhost:18425` for examples.

To see documentation for the more user-friendly `festival-cli`, see here:  
[https://github.com/hinto-janai/festival/blob/main/cli/README.md](https://github.com/hinto-janai/festival/blob/main/cli/README.md).

### JSON-RPC
For the `JSON-RPC` API, anything that can transmit `JSON-RPC` over `HTTP(s)` can be a client, like `curl`:
```bash
# Toggle playback.
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"toggle"}'
```

The equivalent `wget` command:
```bash
wget -qO- http://localhost:18425 --post-data '{"jsonrpc":"2.0","id":0,"method":"toggle"}'
```

The equivalent `festival-cli` command:
```bash
festival-cli rpc toggle
```

For a quick start on the `JSON-RPC` API, see [here](json-rpc/quick-start.md).

### REST
For the `REST` API, you could use anything that can handle `HTTP(s)`, like a browser:
```http
# Opening this link in a browser will show a small player for this song.
http://localhost:18425/map/Artist Name/Artist Title/Song Title
```

The equivalent `curl` command:
```bash
curl -JO http://localhost:18425/map/Artist%20Name/Artist%20Title/Song%20Title
```

The equivalent `wget` command:
```bash
wget --content-disposition "http://localhost:18425/map/Artist Name/Artist Title/Song Title"
```

The equivalent `festival-cli` command:
```bash
festival-cli rest map --artist "Artist Name" --album "Album Title" --song "Song Title"
```

For a quick start on the `REST` API, see [here](rest/quick-start.md).

To disable the `REST` API, set the [config](config.md) option `rest` to `false` OR pass the `--disable-rest` via [command line](command-line.md) on start up.

### Documentation
`festivald` will also serve _this_ documentation.

To access it, start `festivald` and open the root link in a web browser:
```http
http://localhost:18425
```
Or you can open the files locally with:
```bash
./festivald data --docs
```

To disable serving documentation, set the [config](config.md) option `docs` to `false` OR pass the `--disable-docs` via [command line](command-line.md) on start up.
