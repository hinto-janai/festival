# Tor
Like other web services, `festivald` can be set-up & accessed via [Tor](https://torproject.org).

### Onion Service
`festivald` doesn't have special integration with Tor, but it can easily be turned into a [Tor Onion Service](https://community.torproject.org/onion-services/).

After getting `festivald` setup normally, follow the `Onion Service` instructions [here](https://community.torproject.org/onion-services/setup/).

Your `torrc` should look something like:
```plaintext
HiddenServiceDir /var/lib/tor/festivald
HiddenServicePort 80 127.0.0.1:18425
```
And in `/var/lib/tor/festivald`, you should have your onion service hostname, keys, etc.

This onion service will allow you to interact with `festivald` in all the normal ways, although, you must connect to it via `Tor`.
```http
http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion
```

### JSON-RPC
To use `festivald`'s `JSON-RPC` API over Tor, your HTTP client must either use a proxy, or be wrapped with [`torsocks`](https://support.torproject.org/glossary/torsocks).
```bash
ONION="http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion"

festival-cli
	--proxy socks5://127.0.0.1:9050 \ # The Tor SOCKS5 proxy.
	--festivald $ONION \              # The onion address mapped to `festivald`
	state_daemon                      # Method

# or with `torsocks`
torsocks festival-cli $ONION state_daemon
```
```bash
ONION="http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion"

curl \
	--socks5-hostname 127.0.0.1:9050 \                    # The Tor SOCKS5 proxy.
	--festivald $ONION \                                  # The onion address mapped to `festivald`
	-d '{"jsonrpc":"2.0","id":0,"method":"state_daemon"}' # Method

# or with `torsocks`
torsocks curl $ONION -d '{"jsonrpc":"2.0","id":0,"method":"state_daemon"}'
```

### REST
For the `REST` API, it is the same: your HTTP client must connect over Tor.

Although you could just use [`Tor Browser`](https://www.torproject.org/download/).
```http
# Opening this link in Tor Browser will show a small player for this song.
http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion/map/Artist Name/Artist Title/Song Title
```

The equivalent `curl` command:
```bash
ONION="http://2gzyxa5ihm7nsggfxnu52rck2vv4rvmdlkiu3zzui5du4xyclen53wid.onion"
SOCKS="127.0.0.1:9050"

curl --socks5-hostname $SOCKS -JO $ONION/map/Artist%20Name/Artist%20Title/Song%20Title
```

### Authentication
Since [Onion Service's](https://community.torproject.org/onion-services/overview/) are end-to-end encrypted, `HTTPS` is not required.

Thus, `festivald` and `festival-cli` can freely pass [`authentication`](/config.md) tokens around when used with `onion` addresses.

For `festivald`, since it cannot know if an `onion` address is being mapped to it, you must pass:
```bash
festivald --confirm-no-tls-auth
```
to confirm that you allow authentication without TLS.

For `festival-cli`, it will automatically detect if you're connecting to an onion address and will allow authentication.
