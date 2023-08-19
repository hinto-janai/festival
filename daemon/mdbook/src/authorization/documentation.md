# Documentation
The [`no_auth_docs`](../config.md) config option or [`--no-auth-docs`](../command-line/command-line.md) command-line flag will allow documentation to be served without authorization, while still requiring authorization for everything else.

## Example
`festivald.toml`:
```toml
authorization = "user:pass"
no_auth_docs  = true
```

Unauthorized client:
```bash
# Even though we didn't specify `-u user:pass`,
# `festivald` will let us download/read docs.
curl https://localhost:18425

# BUT if `REST` requires authentication, this will not work.
curl https://localhost:18425/rand/album
```

Authorized client:
```bash
# This _does_ have authentication,
# so it can do whatever it wants.
curl https://localhost:18425/rand/album -u user:pass
```
