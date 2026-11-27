# Security policy

## Supported versions

chess-app has no published installer yet. No version is supported until `v0.1.0`.

| Version | Supported |
| --- | --- |
| `0.0.x` (development and beta tags) | No |
| `v0.1.0` and later | After that tag exists |

The engines have their own policies: [rust-chess](https://github.com/RumenDamyanov/rust-chess), [npm-chess](https://github.com/RumenDamyanov/npm-chess), and [go-chess](https://github.com/RumenDamyanov/go-chess).

## Reporting a vulnerability

Report a security issue in private. Do not open a public issue for an undisclosed vulnerability.

- Email [security@rumenx.com](mailto:security@rumenx.com).
- You can also use [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) on [RumenDamyanov/chess-app](https://github.com/RumenDamyanov/chess-app).

Say what you did, what you expected, and what happened. Include the app version from the window title or from `package.json`.

You will get a reply within 7 days. A fix is prepared before any public discussion.

## What belongs in a report

chess-app runs chess engines inside one desktop program. A legal chess move, a lost game, and an AI choice you dislike are the product.

These are in scope:

- The app reading or writing files outside its own saved settings.
- A crafted position or move that crashes the window or the Rust engine.
- A release asset whose checksum does not match `SHA256SUMS`, or a `SHA256SUMS.bundle` that fails the cosign check.

## Transitive `glib` advisory

GitHub reports [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429) on `glib` 0.18.5. The unsound `VariantStrIter` code is fixed in `glib` 0.20.0. That crate is not one chess-app names. It arrives through Tauri's Linux window stack: `tauri` → `gtk` 0.18 → `glib` ^0.18, and the same pin through `wry` and `webkit2gtk` 2.

`gtk` 0.18 accepts only `glib` ^0.18, so Cargo will not select 0.20. The newest Tauri 2.12 release still depends on `gtk` ^0.18 and `webkit2gtk` 2. `wry` 0.58 moves to `gtk` 0.19 and `glib` ^0.22, which is past the fix, and Tauri has not adopted that stack yet.

The bug is in a Linux GTK iterator. The macOS and Windows window paths do not use this crate. `cargo audit` lists it as an unsound warning and still exits successfully. The GitHub alert stays until a Tauri release leaves `gtk` 0.18.

## Checking a release

Release assets include `SHA256SUMS` and a keyless cosign bundle, `SHA256SUMS.bundle`. The verification command is on the wiki page [Check a download](https://github.com/RumenDamyanov/chess-app/wiki/Check-a-download). A failed check means the download should not be installed.
