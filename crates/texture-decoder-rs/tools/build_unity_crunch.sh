#!/usr/bin/env bash
# Build the pinned Unity Crunch submodule with a small modern-GCC compatibility
# patch, without making the submodule worktree dirty.
set -euo pipefail

cd "$(dirname "$0")/.."

VENDOR=vendor/unity-crunch
PATCH="$PWD/tools/patches/unity-crunch-modern-gcc.patch"
OUTPUT="$PWD/$VENDOR/crnlib/crunch"

if [[ ! -f "$VENDOR/crnlib/Makefile" || ! -f "$VENDOR/inc/crn_decomp.h" ]]; then
    echo "error: $VENDOR is missing; initialize the submodule first" >&2
    exit 1
fi

BUILD_DIR=$(mktemp -d "${TMPDIR:-/tmp}/unity-crunch-build.XXXXXX")
cleanup() {
    if [[ -n "${BUILD_DIR:-}" && -d "$BUILD_DIR" ]]; then
        rm -rf -- "$BUILD_DIR"
    fi
}
trap cleanup EXIT

cp -a "$VENDOR/." "$BUILD_DIR/"
rm -f "$BUILD_DIR/.git"
(
    cd "$BUILD_DIR"
    # The pinned submodule stores these sources as CRLF without an eol
    # attribute. Normalize only the patched copies so the repository patch
    # remains readable and portable.
    sed -i 's/\r$//' inc/crn_decomp.h crnlib/crn_vector.cpp
    git apply --no-index --check "$PATCH"
    git apply --no-index "$PATCH"
    make -C crnlib -j"${JOBS:-$(nproc)}"
)

cp "$BUILD_DIR/crnlib/crunch" "$OUTPUT"
chmod +x "$OUTPUT"
echo "built $OUTPUT"
