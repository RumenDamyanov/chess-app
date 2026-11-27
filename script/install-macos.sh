#!/usr/bin/env bash
# Build chess-app and copy it to /Applications.
# The installed app does not need Node or Rust.

set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"

npm run tauri build

if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  app="$CARGO_TARGET_DIR/release/bundle/macos/chess-app.app"
else
  app="$root/src-tauri/target/release/bundle/macos/chess-app.app"
fi
if [[ ! -d "$app" ]]; then
  echo "The build did not produce $app" >&2
  exit 1
fi

destination="/Applications/chess-app.app"
if [[ -e "$destination" ]]; then
  rm -rf "$destination"
fi

if ! ditto "$app" "$destination"; then
  echo "chess-app was built at:" >&2
  echo "  $app" >&2
  echo "Copy that app into /Applications, then open it." >&2
  exit 1
fi

echo "Installed $destination"
open "$destination"
