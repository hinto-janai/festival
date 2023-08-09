# Tor
Like other web services, `festivald` can be set-up & accessed via [Tor](https://torproject.org).

See [here](https://docs.festival.pm/daemon/tor.html) to see how to set `festivald` up with Tor.

`festival-cli` has support for `HTTP`, `SOCKS4`, and `SOCKS5` proxies, which will allow you to connect to `festivald`'s running as [Tor Onion Services](https://community.torproject.org/onion-services/).

### JSON-RPC
To connect to `festivald` over a Tor onion service, you can use `--proxy`:
```bash
ONION="http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion"

festival-cli
	--proxy socks5://127.0.0.1:9050 \ # The Tor SOCKS5 proxy.
	$ONION \                          # The onion address mapped at `festivald`
	state_daemon                      # Method

```
or wrap `festival-cli` with [`torsocks`](https://support.torproject.org/glossary/torsocks):
```bash
torsocks festival-cli $ONION state_daemon
```

### Authentication
Since [Onion Service's](https://community.torproject.org/onion-services/overview/) are end-to-end encrypted, `HTTPS` is not required.

Thus, `festivald` and `festival-cli` can freely pass [`authentication`](config.md) tokens around when used with `onion` addresses.

For `festivald`, since it cannot know if an `onion` address is being mapped to it, you must pass:
```bash
festivald --confirm-no-tls-auth
```
to confirm that you allow authentication without TLS.

For `festival-cli`, it will automatically detect if you're connecting to an onion address and will allow authentication.
