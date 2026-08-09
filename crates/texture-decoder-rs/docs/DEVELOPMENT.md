# Development guide

## Everyday checks

The normal test suite has no external tool dependencies. Run it with all
features so the allocating CRN decoder is included:

```sh
cargo fmt --check
cargo test --all-features --all-targets
cargo doc --all-features --no-deps
```

The library is `no_std`; integration tests use `std` solely for fixture loading
and test utilities.

## Repository layout

| Location | Contents |
| --- | --- |
| `src/astc/` | ASTC LDR decoder, lookup tables, and public API module. |
| `src/etc/` | ETC1, ETC2, and EAC decoder and public API module. |
| `src/bcn/` | BC1–BC7 decoder, lookup tables, and public API module. |
| `src/crn/` | Optional Crunch (CRN v1.04) to raw BC1/BC3 decoder. Enabled by the `crn` cargo feature, which pulls in `alloc`. |
| `src/error.rs` | Shared public `DecodeError` type. |
| `tests/*_end_to_end.rs` | Reference-fixture conformance tests. |
| `tests/data/` | Inputs and expected RGBA8 or raw BC reference output. |
| `tools/` | Programs used to regenerate selected fixtures. |

Keep each decoder family's `mod.rs` re-exports in sync whenever an entry point
is added or removed. The ASTC, ETC, and BC1–BC7 public decoders must retain the
crate-wide contract: raw block payload input, tightly packed row-major RGBA8
output, and `DecodeError` for all recoverable failures. The `crn` module is an
exception on both counts — it takes a compressed container and returns raw BC
block bytes (not RGBA8) inside `CrnTexture`, and reports `CrnDecodeError`.
`decode` returns the complete hierarchy as
`texture.faces[face].levels[level]`: 2D textures have one face, cubemaps use
`+X, -X, +Y, -Y, +Z, -Z` order, and level 0 is the largest mip. Every level's
buffer is tightly packed in row-major block order and contains physical BC1 or
BC3 blocks. Decoder state remains continuous while moving between faces within
one mip level, then resets for the next level. The `crn` feature requires
`alloc`; truly segmented files with out-of-line level payloads remain
unsupported.

## Test fixtures

Tests must not run reference encoders or decoders. Instead, commit their input
and golden output so `cargo test` remains repeatable and offline after Cargo
dependencies are available. See [`tests/data/README.md`](../tests/data/README.md)
for the provenance and licences of every fixture family.

Regeneration is only necessary when deliberately updating fixtures:

| Fixture family | Command | External prerequisite |
| --- | --- | --- |
| Compressonator BC1–BC7 | `tools/gen_bcn_golden.sh` | `vendor/compressonator` submodule with the libraries named in the script built. |
| Independent bcdec BC6H rounding cases | `tools/gen_bc6h_bcdec.sh` | `vendor/bcdec` submodule. |
| BC6H/BC7 lookup tables | `python3 tools/gen_bcn_tables.py` | Python 3. Review the generated diff before committing. |
| Crunch (CRN) DXT1/DXT5 | `tools/gen_crn_golden.sh` | `vendor/crunch` submodule with the `crnlib/crunch` CLI built. Needs the `crn` feature. |
| Unity-fork Crunch DXT1/DXT5/ETC1/ETC2/ETC2A/ETC1S/ETC2AS | `tools/gen_crn_unity_golden.sh` | `vendor/unity-crunch` submodule (`unity` branch). A missing CLI is built automatically in a temporary copy with the committed modern-GCC patch. Needs the `crn-unity` feature. |

The shell scripts create ignored executables in `tools/`. Review all changed
binary fixtures with the related tests, then run the everyday checks above.
