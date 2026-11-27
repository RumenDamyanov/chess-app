# ♟ chess-app

[![CI](https://github.com/RumenDamyanov/chess-app/actions/workflows/ci.yml/badge.svg)](https://github.com/RumenDamyanov/chess-app/actions/workflows/ci.yml)
[![Release](https://github.com/RumenDamyanov/chess-app/actions/workflows/release.yml/badge.svg)](https://github.com/RumenDamyanov/chess-app/actions/workflows/release.yml)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8D8?logo=tauri&logoColor=white)](https://v2.tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.90+-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

chess-app is a desktop chess program for macOS, Windows, and Linux. Play in the window with legal moves, clocks, a move list, and an opponent. The Rust engine is the default. A JavaScript engine is included as well. Both run inside the app.

The first installer release is not published yet. Until `v0.0.1-beta.1` is tagged, the way to open the board is a local build. The wiki pages below are the guides that release will use.

## Play

- [How to play](https://github.com/RumenDamyanov/chess-app/wiki/How-to-play) covers pieces, promotion, undo, clocks, and the AI.
- [Engines](https://github.com/RumenDamyanov/chess-app/wiki/Engines) explains the Rust and JavaScript choices in the window.
- Install when a release exists: [macOS](https://github.com/RumenDamyanov/chess-app/wiki/Install-on-macOS), [Windows](https://github.com/RumenDamyanov/chess-app/wiki/Install-on-Windows), [Linux](https://github.com/RumenDamyanov/chess-app/wiki/Install-on-Linux).
- [Check a download](https://github.com/RumenDamyanov/chess-app/wiki/Check-a-download) verifies `SHA256SUMS` before you install.

What the board already does in a development build:

- Castling, en passant, and under-promotion
- Move list, undo, board flip, and time controls
- An AI from random moves up to a depth-5 search
- Two engines in the same program, with no separate server to start

## Engines

| Engine | Where it runs |
|---|---|
| [rust-chess](https://github.com/RumenDamyanov/rust-chess) `rumenx-chess` 0.2 | Inside the app. This is the default. |
| [@rumenx/chess](https://www.npmjs.com/package/@rumenx/chess) 1.1.1 from [npm-chess](https://github.com/RumenDamyanov/npm-chess) | Inside the window. The Express server in that repository is not part of chess-app. |
| [go-chess](https://github.com/RumenDamyanov/go-chess) | Planned for a later version. |
| [react-chess](https://github.com/RumenDamyanov/react-chess) | The browser demo. The board in this app started there. |

More detail is on the wiki page [Related projects](https://github.com/RumenDamyanov/chess-app/wiki/Related-projects).

## Wiki

The wiki is the player guide. The pages exist locally and will be published to GitHub; the addresses are already fixed:

| Page | Address |
|---|---|
| Home | [wiki](https://github.com/RumenDamyanov/chess-app/wiki) |
| How to play | [How-to-play](https://github.com/RumenDamyanov/chess-app/wiki/How-to-play) |
| Engines | [Engines](https://github.com/RumenDamyanov/chess-app/wiki/Engines) |
| Related projects | [Related-projects](https://github.com/RumenDamyanov/chess-app/wiki/Related-projects) |
| Install on macOS | [Install-on-macOS](https://github.com/RumenDamyanov/chess-app/wiki/Install-on-macOS) |
| Install on Windows | [Install-on-Windows](https://github.com/RumenDamyanov/chess-app/wiki/Install-on-Windows) |
| Install on Linux | [Install-on-Linux](https://github.com/RumenDamyanov/chess-app/wiki/Install-on-Linux) |
| Check a download | [Check-a-download](https://github.com/RumenDamyanov/chess-app/wiki/Check-a-download) |
| Build on macOS | [Build-on-macOS](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-macOS) |
| Build on Windows | [Build-on-Windows](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-Windows) |
| Build on Linux | [Build-on-Linux](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-Linux) |
| Contribute | [Contribute](https://github.com/RumenDamyanov/chess-app/wiki/Contribute) |

## Project documents

- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)
- [Code of conduct](CODE_OF_CONDUCT.md)
- [Changelog](CHANGELOG.md)
- [Agents](AGENTS.md)
- [License](LICENSE.md)

## License

MIT. Copyright 2026 Rumen Damyanov. See [LICENSE.md](LICENSE.md).

The piece images are the Cburnett set by Colin M.L. Burnett, used under CC BY-SA 3.0 or GPL-2.0-or-later. See `public/assets/pieces/LICENSE`.

---

## Development

chess-app is one private repository. The default branch is `master`. The window is React and TypeScript in a Tauri 2 shell. Rust calls `rumenx-chess` in-process. The window calls `@rumenx/chess` for the JavaScript engine.

Building needs Node.js 22.23.4+, 24.22.1+, or 26.12.1+, Rust 1.90 or newer, and the system libraries [Tauri documents for your platform](https://v2.tauri.app/start/prerequisites/). A finished build is the app alone.

```bash
npm install
npm run tauri dev
```

The window uses a Vite server on port 1420. react-chess uses port 5173, so the two can run at the same time. Platform build notes are in the wiki: [macOS](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-macOS), [Windows](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-Windows), and [Linux](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-Linux).

On this Mac, `script/install-macos.sh` builds the app and copies it to `/Applications`. The copy is ad-hoc signed. The first open uses Control-click, then Open.

### Tests

```bash
npm run build
npm test
```

`npm run build` typechecks the window and produces the frontend bundle. `npm test` runs the window tests and the Rust tests. GitHub Actions runs both on every push to `master` and on every pull request.

### Releases

Pushing a tag `vX.Y.Z`, or a pre-release such as `v0.0.1-beta.1`, runs the release workflow. It builds bundles for macOS (Apple silicon and Intel), Windows, and Linux, then publishes one GitHub release with `SHA256SUMS` and a keyless [cosign](https://docs.sigstore.dev/cosign/signing/overview/) signature.

These bundles are not signed with an Apple Developer ID and are not Authenticode-signed. macOS Gatekeeper and Windows SmartScreen will warn. The macOS app is ad-hoc signed so a download is not reported as damaged. The check a player should run is [Check a download](https://github.com/RumenDamyanov/chess-app/wiki/Check-a-download).

Before tagging, set the same version in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`, and add the entry to [CHANGELOG.md](CHANGELOG.md). `v0.1.0` waits until an installed Mac build has finished a game against the Rust engine with no dev server running.

Pull requests, commit style, and what to leave untracked are in [CONTRIBUTING.md](CONTRIBUTING.md).
