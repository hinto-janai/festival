# API Stability
Some notes on what parts of `festivald`'s API can/cannot be relied upon.

In general - **things may be added, but never removed.**

If something is unstable, it will be [`marked`](marker.md) as such.

## Breaking Changes
Breaking changes to the [`stable`](marker.md) API may occur in 3 situations:

1. `festivald v2.0.0` release
2. There is a fundamental/security bug that must be fixed in `festivald`
3. There is a _difference_ between this documentation and the actual `festivald` input/output

These will be noted in release notes if they ever occur.
