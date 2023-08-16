# `JSON-RPC` Quick Start
A quick start to using `festivald`'s `JSON-RPC 2.0` API.

## Create the `Collection` and start playing an `Artist`

1. First, scan the default `Music` directory on `festivald`'s filesystem, and create a `Collection` with [`collection_new`](/json-rpc/collection/collection_new.md):
```bash
festival-cli collection_new
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":null}}'
```

2. Add the `Artist` "LUCKY TAPES" to the queue with [`queue_add_map_artist`](/json-rpc/queue/queue_add_map_artist.md):
```bash
festival-cli queue_add_map_artist --artist "LUCKY TAPES"
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_artist","params":{"artist":"LUCKY TAPES"}}'
```

3. Start playing with [`play`](/json-rpc/playback/play.md):
```bash
festival-cli play
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"play"}'
```


## View state of current audio playback with [`state_audio`](/json-rpc/state/state_audio.md)
```bash
festival-cli state_audio
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_audio"}'
```

## Set the volume to 10% with [`volume`](/json-rpc/playback/volume.md)
```bash
festival-cli volume --volume 10
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume","params":{"volume":10}}'
```

## View the current `Album` with [`current_album`](/json-rpc/current/current_album.md)
```bash
festival-cli current_album
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_album"}'
```

## Clear the queue and stop playback with [`stop`](/json-rpc/playback/stop.md)
```bash
festival-cli stop
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"stop"}'
```

## Create and add an `Artist` to a playlist with [`playlist_add_map_artist`](/json-rpc/playlist/playlist_add_map_artist.md)
```bash
festival-cli playlist_add_map_artist --playlist "Playlist Name" --artist "Artist Name" --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_add_map_artist","params":{"playlist":"Playlist Name","artist":"Artist Name","append":"back","clear":false}}'
```
