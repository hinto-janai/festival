# Authorization
`festivald` has a [`Basic access authentication`](https://en.wikipedia.org/wiki/Basic_access_authentication) option, with a username + password setup.

If enabled, `festival-cli` must also use `--authorization` or setup the [`authorization`](config.md) config option to connect to it.

`festival-cli` must connect to `festivald` over `HTTPS` or [`.onion`](tor.md) if authorization is enabled or `festival-cli` will refuse to start.

## Syntax
The username & password syntax is specified [here](https://en.wikipedia.org/wiki/Basic_access_authentication).

The "authorization" value must be:
1. The username
2. Followed by a single colon ":"
3. Then the password

For example:
```plaintext
my_user:my_pass
```

A request including this information looks like:
```bash
festival-cli --authorization my_user:my_pass <METHOD>
```

Alternatively, you can input an absolute PATH to a file `festival-cli` can access, containing the string, e.g:
```bash
festival-cli --authorization /path/to/auth.txt <METHOD>
```
In this case, `festival-cli` will read the file and attempt to parse it with the same syntax, i.e, the file should contain:
```plaintext
my_user:my_pass
```
