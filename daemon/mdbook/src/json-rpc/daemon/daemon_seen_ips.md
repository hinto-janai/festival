# daemon_seen_ips

#### 🟢 Stable
This API is [stable](../../api-stability/marker.md) since `festivald v1.0.0`.

---

Retrieve an array of the IP addresses `festivald` has seen.

#### Inputs

`None`

#### Outputs
The output is an un-named array containing:

| Field     | Type                  | Description |
|-----------|-----------------------|-------------|
| ip        | string (IPv4 address) | IP address `festivald` has seen
| count     | unsigned integer      | How many connections this IP has made to `festivald`

#### Example Request
```bash
festival-cli daemon_seen_ips
```
```bash
curl http://localhost:18425 -d '{"jsonrpc":"2.0","id":0,"method":"daemon_seen_ips"}'
```

#### Example Response
```
{
  "jsonrpc": "2.0",
  "result": [
    {
      "ip": "127.0.0.1",
      "count": 14
    },
    {
      "ip": "192.168.2.1",
      "count": 2
    }
  ],
  "id": 0
}
```
