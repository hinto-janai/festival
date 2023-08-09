# JSON-RPC
`festival-cli`'s main purpose is to send and receive [`JSON-RPC 2.0`](https://www.jsonrpc.org/specification) messages from/to a `festivald`.

The way to send a method to `festival` using `festival-cli` is always the same:
```bash
festival-cli <METHOD> [--PARAM <VALUE>]
```
The `<METHOD>` will always be a method name as [documented in `festivald`](https://docs.festival.pm/daemon/json-rpc/json-rpc.html), and parameters are represented by `--flags <value>`. If a parameter is _optional_, then not passing the `--flag` would be the same as not including the parameter.

For example, to send the [`collection_new`](https://docs.festival.pm/daemon/json-rpc/collection/collection_new.html) method to create a new `Collection`, you would run:
```bash
festival-cli collection_new
```
This method also has an optional parameter, `paths`, which can be specified like this:
```bash
festival-cli collection_new --path /first/path --path /second/path
```

Methods without parameters do not need (and don't have) any associated command flags:
```bash
festival-cli collection_full
```

### Pre-flags
Before specifying a `method`, you can insert some command-lines that alter various things.

For example, to connect to a different `festivald`:
```bash
festival-cli --festivald https://festivald.pm:18425 collection_full
```
To print the configuration that _would_ have been used, but without connecting to anything:
```bash
festival-cli --dry-run collection_full
```
To set a connection timeout
```bash
festival-cli --timeout 5 collection_full
```

These pre-flags must come _before_ the method name, because every `--flag` that comes _after_ `<METHOD>` will be assumed to be a `--parameter`.

### Help
To list all available methods:
```bash
festival-cli --methods
```
To show the parameters/values needed for a specific method:
```bash
festival-cli <METHOD> -h
```

### Detailed Help
Running:
```bash
festival-cli <METHOD> --help
```
will output markdown text equivalent to the `festivald` documentation for that method.

To view the `festivald` documentation proper:
- View it at [`https://docs.festival.pm/daemon`](https://docs.festival.pm/daemon), or
- View it locally with `festivald data --docs`, or
- Serve/view it yourself at `http://localhost:18425` after starting `festivald`

All method documentation will include what inputs it needs, what output to expect, and examples.

### Examples
Each method in [`festivald`'s documentation](https://docs.festival.pm/daemon/json-rpc/json-rpc.html) has a `festival-cli` example.
