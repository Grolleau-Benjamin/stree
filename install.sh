#!/usr/bin/env bash
set -euo pipefail

REPO="${REPO:-Grolleau-Benjamin/arbor}"
BIN="${BIN:-arbor}"
DEST="${DEST:-/usr/local/bin}"
VERSION="${VERSION:-latest}"
TMP="$(mktemp -d)"

log(){ printf '\033[1;34m[arbor-install]\033[0m %s\n' "$*"; }

uname_s="$(uname -s)"
uname_m="$(uname -m)"
case "$uname_s" in
  Linux)  os_musl="unknown-linux-musl"; os_gnu="unknown-linux-gnu";;
  Darwin) os_musl="apple-darwin"; os_gnu="apple-darwin";;
  *) echo "Unsupported OS: $uname_s" >&2; exit 1;;
esac
case "$uname_m" in
  x86_64|amd64) arch="x86_64";;
  arm64|aarch64) arch="aarch64";;
  *) echo "Unsupported arch: $uname_m" >&2; exit 1;;
esac

resolve_latest_tag() {
  local t
  t="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | tr -d '\r' \
        | awk -F'"' '/"tag_name":/ { print $4; exit }' || true)"
  if [ -z "${t:-}" ]; then
    t="$(curl -fsSL -o /dev/null -w '%{redirect_url}\n' \
      "https://github.com/${REPO}/releases/latest" | awk -F/ '{print $NF}')"
  fi
  echo "$t"
}

if [ "$VERSION" = "latest" ]; then
  TAG="$(resolve_latest_tag)"
  [ -n "$TAG" ] || { echo "No releases found for ${REPO}" >&2; exit 1; }
else
  if [[ "$VERSION" =~ ^v ]]; then TAG="$VERSION"; else TAG="v$VERSION"; fi
fi

log "Using tag: $TAG"

assets_json="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/tags/${TAG}")" || {
  echo "Release ${TAG} not found on ${REPO}" >&2; exit 1;
}

candidates=()
candidates+=("${BIN}-${TAG}-${arch}-${os_musl}.tar.gz")
candidates+=("${BIN}-${TAG#v}-${arch}-${os_musl}.tar.gz")
if [ "$uname_s" = "Linux" ]; then
  candidates+=("${BIN}-${TAG}-${arch}-${os_gnu}.tar.gz")
  candidates+=("${BIN}-${TAG#v}-${arch}-${os_gnu}.tar.gz")
fi

asset_url=""
for c in "${candidates[@]}"; do
  url="$(echo "$assets_json" | tr -d '\r' | awk -v name="$c" -F'"' '
    $2=="name" && $4==name {found=1}
    found && $2=="browser_download_url" {print $4; exit}')"
  if [ -n "${url:-}" ]; then
    asset_url="$url"
    asset_name="$c"
    break
  fi
done

if [ -z "${asset_url:-}" ]; then
  echo "❌ No matching asset found for tag ${TAG}."
  echo "Tried:"
  printf ' - %s\n' "${candidates[@]}"
  exit 1
fi

log "Downloading ${asset_name}"
curl -fL "$asset_url" -o "$TMP/${asset_name}"

log "Extracting..."
tar -xzf "$TMP/${asset_name}" -C "$TMP"

SRC="$TMP/$BIN"; [ -x "$SRC" ] || SRC="$TMP/pack/$BIN"
[ -x "$SRC" ] || { echo "Binary not found in archive" >&2; exit 1; }

if [ ! -w "$DEST" ]; then
  log "Installing with sudo to $DEST"
  sudo install -m 0755 "$SRC" "$DEST/$BIN"
else
  install -m 0755 "$SRC" "$DEST/$BIN"
fi

echo "✅ Installed: $DEST/$BIN"
echo "Try: $BIN --version  |  $BIN --help"
