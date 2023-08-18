# Tor
Like other web services, `festivald` can be set-up & accessed via [Tor](https://torproject.org).

A public instance of `festivald` with [`Creative Commons`](https://creativecommons.org/licenses/by-nc-nd/4.0/) licensed music is available at:
```http
https://daemon.festival.pm
```

and its [`Onion Service`](https://community.torproject.org/onion-services) is available at:
```http
http://omjo63yjj66ga7jlvhqib4z4qgx6y6oigjcpjcr5ehhfdugfuami3did.onion
```

### Onion Service
`festivald` doesn't have special integration with Tor, but it can easily be turned into an [Onion Service](https://community.torproject.org/onion-services).

After getting `festivald` setup normally, follow the `Onion Service` instructions [here](https://community.torproject.org/onion-services/setup).

Your `torrc` should look something like:
```plaintext
HiddenServiceDir /var/lib/tor/festivald
HiddenServicePort 18425 127.0.0.1:18425
HiddenServicePort 80 127.0.0.1:18425
```
And in `/var/lib/tor/festivald`, you should have your onion service hostname, keys, etc.

This onion service will allow you to interact with `festivald` in all the normal ways, although, you must connect to it via `Tor`.

### JSON-RPC
To use `festivald`'s `JSON-RPC` API over Tor, your HTTP client must either use a proxy, or be wrapped with [`torsocks`](https://support.torproject.org/glossary/torsocks).
```bash
ONION="http://omjo63yjj66ga7jlvhqib4z4qgx6y6oigjcpjcr5ehhfdugfuami3did.onion"

festival-cli
	--proxy socks5://127.0.0.1:9050 \ # The Tor SOCKS5 proxy.
	--festivald $ONION \              # The onion address mapped to `festivald`
	state_daemon                      # Method

# or with `torsocks`
torsocks festival-cli -f $ONION state_daemon
```
```bash
ONION="http://omjo63yjj66ga7jlvhqib4z4qgx6y6oigjcpjcr5ehhfdugfuami3did.onion"

curl \
	--socks5-hostname 127.0.0.1:9050 \                    # The Tor SOCKS5 proxy.
	--festivald $ONION \                                  # The onion address mapped to `festivald`
	-d '{"jsonrpc":"2.0","id":0,"method":"state_daemon"}' # Method
```

### REST
For the `REST` API, it is the same: your HTTP client must connect over Tor.

Although you could just use [`Tor Browser`](https://www.torproject.org/download/).
```http
# Opening this link in Tor Browser will show a small player for this song.
http://omjo63yjj66ga7jlvhqib4z4qgx6y6oigjcpjcr5ehhfdugfuami3did.onion/map/Ludwig van Beethoven/Moonlight Sonata 1/Moonlight Sonata Op. 27, No. 2 In C Sharp Minor: Allegretto
```

The equivalent `curl` command:
```bash
ONION="http://omjo63yjj66ga7jlvhqib4z4qgx6y6oigjcpjcr5ehhfdugfuami3did.onion"
SOCKS="127.0.0.1:9050"

curl --socks5-hostname $SOCKS -JO $ONION/map/Artist%20Name/Artist%20Title/Song%20Title
```

### Authentication
Since [Onion Service's](https://community.torproject.org/onion-services/overview/) are end-to-end encrypted, `HTTPS` is not required.

Thus, `festivald` can freely pass [`authentication`](/config.md) tokens around when used as an `Onion Service`.

Although, since `festivald` cannot know if an `onion` address is being mapped to it, you must pass:
```bash
festivald --confirm-no-tls-auth
```
or set the [`confirm_no_tls_auth`](/config.md) configuration to confirm that you allow authentication without TLS.

For `festival-cli`, it will automatically detect if you're connecting to an onion address and will allow authentication.
