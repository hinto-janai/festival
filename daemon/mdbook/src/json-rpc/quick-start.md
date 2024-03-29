# `JSON-RPC` Quick Start
A quick start to using `festivald`'s `JSON-RPC 2.0` API.

## Create the [`Collection`](../common-objects/collection.md) and start playing an [`Artist`](../common-objects/artist.md)

1. First, scan the default `Music` directory on `festivald`'s filesystem, and create a `Collection` with [`collection_new`](collection/collection_new.md):
```bash
festival-cli collection_new
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"collection_new","params":{"paths":null}}'
```

2. Add the `Artist` "LUCKY TAPES" to the queue with [`queue_add_map_artist`](queue/queue_add_map_artist.md):
```bash
festival-cli queue_add_map_artist --artist "LUCKY TAPES"
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"queue_add_map_artist","params":{"artist":"LUCKY TAPES"}}'
```

3. Start playing with [`play`](playback/play.md):
```bash
festival-cli play
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"play"}'
```


## View state of current audio playback with [`state_audio`](state/state_audio.md)
```bash
festival-cli state_audio
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"state_audio"}'
```

## Set the volume to 10% with [`volume`](playback/volume.md)
```bash
festival-cli volume --volume 10
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"volume","params":{"volume":10}}'
```

## View the current [`Album`](../common-objects/album.md) with [`current_album`](current/current_album.md)
```bash
festival-cli current_album
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"current_album"}'
```

## Clear the queue and stop playback with [`stop`](playback/stop.md)
```bash
festival-cli stop
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"stop"}'
```

## Create and add an `Artist` to a playlist with [`playlist_add_map_artist`](playlist/playlist_add_map_artist.md)
```bash
festival-cli playlist_add_map_artist --playlist "Playlist Name" --artist "Artist Name" --append back
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"playlist_add_map_artist","params":{"playlist":"Playlist Name","artist":"Artist Name","append":"back","clear":false}}'
```
