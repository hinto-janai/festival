# Authorization
`festivald` has a basic username + password authorization [configuration](config.md) option.

If enabled, `festivald` will only process connections to it that have the "authorization" HTTP header with this username and password.

TLS must be enabled for this feature to work or `festivald` will refuse to start.

The option can either be set in the [config](config.md) file or passed via a [command-line flag](command-line/command-line.md).

## Syntax
The username & password syntax is the same as `curl`.

The "authorization" value must be:
1. The username
2. Followed by a single colon ":"
3. Then the password

For example:
```
my_user:my_pass
```

A request including this information looks like:
```
curl -u my_user:my_pass https://127.0.0.1:18425
```
or the equivalent wget command:
```
wget --user my_user --password my_pass https://localhost:18425
```

Alternatively, you can input an absolute PATH to a file `festivald` can access, containing the string, e.g:
```
authorization = "/path/to/user_and_pass.txt"
```
In this case, `festivald` will read the file and attempt to parse it with the same syntax, i.e, the file should contain:
```
my_user:my_pass
```

## Web Browser
If connecting to `festivald` via a web browser, you will receive a visual prompt for authentication:

![authorization](authorization.png)
