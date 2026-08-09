#!/usr/bin/env bash
# Builds and runs the Unity-fork Crunch golden fixture generator, producing the
# .crn / .bc pairs under tests/data/crunch_unity/.
#
# Prerequisite: the vendor/unity-crunch submodule (on the `unity` branch) must
# contain the built `crnlib/crunch` CLI (the Unity-fork compressor) and the
# decompressor header at vendor/unity-crunch/inc/crn_decomp.h. The reference
# transcoder is built from gen_crn_unity_golden.cpp against that header.
#
# Unlike gen_crn_golden.sh, this exercises the Unity fork's incompatible
# "reference encoding" bitstream and the ETC1/ETC2A formats it added.
set -euo pipefail

cd "$(dirname "$0")/.."
CR=vendor/unity-crunch
CLI="$CR/crnlib/crunch"
INC="$CR/inc"

if [[ ! -x "$CLI" ]]; then
    echo "$CLI not found; building it with tools/build_unity_crunch.sh"
    tools/build_unity_crunch.sh
fi
if [[ ! -f "$INC/crn_decomp.h" ]]; then
    echo "error: $INC/crn_decomp.h not found." >&2
    exit 1
fi

# 1. Build the reference transcoder.
g++ -O2 -std=c++17 -fpermissive -I"$INC" \
    tools/gen_crn_unity_golden.cpp -o tools/gen_crn_unity_golden

# 2. Generate the small source images (shared with the upstream crn fixtures).
python3 tools/gen_crn_source.py tests/data/crunch_unity

OUT=tests/data/crunch_unity

# 3. Compress the 2D sources, then transcode. One fixture per target format.
"$CLI" -quiet -file "$OUT/src-color-6x6.tga" -out "$OUT/dxt1-6x6.crn" \
    -DXT1 -fileformat crn -mipMode None -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden "$OUT/dxt1-6x6.crn" "$OUT/dxt1-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-6x6.tga" -out "$OUT/dxt5-6x6.crn" \
    -DXT5 -fileformat crn -mipMode None -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_unity_golden "$OUT/dxt5-6x6.crn" "$OUT/dxt5-6x6.bc"

"$CLI" -quiet -file "$OUT/src-color-6x6.tga" -out "$OUT/etc1-6x6.crn" \
    -ETC1 -fileformat crn -mipMode None -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden "$OUT/etc1-6x6.crn" "$OUT/etc1-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-6x6.tga" -out "$OUT/etc2a-6x6.crn" \
    -ETC2A -fileformat crn -mipMode None -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_unity_golden "$OUT/etc2a-6x6.crn" "$OUT/etc2a-6x6.bc"

"$CLI" -quiet -file "$OUT/src-color-6x6.tga" -out "$OUT/etc2-6x6.crn" \
    -ETC2 -fileformat crn -mipMode None -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden "$OUT/etc2-6x6.crn" "$OUT/etc2-6x6.bc"

"$CLI" -quiet -file "$OUT/src-color-6x6.tga" -out "$OUT/etc1s-6x6.crn" \
    -ETC1S -fileformat crn -mipMode None -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden "$OUT/etc1s-6x6.crn" "$OUT/etc1s-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-6x6.tga" -out "$OUT/etc2as-6x6.crn" \
    -ETC2AS -fileformat crn -mipMode None -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_unity_golden "$OUT/etc2as-6x6.crn" "$OUT/etc2as-6x6.bc"

# 4. Three-level cubemaps for each format. The DDS inputs store six faces in
# the standard +X, -X, +Y, -Y, +Z, -Z order.
"$CLI" -quiet -file "$OUT/src-color-cubemap-6x6.dds" \
    -out "$OUT/dxt1-cubemap-6x6.crn" -DXT1 -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/dxt1-cubemap-6x6.crn" "$OUT/dxt1-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-cubemap-6x6.dds" \
    -out "$OUT/dxt5-cubemap-6x6.crn" -DXT5 -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 \
    -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/dxt5-cubemap-6x6.crn" "$OUT/dxt5-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-color-cubemap-6x6.dds" \
    -out "$OUT/etc1-cubemap-6x6.crn" -ETC1 -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/etc1-cubemap-6x6.crn" "$OUT/etc1-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-cubemap-6x6.dds" \
    -out "$OUT/etc2a-cubemap-6x6.crn" -ETC2A -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 \
    -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/etc2a-cubemap-6x6.crn" "$OUT/etc2a-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-color-cubemap-6x6.dds" \
    -out "$OUT/etc2-cubemap-6x6.crn" -ETC2 -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/etc2-cubemap-6x6.crn" "$OUT/etc2-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-color-cubemap-6x6.dds" \
    -out "$OUT/etc1s-cubemap-6x6.crn" -ETC1S -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 -c 64 -s 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/etc1s-cubemap-6x6.crn" "$OUT/etc1s-cubemap-6x6.bc"

"$CLI" -quiet -file "$OUT/src-alpha-cubemap-6x6.dds" \
    -out "$OUT/etc2as-cubemap-6x6.crn" -ETC2AS -fileformat crn \
    -mipMode Generate -maxmips 3 -minmipsize 1 \
    -c 64 -s 64 -ca 64 -sa 64 -quality 128
tools/gen_crn_unity_golden \
    "$OUT/etc2as-cubemap-6x6.crn" "$OUT/etc2as-cubemap-6x6.bc"

echo "wrote fixtures under $OUT/"
ls -la "$OUT"/*.crn "$OUT"/*.bc
