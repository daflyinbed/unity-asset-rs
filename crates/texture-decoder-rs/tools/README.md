# Development tools

This directory contains generators for committed decoder tables and test
fixtures. They are development-only tools: neither the Rust library nor its
test suite invokes the external reference projects at run time.

## Generated BC6H and BC7 tables

`gen_bcn_tables.py` reads BC6H and BC7 reference tables from the
`vendor/compressonator` submodule and writes `src/bcn/tables.rs`. It extracts
partition patterns, fixup anchors, interpolation weights, and mode
descriptors.

Run it after updating the reference tables:

```sh
python3 tools/gen_bcn_tables.py
```

Review and commit the resulting change to `src/bcn/tables.rs`.

## Compressonator golden fixtures

`gen_bcn_golden.cpp` builds representative BC1--BC7 blocks, round-trips them
through AMD Compressonator, and emits a pair of files for each fixture:

- `*.bcn`: raw compressed decoder input;
- `*.rgba`: tightly packed RGBA8 output from the reference decoder.

`gen_bcn_golden.sh` compiles and runs that program, writing fixtures to
`tests/data/compressonator/`:

```sh
tools/gen_bcn_golden.sh
```

It requires the `vendor/compressonator` submodule with the static libraries
built at the paths expected by the script. The compiled `tools/gen_bcn_golden`
binary is ignored by Git and may be removed at any time.

## Independent BC6H rounding fixtures

`gen_bc6h_bcdec.c` uses the independent header-only `bcdec` decoder to produce
BC6H reference output. `gen_bc6h_bcdec.sh` compiles and runs it, writing two
fixtures under `tests/data/bcdec/`:

```sh
tools/gen_bc6h_bcdec.sh
```

This requires the `vendor/bcdec` submodule. The fixtures deliberately hit
the BC6H interpolation rounding boundary, where truncation and round-to-nearest
differ by one byte. They protect the decoder's `+ 32` rounding behavior for
both unsigned and signed half-float BC6H data. The generated
`tools/gen_bc6h_bcdec` executable is also ignored by Git.

## Crunch (CRN) golden fixtures

`gen_crn_source.py` writes small 6x6 TGA images and six-face legacy DDS
cubemaps (no third-party dependencies). `gen_crn_golden.cpp` transcodes every
face and mip level of a Crunch (CRN v1.04) file to raw BC1/BC3 blocks using the
header-only reference decompressor `crn_decomp.h`. The `.bc` output is
face-major and mip-minor, matching `CrnTexture::faces[face].levels[level]`.

`gen_crn_golden.sh` ties them together: it synthesizes the sources, compresses
2D and three-mip cubemap fixtures to DXT1 and DXT5 CRN with the
`vendor/crunch` CLI, then transcodes each to a `.bc` golden under
`tests/data/crunch/`:

```sh
tools/gen_crn_golden.sh
```

It requires the `vendor/crunch` submodule with the `crnlib/crunch` CLI built
and the `crn` cargo feature enabled for the consuming tests. Cubemap faces use
the standard `+X, -X, +Y, -Y, +Z, -Z` order. The generated
`tools/gen_crn_golden` executable is ignored by Git.

## Unity-fork Crunch (CRN) golden fixtures

`gen_crn_unity_golden.cpp` / `gen_crn_unity_golden.sh` are the Unity-fork
counterparts of the CRN generators above. The Unity fork (the
`vendor/unity-crunch` submodule, `unity` branch) replaced upstream Crunch's "chunk encoding"
bitstream with an incompatible "reference encoding" scheme and added ETC1/ETC2A
support, so its fixtures live under `tests/data/crunch_unity/` and are consumed
by the `crn-unity` cargo feature.

The script reuses `gen_crn_source.py` for the 6×6 sources and the
`vendor/unity-crunch/crnlib/crunch` CLI to compress DXT1, DXT5, ETC1
(ETC_RGB4), ETC2 (ETC2_RGB8), ETC2A (ETC2_RGBA8), and the ETC1S/ETC2AS
single-subblock variants — both 2D and three-mip cubemaps — then transcodes
each to a `.bc` golden in the natural tightly-packed block layout:

```sh
tools/gen_crn_unity_golden.sh
```

It requires the `vendor/unity-crunch` submodule. If its CLI is absent, the
generator runs `tools/build_unity_crunch.sh` automatically. That helper copies
the pinned submodule to a temporary directory, applies
`tools/patches/unity-crunch-modern-gcc.patch`, builds there, and copies only the
ignored `crnlib/crunch` executable back into the vendor directory. The patch
adds the compatibility fixes needed by current 64-bit GCC without modifying or
forking the checked-out vendor sources.

To build the CLI without regenerating fixtures:

```sh
tools/build_unity_crunch.sh
```

Set `JOBS` to override the default `nproc` parallelism. The generated
`tools/gen_crn_unity_golden` executable is ignored by Git.
