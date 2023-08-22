# `festivald`
[`Festival`](https://festival.pm) [daemon](https://en.wikipedia.org/wiki/Daemon_(computing)).

`festivald` is a music server that plays on the device it is running on, and is remotely controlled via clients.

It can also serve music resources, such as song files, album art, and organized archives of whole artists.

The 3 APIs `festivald` exposes:
- [`JSON-RPC 2.0`](https://www.jsonrpc.org/specification) for state retrieval & control
- [`REST`](https://en.wikipedia.org/wiki/Representational_state_transfer) endpoints for serving large resources (audio, art, etc)
- [`Docs`](https://docs.festival.pm/daemon) - `festivald` serves its own documentation

The transport used is `HTTP(s)`.

A public instance of `festivald` with [`Creative Commons`](https://creativecommons.org/licenses/by-nc-nd/4.0/) licensed music is available at:
```
https://daemon.festival.pm
```

For full documentation, see: https://docs.festival.pm/daemon.

### Clients
To interact with `festivald`, you need a client.

The reference `JSON-RPC` client is [`festival-cli`](https://docs.festival.pm/cli).

General purpose `HTTP` clients like [`curl`](https://en.wikipedia.org/wiki/CURL), [`wget`](https://en.wikipedia.org/wiki/Wget) or a web browser will also do.

The documentation will use `festival-cli` & `curl` on the default `http://localhost:18425` for examples.

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
festival-cli toggle
```

For a quick start on the `JSON-RPC` API, see [`JSON-RPC/Quick Start`](/json-rpc/quick-start.md).

### REST
For the `REST` API, you could use anything that can handle `HTTP(s)`, like a web browser:
```
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

For a quick start on the `REST` API, see [`REST/Quick Start`](/rest/quick-start.md).

To disable the `REST` API, set the [config](/config.md) option `rest` to `false` OR pass `--disable-rest` via [command line](/command-line/command-line.md) on start up.

### Documentation
`festivald` will also serve its own documentation.

To access it, start `festivald` and open the root link in a web browser:
```
http://localhost:18425
```
Or you can open the files locally with:
```bash
festivald --docs
```

To disable serving documentation, set the [config](/config.md) option `docs` to `false` OR pass `--disable-docs` via [command line](/command-line/command-line.md) on start up.
