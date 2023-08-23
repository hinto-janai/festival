# `festival-cli`
`festival-cli` is a [`JSON-RPC 2.0`](https://www.jsonrpc.org/specification) client for [`festivald`](https://docs.festival.pm/daemon).

`festivald` is a music server that plays on the device it is running on, and is remotely controlled by clients.

`festival-cli` is a client that reduces the verbosity of `JSON-RPC` requests, so instead of:
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":null}}'
```
You can use:
```bash
festival-cli collection_new
```

The typical invocation of `festival-cli`:
1. Reads the config file
2. Reads the command-line options
3. Connects and sends a request to `festivald`
4. Prints out the `JSON` response

A public instance of `festivald` with [`Creative Commons`](https://creativecommons.org/licenses/by-nc-nd/4.0/) licensed music is available at:
```
https://daemon.festival.pm
```

For full documentation, see: https://docs.festival.pm/cli.

### Purpose
`festival-cli` is solely for connecting to, sending requests, and receiving `JSON-RPC 2.0` responses from a `festivald`.

For more information on `festivald`'s API, see here: [https://docs.festival.pm/daemon](https://docs.festival.pm/daemon).

`festival-cli` doesn't have any options for interacting with `festivald`'s `REST` API. A generic HTTP client is better suited for this job, like `curl`, `wget`, or a web browser.

### Documentation
To open documentation locally:
```bash
festival-cli --docs
```
