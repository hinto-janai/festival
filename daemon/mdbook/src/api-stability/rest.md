# REST
[`REST`](../rest/rest.md)-specific API stability notes.

## ZIP File Hierarchy
The file hierarchy within ZIPs are stable.

This below example will be the same for the rest of `festivald v1.x.x`:
```plaintext
# Input
https://localhost:18425/map/My Artist

# Output
My Artist.zip

# Extracted
My Artist/
    ├─ Album Name 1/
    │    ├─ Album Name 1.jpg
    │    ├─ Song Name 1.mp3
    │
    │─ Album Name 2/
    │    ├─ Album Name 2.png
    │    ├─ Song Name 2.mp3
```


