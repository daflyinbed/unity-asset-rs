#!/usr/bin/env bash
# Builds and runs the BC6H bcdec golden fixture generator, producing the
# .bcn/.rgba pairs under tests/data/bcdec/ that pin the `+ 32` interpolation
# rounding.
#
# Prerequisite: the vendor/bcdec submodule must be checked out (it is a
# header-only C library; no build needed). bcdec is MIT/Apache-2.0 dual
# licensed.
set -euo pipefail

cd "$(dirname "$0")/.."
BCDEC=vendor/bcdec

cc -O2 -I"$BCDEC" tools/gen_bc6h_bcdec.c -lm -o tools/gen_bc6h_bcdec

mkdir -p tests/data/bcdec
# These two blocks straddle the `>> 6` rounding boundary in the endpoint
# interpolation: pixel 1 blue (UF16) and pixel 11 green (SF16) each differ by
# one between round-to-nearest and truncation. They were found by decoding
# random blocks with bcdec and filtering for that single-byte divergence.
UF16_BLOCK=27be2780a7eb940e30de8e7c161bcd0a
SF16_BLOCK=8a2f8952f184cd71331acc032d5d6f16

tools/gen_bc6h_bcdec 0 "$UF16_BLOCK" | {
    head -c 16 > tests/data/bcdec/bc6h-uf16-lerp-round.bcn
    tail -c 64 > tests/data/bcdec/bc6h-uf16-lerp-round.rgba
}
tools/gen_bc6h_bcdec 1 "$SF16_BLOCK" | {
    head -c 16 > tests/data/bcdec/bc6h-sf16-lerp-round.bcn
    tail -c 64 > tests/data/bcdec/bc6h-sf16-lerp-round.rgba
}
echo "Generated tests/data/bcdec/{bc6h-uf16-lerp-round,bc6h-sf16-lerp-round}.{bcn,rgba}"
