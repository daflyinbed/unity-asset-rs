# texture-decoder-rs

Small, `no_std`, allocation-free decoders for the common GPU block-compressed
texture formats: **ASTC (LDR)**, **ETC2 / EAC**, and **BC1–BC7**. The optional
`crn` and `crn-unity` features add allocating Crunch (CRN v1.04) container
decoders — the former for upstream Crunch files, the latter for the incompatible
Unity fork.

The ASTC, ETC/EAC, and BC decoders write tightly packed **RGBA8** in row-major
order. Each format provides a single-block entry point (`..._block`) and an
image-level entry point that walks a whole image and clips the right/bottom
edges for non-4-multiple dimensions. These entry points consume raw block
payloads directly rather than parsing PKM/KTX/DDS containers. The optional CRN
decoder is the exception: it reads a CRN container and returns raw physical
BC1/BC3 blocks, not RGBA8.

## Project layout

| Path | Purpose |
| --- | --- |
| `src/astc/`, `src/etc/`, `src/bcn/` | Decoder implementation and public API for each format family. |
| `src/crn/` | Optional Crunch container decoder; enabled by the `crn` feature. |
| `src/crn_unity/` | Optional Unity-fork Crunch container decoder; enabled by the `crn-unity` feature. |
| `tests/` | Unit and end-to-end conformance tests. |
| `tests/data/` | Committed compressed inputs and reference outputs. |
| [`tools/`](tools/README.md) | Optional generators for BC6H/BC7 tables and reference fixtures; not needed to build or test the crate. |
| `docs/DEVELOPMENT.md` | Local development and fixture-regeneration notes. |

## Supported formats

| Family | Entry points | Output convention |
| --- | --- | --- |
| ASTC LDR | `decode_ldr_{4_4 .. 12_12}` (+ `_block` variants) | RGBA8; 2D footprints from 4×4 to 12×12 |
| ETC1 | `decode_etc1_rgb8` | RGB, alpha = 255 |
| ETC2 | `decode_etc2_rgb8`, `decode_etc2_rgb8a1`, `decode_etc2_rgba8` | RGB + alpha as defined by the mode |
| EAC | `decode_eac_r11`, `decode_eac_rg11` (+ `signed` variants) | R11 → `[R,0,0,255]`, RG11 → `[R,G,0,255]`; signed values use SNORM |
| BC1 | `decode_bc1` | RGBA; three-color selector 3 = transparent black |
| BC2 | `decode_bc2` | RGBA; 4-bit alpha expanded to `value * 17` |
| BC3 | `decode_bc3` | RGBA; 8-entry alpha interpolation |
| BC4 | `decode_bc4` | `[R,0,0,255]` (UNORM only) |
| BC5 | `decode_bc5` | `[R,G,0,255]` (UNORM only) |
| BC6H | `decode_bc6h_uf16`, `decode_bc6h_sf16` | `[R,G,B,255]`; clamped `f16`→8-bit (see below) |
| BC7 | `decode_bc7` | the decoded RGBA |
| CRN (`crn` feature) | `crn::decode` | Complete 2D/cubemap mip hierarchy as raw BC1/BC3 blocks |
| CRN Unity (`crn-unity` feature) | `crn_unity::decode` | Complete 2D/cubemap mip hierarchy as raw BC1/BC3/ETC1/ETC2_RGB8/ETC2_RGBA8 blocks |

### Half-float → 8-bit mapping (BC6H)

BC6H endpoints are decoded to `f16`, then clamped to RGBA8 as
`<= 0 -> 0`, `>= 1 -> 255`, else `floor(value * 255 + 0.5)`. SF16 negatives clip
to 0; HDR values above 1 clip to 255.

### What this crate does *not* do

No sRGB, gamma, exposure, or tone mapping of any kind. BC4/BC5 SNORM is not
implemented. sRGB-tagged blocks use the matching RGB entry point, since decoding
preserves code values and performs no color-space conversion.

## Usage

```rust
use texture_decoder_rs::bcn::{decode_bc1, decode_bc1_block};

// One 4x4 block (8 bytes) -> 64 bytes of RGBA8.
let mut out = [0u8; 4 * 4 * 4];
decode_bc1_block(&block, &mut out)?;

// A whole image (non-4 dimensions are clipped at the right/bottom edge).
let mut image = vec![0u8; width * height * 4];
decode_bc1(&data, width, height, &mut image)?;
```

Errors are reported via the `DecodeError` enum:
`InputTooShort`, `OutputTooShort`, `SizeOverflow`, and `InvalidBlock`
(e.g. a BC6H reserved mode or a BC7 all-zero block).

Invalid physical blocks are rejected instead of being replaced with a reference
decoder's diagnostic color. In particular, reserved BC6H modes return
`InvalidBlock`, and invalid ASTC LDR or ETC1 differential blocks fail the image
decode. BC2 and BC3 always use their specified four-color interpolation mode;
they intentionally do not inherit Compressonator's BC1 three-color behavior
when `color0 <= color1`.

Public functions are grouped by format family: `texture_decoder_rs::astc::*`,
`texture_decoder_rs::etc::*`, and `texture_decoder_rs::bcn::*`. Enable the
`crn` cargo feature for `texture_decoder_rs::crn::*`.

### Crunch containers

`crn::decode` returns a `CrnTexture` containing every face and mip level:

```rust
use texture_decoder_rs::crn::decode;

let texture = decode(&crn_bytes)?;
let positive_x_base = &texture.faces[0].levels[0];
```

Use `texture.faces[face].levels[level]`; level 0 is the largest mip. A 2D
texture has one face, while a cubemap has six in `+X, -X, +Y, -Y, +Z, -Z`
order. Each level records its own width and height, and its `data` is tightly
packed in row-major block order. DXT1 produces raw BC1 blocks and DXT5-family
formats produce raw physical BC3 blocks; callers can pass these blocks to the
matching `bcn` decoder when RGBA8 is needed. CRN palette and output storage
requires `alloc`. Truly segmented CRN files whose level payloads are stored
out-of-line are not supported.

### Unity-fork Crunch containers

The `crn-unity` feature ports the **Unity fork** of Crunch, whose files use an
incompatible "reference encoding" bitstream (and add ETC1/ETC2A support), so
they cannot be read by the `crn` module above. `crn_unity::decode` supports
DXT1 (BC1), DXT5 (BC3), ETC_RGB4 (ETC1), and ETC2_RGBA8, returning the same
face/mip hierarchy shape:

```rust
use texture_decoder_rs::crn_unity::decode;

let texture = decode(&unity_crn_bytes)?;
let level = &texture.faces[0].levels[0];
```

ETC1 produces raw 8-byte ETC1 blocks and ETC2_RGBA8 produces raw 16-byte blocks
(8 alpha + 8 color); pass these to the matching `etc` decoder for RGBA8. Like
`crn`, it requires `alloc` and does not support truly out-of-line segmented
payloads.

### Input and output contract

- Image functions accept blocks in row-major block order. ASTC blocks are always
  16 bytes; ETC1/ETC2 RGB8/RGB8A1 and EAC R11 are 8 bytes; ETC2 RGBA8, EAC RG11,
  and BC2–BC7 are 16 bytes; BC1/BC4 are 8 bytes.
- `output` may be larger than required; only the `width * height * 4` leading
  bytes are written. A zero-width or zero-height image requires no input or
  output storage.
- Whole-image functions decode partial right and bottom blocks, writing only the
  texels that belong to the requested dimensions.

## Testing

The ASTC, ETC2/EAC, BC1–BC7, and optional CRN/Unity-CRN decoders are covered by
byte-exact end-to-end tests against independent reference decoders
(`astc-encoder`, ETCPACK, AMD Compressonator, `iOrange/bcdec`, and the Crunch /
Unity-fork Crunch reference decoders), all run hermetically at test time — the
references are only used to *generate* the committed golden fixtures. These
reference projects are pinned as git submodules under `vendor/`; fetch them
with `git submodule update --init --recursive` when regenerating fixtures. See
[`tests/data/README.md`](tests/data/README.md) for fixture provenance and
[`tools/README.md`](tools/README.md) for the generators, external prerequisites,
and regeneration commands.

## License

This crate is licensed under Apache-2.0. The committed BC6H/BC7 constant tables
and conformance fixtures are derived from AMD Compressonator, which is MIT
licensed; see [`LICENSE-MIT-COMPRESSONATOR`](LICENSE-MIT-COMPRESSONATOR).

The reference projects pinned as git submodules under `vendor/` are **not**
covered by this crate's license — each retains its own upstream license, which
ships with the submodule and applies to its contents:

| Submodule | License |
| --- | --- |
| `vendor/astc-encoder` | Apache-2.0 |
| `vendor/etcpack` | Ericsson ETCPACK SLA (see `vendor/etcpack/licence.txt`) |
| `vendor/compressonator` | MIT |
| `vendor/bcdec` | MIT or Unlicense (dual) |
| `vendor/crunch` | Public domain (BinomialLLC release) |
| `vendor/unity-crunch` | ZLIB |

These submodules are used only to regenerate the committed tables and fixtures;
they are not linked into the library or exercised at test time.

## Development

Run the full local validation suite with:

```sh
cargo fmt --check
cargo test --all-features --all-targets
cargo doc --all-features --no-deps
```

The checked-in test data keeps normal test runs hermetic. Instructions and
external prerequisites for regenerating it are in
[`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md).
