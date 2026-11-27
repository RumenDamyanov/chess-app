# Contributing

Thank you for helping chess-app. By taking part you agree to the [code of conduct](CODE_OF_CONDUCT.md).

## What this project is

chess-app is one desktop app in one repository. The default branch is `master`. Pull requests target `master`.

A change should keep these properties:

- A person plays in the app window. The app does not start an Express or Axum server.
- The Rust engine is `rumenx-chess`, called in-process. The JavaScript engine is `@rumenx/chess`, called from the window.
- Go stays out until a later version. [go-chess](https://github.com/RumenDamyanov/go-chess) remains its own project.
- Rust tests use in-memory games. They do not bind a port.

The [README](README.md) is the product page. The player guide lives in the wiki, starting at [How to play](https://github.com/RumenDamyanov/chess-app/wiki/How-to-play). [AGENTS.md](AGENTS.md) is the short brief for coding agents.

## Set up

Install Node.js 22.23.4+, 24.22.1+, or 26.12.1+, Rust 1.90 or newer, and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your system. Then:

```bash
npm install
npm run tauri dev
```

Build notes for each system are in the wiki: [macOS](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-macOS), [Windows](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-Windows), and [Linux](https://github.com/RumenDamyanov/chess-app/wiki/Build-on-Linux).

Before you open a pull request:

```bash
npm run build
npm test
```

## Commits and pull requests

Use a [conventional commit](https://www.conventionalcommits.org/) subject: `feat`, `fix`, `chore`, `docs`, `refactor`, or `test`.

A pull request needs a short summary and a test plan. Say which operating system you ran, and whether the window still opens. If you did not run a check, say so.

Keep the change focused. A drive-by rewrite of an unrelated file is harder to review.

Do not commit `.ai/`, `wiki/`, `.cursor/`, `.vscode/`, secrets, `node_modules/`, or `target/`. `.gitignore` already excludes them. `package-lock.json` and `src-tauri/Cargo.lock` do belong in the commit. `wiki/` is the local source for the GitHub wiki, including the script that publishes it.

## Versions

`v0.1.0` waits until an installed Mac build has finished a game against the Rust engine with no dev server running. A beta tag such as `v0.0.1-beta.1` may exercise the release workflow before that.

When a release is ready, set the same version in all three places:

- `package.json`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`

Add the entry to [CHANGELOG.md](CHANGELOG.md), commit, and push a tag `vX.Y.Z` on `master`. The release workflow builds the bundles, writes `SHA256SUMS`, signs that file with keyless cosign, and publishes the GitHub release. Do not tag from a pull-request branch.

## Reporting issues

Security reports go through [SECURITY.md](SECURITY.md), not a public issue.

Questions about play belong in an issue, or on the wiki page [How to play](https://github.com/RumenDamyanov/chess-app/wiki/How-to-play). A local chess HTTP server, an account, and telemetry are outside this app.
