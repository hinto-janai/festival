# Rand
Access a random [`Song`](../../common-objects/song.md), [`Album`](../../common-objects/album.md), or [`Artist`](../../common-objects/artist.md), or in [`Entry`](../../common-objects/entry.md) form.

If the [`Collection`](../../common-objects/collection.md) is empty, a JSON-RPC [`error`](../json-rpc.md#example-json-rpc-20-failed-response) will be returned.

Repeating is not prevented, so there is nothing preventing you from retrieving the same object multiple times in a row.
