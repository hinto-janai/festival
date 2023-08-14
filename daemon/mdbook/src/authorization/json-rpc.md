# JSON-RPC
The [`no_auth_rpc`](../conf/config.md) config option or [`--no-auth-rpc`](../command-line/command-line.md) command-line flag will allow specified JSON-RPC calls without authorization, while still requiring authorization for everything else.

If a [JSON-RPC method](../json-rpc/json-rpc.md) is listed in these options `festivald` will allow any client to use it, regardless of authorization.

This allows you to have `authorization` enabled across the board, but allow specific JSON-RPC calls for public usage.

## Usage
The method names in the option must match the exact names of the [actual methods](../json-rpc/json-rpc.md).

For example:
- [`toggle`](../json-rpc/playback/toggle.md)
- [`state_audio`](../json-rpc/state/state_audio.md)
- [`collection_new`](../json-rpc/collection/collection_new.md)

If a specified method name is incorrect, `festivald` will not start.

## Example
For example, if only `toggle` is listed, then clients WITHOUT authorization will only be allowed to use the `toggle` method, for every other method, they must authenticate.

`festivald.toml`:
```toml
authorization = "user:pass"
no_auth_rpc   = ["toggle"]
```

Unauthorized client:
```bash
# Even though we didn't specify `-u user:pass`,
# `festivald` will accept this RPC call.
curl https://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"toggle"}'

# But not this one.
curl https://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"stop"}'
```

Authorized client:
```bash
# This _does_ have authentication,
# so it can do whatever it wants.
curl https://localhost:18425 -u user:pass -d '{"jsonrpc":"2.0","id":0,"method":"stop"}'
```
