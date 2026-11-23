#!/usr/bin/env bash
# Build, sign, package and publish a macOS Round release by hand.
#
# CI does this on a `Round/v*` tag (.github/workflows/Build.yml); this script is
# the local equivalent, for a signed build produced on a machine that holds the
# updater key.
#
# Prompts for:
#   - the Tauri signing-key password (hidden input)
#   - the GitHub release tag to upload assets to (e.g. Round/v0.0.9)
#
# Requires: pnpm, gh (authenticated), codesign, ditto. Round.key must be present
# in the repo root and remain gitignored.

set -euo pipefail

cd "$(dirname "$0")"

KEY_FILE="Round.key"

if [[ ! -f "$KEY_FILE" ]]; then
	echo "Error: $KEY_FILE not found in $(pwd)." >&2
	exit 1
fi

# --- Prompt for signing-key password (silent) -------------------------------
printf "Tauri signing key password: "
read -rs TAURI_SIGNING_PRIVATE_KEY_PASSWORD
printf "\n"

if [[ -z "$TAURI_SIGNING_PRIVATE_KEY_PASSWORD" ]]; then
	echo "Error: empty password." >&2
	exit 1
fi

# --- Prompt for release tag --------------------------------------------------
DEFAULT_TAG="Round/v$(node -p "require('./package.json').version" 2>/dev/null || echo "0.0.0")"
printf "Release tag [%s]: " "$DEFAULT_TAG"
read -r TAG
TAG="${TAG:-$DEFAULT_TAG}"

# --- Build (signed) ----------------------------------------------------------
# The Tauri CLI runs from the repository root: it finds Crate/Round's
# tauri.conf.json by walking down and treats the root as the frontend directory.
#
# `dmg` is bundled by driving Finder through AppleScript, so the process that
# runs this script needs macOS Automation permission for Finder. Without it the
# dmg step dies with `Finder got an error: Can't set toolbar visible ... (-10006)`
# *after* Round.app has been built and signed - grant the permission to your
# terminal (System Settings -> Privacy & Security -> Automation) or build with
# `--bundles app` and package the dmg yourself.
# shellcheck disable=SC2155
export TAURI_SIGNING_PRIVATE_KEY="$(cat "$KEY_FILE")"
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD

pnpm tauri build --bundles app,dmg

# --- Verify the signature does NOT carry the runtime flag --------------------
APP="Target/release/bundle/macos/Round.app"

if [[ ! -d "$APP" ]]; then
	echo "Error: $APP not found after build." >&2
	exit 1
fi

CODE_LINE=$(codesign -dvv "$APP" 2>&1 | grep CodeDirectory || true)
echo "$CODE_LINE"

if echo "$CODE_LINE" | grep -q "runtime"; then
	echo "Error: bundle is hardened-runtime signed - Apple Silicon will reject the download." >&2
	echo "Set bundle.macOS.hardenedRuntime to false in Crate/Round/tauri.conf.json and rebuild." >&2
	exit 1
fi

# --- Package with ditto so xattrs / symlinks / +x survive --------------------
BUNDLE_DIR="Target/release/bundle"
ZIP="$BUNDLE_DIR/macos/Round.app.zip"
# shellcheck disable=SC2012
DMG=$(ls "$BUNDLE_DIR/dmg"/Round_*_aarch64.dmg 2>/dev/null | head -n1 || true)

if [[ -z "$DMG" ]]; then
	echo "Error: no Round_*_aarch64.dmg found under $BUNDLE_DIR/dmg." >&2
	exit 1
fi

rm -f "$ZIP"

(
	cd "$BUNDLE_DIR/macos"
	ditto -c -k --keepParent Round.app Round.app.zip
)

# --- Clean up existing macOS assets on the release --------------------------
# Removes anything that looks like a macOS artifact (.app, .app.zip, .app.tar.gz,
# .dmg, .pkg) so we re-upload a clean set rather than accumulating stale names.
EXISTING_ASSETS=$(gh release view "$TAG" --json assets --jq '.assets[].name' 2>/dev/null || true)

if [[ -n "$EXISTING_ASSETS" ]]; then
	while IFS= read -r ASSET; do
		case "$ASSET" in
		*.app | *.app.zip | *.app.tar.gz | *.dmg | *.pkg)
			echo "Deleting existing asset: $ASSET"
			gh release delete-asset "$TAG" "$ASSET" --yes
			;;
		esac
	done <<<"$EXISTING_ASSETS"
fi

# --- Upload (clobber existing assets) ----------------------------------------
gh release upload "$TAG" --clobber "$ZIP" "$DMG"

echo
echo "Done. Uploaded:"
echo "  $ZIP"
echo "  $DMG"
echo "to release: $TAG"
