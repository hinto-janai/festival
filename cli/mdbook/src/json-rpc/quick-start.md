# `JSON-RPC` Quick Start
A quick start to using `festivald`'s `JSON-RPC 2.0` API.

## Create the `Collection` and start playing an `Artist`

1. First, scan the default `Music` directory on `festivald`'s filesystem, and create a `Collection` with [`collection_new`](collection/collection_new.md):
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":null}}'
```

2. Add the `Artist` "LUCKY TAPES" to the queue with [`add_queue_map_artist`](playback-control/add_queue_map_artist.md):
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"add_queue_map_artist","params":{"artist":"LUCKY TAPES"}}'
```

3. Start playing with [`play`](playback-control/play.md):
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"play"}'
```


## View state of current audio playback with [`state_audio`](state-retrieval/state_audio.md)
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_audio"}'
```

## Set the volume to 10% with [`volume`](playback-control/volume.md)
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume","params":{"volume":10}}'
```

## View the current `Album` with [`current_album`](current/current_album.md)
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_album"}'
```

## Clear the queue and stop playback with [`stop`](playback-control/stop.md)
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"stop"}'
```
