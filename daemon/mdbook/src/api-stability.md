# API Stability
`festivald`'s JSON output and REST endpoint changes will only be _additional_ until a breaking `v2.0.0` release.

More JSON fields and/or more REST endpoints could be added in the future, however previous fields and endpoints will remain.

Example old `v1.0.0` JSON:
```json
{
  "old_field": "This is a field introduced in v1.0.0",
}
```
Example `v1.x.x`+ JSON:
```json
{
  "old_field": "This is a field introduced in v1.0.0",
  "new_field": "I'm new, introduced in a future v1.x.x release"
}
```
Example `v2.0.0` JSON:
```json
{
  "new_new_field": "The entire output is different",
}
```
