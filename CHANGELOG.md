# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Types of changes:
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities

While `festival-gui` is the only `Frontend` available, the changelog and release versions will refer to `festival-gui`'s version.

---


## Unreleased
## Changed
* `Collection` directories are now pre-emptively cached on startup and addition; initial reset speeds are faster

## Fixed
* Crashes with songs that have odd date metadata, again (https://github.com/hinto-janai/readable/commit/02bdd467363e50627e68af56497eaeb13cdf632d)

---


# v1.0.1 - 2023-07-02
## Added
* `Pixels Per Point` setting for manual UI pixel sizing (https://github.com/hinto-janai/festival/commit/f1fc011fac05e6daec9894477e52030745fea25b)

## Changed
* Lowered minimum resolution to `870x486` from `1000x800` (https://github.com/hinto-janai/festival/commit/075f1b8e48e2c733878ce3ee982e54ca4051fee9)

## Fixed
* Crashes with songs that have `MONTH-DAY` date metadata only ([#1](https://github.com/hinto-janai/readable/pull/1))
* Non-`jpg/png` album art image decoding issues (https://github.com/hinto-janai/festival/commit/f0e96b44e95555a08441e0b99983030bc528f490)


---


# v1.0.0 - 2023-06-28
The PGP key used to sign releases can be found at https://festival.pm/hinto or [`pgp/hinto-janai.asc`](https://github.com/hinto-janai/festival/blob/main/pgp/hinto-janai.asc).
