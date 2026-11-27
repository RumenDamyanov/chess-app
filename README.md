# chess-app

A desktop window for the chess engines already built.

The person playing opens chess-app. They do not open a browser tab. The window is a system web view, the same shape as [Namevane](https://github.com/namevane/namevane). The engines stay in their own repositories. This repository composes them.

## Related projects

| Project | Role |
|---|---|
| [react-chess](https://github.com/RumenDamyanov/react-chess) | UI source. It stays a browser demo. |
| [@rumenx/chess](https://www.npmjs.com/package/@rumenx/chess) and [npm-chess](https://github.com/RumenDamyanov/npm-chess) | JavaScript engine. The desktop window will call the library. The Express server in npm-chess is not a dependency. |
| [rust-chess](https://github.com/RumenDamyanov/rust-chess) | `rumenx-chess` crate. The desktop process will call `engine` and `ai` in-process. |
| [go-chess](https://github.com/RumenDamyanov/go-chess) | Go engine. Not linked into this version. |

## Status

The window opens and shows the product name. The board and the engines are not wired in yet.

## Develop

You need Node.js 22.23.4+, 24.22.1+, or 26.12.1+, Rust, and the system libraries [Tauri documents for your platform](https://v2.tauri.app/start/prerequisites/). A person installing a finished build does not install Node or Rust.

```bash
npm install
npm run tauri dev
```

The window talks to a Vite dev server on port 1420. If that port is already in use, quit the other chess-app window first. react-chess uses port 5173, so the two can run together.

## Install on this Mac

This builds the app and copies it to `/Applications`. The installed copy does not need Node or Rust.

```bash
script/install-macos.sh
```

The built app is also at `src-tauri/target/release/bundle/macos/chess-app.app`, inside a disk image in the same folder. It is ad-hoc signed, not notarized, so the first open uses Control-click, then Open.

On Windows, `npm run tauri build` writes an installer under `src-tauri/target/release/bundle/nsis/`. On Linux it writes `.deb`, `.rpm`, and `.AppImage` packages under `src-tauri/target/release/bundle/`.

## Tests

```bash
npm run build
npm test
```

`npm run build` typechecks the window and produces the frontend bundle. `npm test` runs the Rust tests.

GitHub Actions runs both on every push to `master` and on every pull request.

## Releases

Pushing a tag `vX.Y.Z` (optional pre-release suffix, such as `v0.0.1-beta.1`) runs the release workflow. It builds bundles for macOS (Apple silicon and Intel), Windows, and Linux, then publishes one GitHub release.

These bundles are not signed with an Apple Developer ID and are not Authenticode-signed. macOS Gatekeeper and Windows SmartScreen will warn. The macOS app is ad-hoc signed (`signingIdentity` is `-`) so a download is not reported as damaged. Platform signing is a later release.

The release also contains `SHA256SUMS` and `SHA256SUMS.bundle`. The bundle is a keyless [cosign](https://docs.sigstore.dev/cosign/signing/overview/) signature from the release workflow. To check a download, put the bundles, `SHA256SUMS`, and `SHA256SUMS.bundle` in one directory and run:

```bash
cosign verify-blob \
  --bundle SHA256SUMS.bundle \
  --certificate-identity "https://github.com/RumenDamyanov/chess-app/.github/workflows/release.yml@refs/tags/vX.Y.Z" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  SHA256SUMS

sha256sum -c SHA256SUMS
```

Replace `vX.Y.Z` with the tag you downloaded. `sha256sum -c` checks each file named in `SHA256SUMS`.

Before tagging, set the same version in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`.

The release workflow is added with the first tag. Until then this section describes the intended check.

## License

MIT. Copyright 2026 Rumen Damyanov. See [LICENSE.md](LICENSE.md).
