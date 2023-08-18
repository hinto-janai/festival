# Intro
`festival-cli` is a `JSON-RPC` client for [`festivald`](https://docs.festival.pm/daemon).

`festivald` is a music server that plays on the device it is running on, while allowing remote control via clients.

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
4. Prints out `JSON` response

There is a public instance of `festivald` you can use test the API with at:
```http
https://daemon.festival.pm
```
or its [`Onion Address`](https://docs.festival.pm/daemon/tor.html):
```http
http://omjo63yjj66ga7jlvhqib4z4qgx6y6oigjcpjcr5ehhfdugfuami3did.onion
```

For a general quick start, see the next section: [Quick Start](quick-start.md).

### Purpose
`festival-cli` is solely for connecting to, sending requests, and receiving `JSON-RPC 2.0` responses from a `festivald`.

For more information on `festivald`'s API, see here: [https://docs.festival.pm/daemon](https://docs.festival.pm/daemon).

`festival-cli` doesn't have any options for interacting with `festivald`'s `REST` API. A generic HTTP client is better suited for this job, like `curl`, `wget`, or a web browser.

### Documentation
To open this documentation locally:
```bash
festival-cli --docs
```
