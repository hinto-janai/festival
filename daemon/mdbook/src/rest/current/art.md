# /current/art
Download the `Album` art of the currently set `Song`, in the original format.

If no art was found, the response will be a 404 error.

#### Input
`None`

#### Output
Art in original format.

#### Example Input
```http
http://localhost:18425/current/art
```

#### Example Output
```plaintext
Artist Name - Album Title.jpg
```