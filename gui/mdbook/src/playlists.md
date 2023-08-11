# Playlists
Festival's playlists are not necessarily tied to a `Collection`.

For example, after a `Collection` reset, the songs in a playlist may not exist anymore.

The way Festival handles this is to continue holding onto the "invalid" playlist indefinitely.

Upon every `Collection` reset, Festival will check _every_ "invalid" playlist entry and attempt to "recover" it.

If a matching song with the correct metadata is found after a `Collection` reset, any "invalid" playlist entry that was referencing it will automatically recover.

