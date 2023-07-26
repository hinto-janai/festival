# `festivald`
`Festival` daemon.

* [Command Line](#Command-Line)
* [Disk](#Disk)
* [REST](#REST)
	- [/key](#key)
		- [/artist/$ARTIST_KEY](#artistartist_key)
		- [/album/$ALBUM_KEY](#albumalbum_key)
		- [/song/$SONG_KEY](#songsong_key)
		- [/art/$ALBUM_KEY](#artalbum_key)
	- [/string](#string)
		- [/$ARTIST_NAME](#artist_name)
		- [/$ARTIST_NAME/$ALBUM_TITLE](#artist_namealbum_title)
		- [/$ARTIST_NAME/$ALBUM_TITLE/$SONG_TITLE](#artist_namealbum_titlesong_title)
	- [/art/$ARTIST_NAME/$ALBUM_TITLE](#artartist_namealbum_title)
* [JSON-RPC](#JSON-RPC)
	- [State Retrieval](#State-Retrieval)
		- [state_daemon](#state_daemon)
		- [state_audio](#state_audio)
		- [state_reset](#state_reset)
		- [state_collection](#state_collection)
	- [Playback Control](#Playback-Control)
		- [toggle](#toggle)
		- [play](#play)
		- [pause](#pause)
		- [next](#next)
		- [stop](#stop)
		- [repeat_off](#repeat_off)
		- [repeat_song](#repeat_song)
		- [repeat_queue](#repeat_queue)
		- [shuffle](#shuffle)
		- [previous](#previous)
		- [volume](#volume)
		- [add_queue_song](#add_queue_song)
		- [add_queue_album](#add_queue_album)
		- [add_queue_artist](#add_queue_artist)
		- [clear](#clear)
		- [seek](#seek)
		- [skip](#skip)
		- [back](#back)
		- [set_queue_index](#set_queue_index)
		- [remove_queue_range](#remove_queue_range)
	- [Key](#Key)
		- [key_artist](#key_artist)
		- [key_album](#key_album)
		- [key_song](#key_song)
	- [Map](#Map)
		- [map_artist](#map_artist)
		- [map_album](#map_album)
		- [map_song](#map_song)
	- [Search](#Search)
		- [search](#search)
		- [search_artist](#search_artist)
		- [search_album](#search_album)
		- [search_song](#search_song)
	- [Collection](#Collection)
		- [new_collection](#new_collection)

# Command Line

---

# Disk

---

# REST
## /key
Description.

### /artist/$ARTIST_KEY
Description.

| Input | Type |
|-------|------|

| Output | Type |
|--------|------|

Example:
```bash
curl http://127.0.0.1:18425 -d '{"jsonrpc":"2.0","id":"0","method":"$METHOD_NAME","params":["$PARAMS"]'
```

### /album/$ALBUM_KEY
### /song/$SONG_KEY
### /art/$ALBUM_KEY

## /string
### /$ARTIST_NAME
### /$ARTIST_NAME/$ALBUM_TITLE
### /$ARTIST_NAME/$ALBUM_TITLE/$SONG_TITLE

## /art/$ARTIST_NAME/$ALBUM_TITLE

---

# JSON-RPC
## State Retrieval
### state_daemon
### state_audio
### state_reset
### state_collection

## Playback Control
### toggle
### play
### pause
### next
### stop
### repeat_off
### repeat_song
### repeat_queue
### shuffle
### previous
### volume
### add_queue_song
### add_queue_album
### add_queue_artist
### clear
### seek
### skip
### back
### set_queue_index
### remove_queue_range

## Key
### key_artist
### key_album
### key_song

## Map
### map_artist
### map_album
### map_song

## Search
### search
### search_artist
### search_album
### search_song

## Collection
### new_collection

---
