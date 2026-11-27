# Agents

chess-app is one desktop app in one repository. The default branch is `master`.

When `.ai/action-plan.md` is on the machine, it is the implementation sequence. Follow it in order. Do not start a later step until the earlier step’s check passes. That file is local. It is gitignored and is not part of the repository.

## Product rules

- The person plays in the app window. Do not add an Express server, an Axum server, or a requirement to start another chess process.
- The Rust engine is the `rumenx-chess` crate, called in-process from Tauri commands. The JavaScript engine is `@rumenx/chess`, called from the window.
- Rust is the default engine. JavaScript stays available in the engine menu. Go is a later version.
- The window is React, TypeScript, and Vite inside a Tauri 2 shell.
- Rust tests use in-memory games. No test binds a port or starts a chess server.
- Keep the depth-5 cap on the in-app Rust search. Harmless stays a random legal move.

## Repository rules

- Do not commit `.ai/`, `wiki/`, `.cursor/`, `.vscode/`, secrets, `node_modules/`, or `target/`. `wiki/` is the local source for the GitHub wiki, including the script that publishes it.
- The license is MIT. The text is `LICENSE.md`. Machine-readable license fields live in `package.json` and `src-tauri/Cargo.toml`.
- Keep `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json` on the same version. Do not tag `v0.1.0` until an installed Mac build has finished a game against the Rust engine with no dev server running.
- Tests run from `.github/workflows/ci.yml`. A tag `vX.Y.Z` runs `.github/workflows/release.yml`, which publishes unsigned bundles plus `SHA256SUMS` and a keyless cosign bundle. Do not add Apple notarization or Authenticode to that workflow.
- The macOS bundle uses an ad-hoc signing identity (`-`). Leave it that way until a later signing release.
- `rust-version` in `src-tauri/Cargo.toml` is `1.90`, so the locked `time` and `quick-xml` crates stay on patched releases.
- Conventional commits. Pull requests target `master`.

## Checks

```bash
npm run build
npm test
```

`npm run build` typechecks and builds the window. `npm test` runs the window tests, then `cargo test` for `src-tauri`. After a change to the window, run `npm run tauri dev` and confirm the window opens and a move can be played.
