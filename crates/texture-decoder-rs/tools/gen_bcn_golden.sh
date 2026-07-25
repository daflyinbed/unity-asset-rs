#!/usr/bin/env bash
# Builds and runs the BCN golden fixture generator, producing the .bcn/.rgba
# pairs under tests/data/compressonator/.
#
# Prerequisite: the vendor/compressonator submodule must have its static
# libraries built (default: build/cli-release/lib/libCMP_*.a and
# build/test-release/lib/libCMP_Math.a).
set -euo pipefail

cd "$(dirname "$0")/.."
COMP=vendor/compressonator
LIBS="$COMP/build/cli-release/lib"
MATHLIB="$COMP/build/test-release/lib/libCMP_Math.a"

g++ -O2 -std=c++17 \
    -I"$COMP/cmp_core/source" \
    -I"$COMP/cmp_compressonatorlib" \
    -I"$COMP/cmp_framework/common" \
    tools/gen_bcn_golden.cpp \
    -Wl,--start-group \
    "$LIBS/libCMP_Compressonator.a" \
    "$LIBS/libCMP_Framework.a" \
    "$LIBS/libCMP_Common.a" \
    "$LIBS/libCMP_Core.a" \
    "$LIBS/libCMP_Core_SSE.a" \
    "$LIBS/libCMP_Core_AVX.a" \
    "$LIBS/libCMP_Core_AVX512.a" \
    "$MATHLIB" \
    -Wl,--end-group \
    -lstdc++ -lm \
    -o tools/gen_bcn_golden

mkdir -p tests/data/compressonator
tools/gen_bcn_golden tests/data/compressonator
