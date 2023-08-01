# /map
This is the same as the [`/key`](../key/key.md) endpoint, but instead of numbers, you can directly use:
- Artist names
- Album titles
- Song titles

So instead of:
```
http://localhost:18425/key/song/123
```
you can use:
```
http://localhost:18425/map/Artist Name/Artist Title/Song Title
```
Browsers will secretly percent-encode this URL, so it'll actually be:
```
http://localhost:18425/map/Artist%20Name/Artist%20Title/Song%20Title
```
This is fine, `festivald` will decode it, along with any other percent encoding, so you can use spaces or any other UTF-8 characters directly in the URL:
```
http://localhost:18425/map/артист/❤️/ヒント じゃない
```

The reason `Artist` names and `Album` titles have to be specified is to prevent collisions.

If there's 2 songs in your `Collection` called: `Hello World`, which one should `festivald` return?

Since `Artist` names are unique, and `Album` titles within `Artist`'s are unique, they serve as an identifier.

Also note: words are case-sensitive and must be exact.

If you have an `Album` called `Hello World`, none of these inputs will work:
- `Hello world`
- `hello World`
- `HELlo World`
- `HelloWorld`
- `H3ll0 W0rld`

The input must _exactly_ be `Hello World`.
