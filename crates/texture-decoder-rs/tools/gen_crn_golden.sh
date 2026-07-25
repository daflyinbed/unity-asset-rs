#!/usr/bin/env bash
# Builds and runs the Crunch golden fixture generator, producing the .crn /
# .bc pairs under tests/data/crunch/.
#
# Prerequisite: the vendor/crunch submodule must contain the built
# `crnlib/crunch` CLI (the DXTn/C compressor) and the decompressor header at
# vendor/crunch/inc/crn_decomp.h. The reference transcoder is built from
# gen_crn_golden.cpp against that header.
set -euo pipefail

cd "$(dirname "$0")/.."
CR=vendor/crunch
CLI="$CR/crnlib/crunch"
INC="$CR/inc"

if [[ ! -x "$CLI" ]]; then
    echo "error: $CLI not found or not executable. Build the crunch CLI first." >&2
    exit 1
fi
if [[ ! -f "$INC/crn_decomp.h" ]]; then
    echo "error: $INC/crn_decomp.h not found." >&2
    exit 1
fi

# 1. Build the reference transcoder.
g++ -O2 -std=c++17 -fpermissive -I"$INC" \
    tools/gen_crn_golden.cpp -o tools/gen_crn_golden

# 2. Generate the small source images.
python3 tools/gen_crn_source.py tests/data/crunch

OUT=tests/data/crunch

# 3. Compress the 2D color source to DXT1 CRN, then transcode it.
"$CLI" -quiet -file "$OUT/src-color-6x6.tga" -out "$OUT/dxt1-6x6.crn" \
    -DXT1 -fileformat crn -mipMode None -c 64 -s 64 -quality 128
tools/gen_crn_golden "$OUT/dxt1-6x6.crn" "$OUT/dxt1-6x6.bc"

# 4. Compress the 2D alpha source to DXT5 CRN, then transcode it.
"$CLI" -quiet -file "$OUT/src-alpha-6x6.tga" -out "$OUT/dxt5-6x6.crn" \
    -DXT5 -fileformat crn -mipMode None -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_golden "$OUT/dxt5-6x6.crn" "$OUT/dxt5-6x6.bc"

# 5. Generate three-level DXT1 and DXT5 cubemap fixtures. The DDS inputs store
# six faces in the standard +X, -X, +Y, -Y, +Z, -Z order.
"$CLI" -quiet -file "$OUT/src-color-cubemap-6x6.dds" \
    -out "$OUT/dxt1-cubemap-6x6.crn" -DXT1 -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 -c 64 -s 64 -quality 128
tools/gen_crn_golden \
    "$OUT/dxt1-cubemap-6x6.crn" "$OUT/dxt1-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-cubemap-6x6.dds" \
    -out "$OUT/dxt5-cubemap-6x6.crn" -DXT5 -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 \
    -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_golden \
    "$OUT/dxt5-cubemap-6x6.crn" "$OUT/dxt5-cubemap-6x6.bc"

echo "wrote fixtures under $OUT/"
ls -la "$OUT"/*.crn "$OUT"/*.bc
