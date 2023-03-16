# Search
`Search` receives `String` input from `Kernel`, and runs [`strsim::jaro()`](https://docs.rs/strsim) on all the `Artist/Album/Song`'s in the `Collection`, returning a `Vec<Keychain>` to `Kernel`, with the most similar (`1.0`) at the beginning of the `Vec`, and least similar (`0.0`) at the end.

`Search` also holds a `HashMap<String, Vec<Keychain>>` to act as a cache of previous results. This is limited to `1000` results, before getting reset.

## Files
| File           | Purpose |
|----------------|---------|
| search.rs      | Main `Search` loop
| msg.rs         | Types of messages `Search` and `Kernel` can send to each other
