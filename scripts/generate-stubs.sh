#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"
OPENAPI_DIR="${ROOT_DIR}/incoming/kani_openapi"

IMAGE_DOWNLOAD="$OPENAPI_DIR/src/apis/image_download.rs"
SERVER="$OPENAPI_DIR/src/server/mod.rs"

OPENAPI_GENERATOR_VERSION=7.21.0
OPENAPI_GENERATOR_DOWNLOAD_CACHE_DIR="$SCRIPT_DIR"

export OPENAPI_GENERATOR_VERSION
export OPENAPI_GENERATOR_DOWNLOAD_CACHE_DIR

bash "$SCRIPT_DIR/openapi-generator-cli.sh" generate \
  -i "$ROOT_DIR/openapi.yaml" \
  -o "$OPENAPI_DIR" \
  -g rust-axum \
  -p packageName=kani-openapi

# Add content_type String to the 200 image response variants
sed -i 's/^        body: ByteArray,$/        body: ByteArray,\n        content_type: String,/' "$IMAGE_DOWNLOAD"
# Destructure content_type in server handlers
sed -i '/apis::image_download::[A-Za-z]*Response::[A-Za-z]*/{n;n; s/\([[:space:]]\+\)body/\1body,\n\1content_type/}' "$SERVER"
# Use runtime content_type instead of hardcoded "image/jpeg"
sed -i 's/HeaderValue::from_static("image\/jpeg")/HeaderValue::from_str(\&content_type).unwrap_or_else(|_| HeaderValue::from_static("application\/octet-stream"))/' "$SERVER"
