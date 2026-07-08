# AGENTS.md

Compact guidance for OpenCode sessions working on `unity-asset-rs`.

## What this repo is

A Rust library + CLI for parsing Unity AssetBundles (UnityFS format). Long-term
goal: a typed, high-performance UnityPy/AssetStudio alternative with Python
bindings. **Current state is an MVP** — only the file-loading + decompression
layer exists; there is no typed object parsing yet.

## Critical: docs describe the target, code is the truth

`docs/` (ADRs + `file-loading-layer.md`, written in Chinese) describes the
**planned** architecture. The current code does NOT match it:

- ADR-0001 (typed object parsing via offline TPK codegen) — **not implemented**
- ADR-0002 (pyo3 Python bindings) — **not implemented**
- ADR-0003 (file loading + decompression) — **only this is implemented**, and
  only partially: UnityFS only. No UnityWeb / UnityRaw / WebFile / gzip / brotli
  / CN-encryption / recursive re-dispatch yet.

The docs propose a workspace (`crates/unity-asset`, `crates/generated`,
`crates/codegen`, `crates/pyo3`). The actual repo is a **single flat crate**
with `src/{lib,main,bundle,decompress,error}.rs`. When docs and code conflict,
trust the code.

## Commands

```bash
cargo build                       # build lib + bin
cargo test                        # all tests (one integration file)
cargo test load_from_path         # run a single test by name
cargo test --test parse_bundle    # run just the integration test file
cargo run                         # bin defaults to example/char_002_amiya.ab
cargo run -- path/to/x.bundle     # parse a specific bundle
cargo clippy --all-targets        # no clippy.toml; default lints
```

No CI, no pre-commit hooks, no rustfmt.toml, no task runner — plain cargo only.
Edition is `2024` → requires Rust >= 1.85.

## Tests & fixtures

- All tests live in `tests/parse_bundle.rs` (integration; no unit tests yet).
- They require the committed fixture `example/char_002_amiya.ab` — do not
  delete or modify it.
- Expected byte values (`EXPECTED_FIRST16`, `EXPECTED_LAST16`, sizes, revision)
  were cross-checked **bit-for-bit against UnityPy**. Changing them means you
  changed parser semantics; only update with a verified reference output.
- Tests init `env_logger` at `debug`; set `RUST_LOG=trace` to see block /
  blocks-info decode detail.

## Architecture notes that aren't obvious from filenames

- **`src/bundle.rs`** is the UnityFS parser. It uses a private big-endian
  `Cursor` with explicit bounds checks — Unity bundles are big-endian on disk.
  Don't swap in `byteorder`; the inline cursor is intentional.
- **`src/decompress.rs`** exposes an *open* registry: `CompressionType(pub u16)`
  lets users register codecs at any flag slot, including hijacking standard
  flags (e.g. Arknights uses an LZ4 variant on flag 4/5 — see ADR-0003 / UnityPy
  issue #308). **Never hardcode game-specific codecs in the library**; that is
  exactly what the `Registry` extension point is for.
- `Registry::defaults()` registers **LZMA + LZ4 + LZ4HC + NONE** (UnityFS flags
  0-3). LZMA (flag 1) uses `lzma-rs` and the same `lzma_decompress_with_options`
  + `UseProvided` approach as `crates/codegen/src/tpk/compression.rs` — Unity's
  LZMA streams carry a 5-byte properties header but **no embedded unpacked size**,
  so the size must be supplied by the caller. Unity LZMA ≠ standard `.lzma`.
- Naming drift between code and doc: code uses `Registry` / `Decompressor`; the
  doc writes `DecompressorRegistry`. Code wins — don't rename to match the doc.
- `bytes::Bytes` is threaded through the pipeline for cheap Arc-shared slices.
  Public API hands out `Bytes`, not `Vec<u8>` — keep it that way.
- Public surface is deliberately tiny (`src/lib.rs`): `load` / `load_with` /
  `load_bytes` / `load_bytes_with` + `Bundle` / `BundleFile` / `Registry` /
  `Decompressor*`. `Bundle` here is just the unpacked archive (file names +
  decompressed bytes); it is **not** the `World` object-graph from the design
  doc — object parsing does not exist yet.
- Errors use `snafu`. Add variants in `src/error.rs` following the existing
  `#[snafu(display(...))]` + `#[snafu(visibility(pub))]` pattern.

## Conventions

- **Docstrings and inline comments are written in Chinese.** Match this when
  editing existing files; keep new code consistent with its neighbors.

## Reference docs (read before non-trivial work)

- `docs/adr/0003-file-loading-and-decompression-layer.md` — architecture the
  current code is building toward.
- `docs/file-loading-layer.md` — detailed design (describes target state; see
  the warning above).
- `docs/adr/0001-*.md`, `docs/adr/0002-*.md` — future object-parsing and
  Python-binding layers, not yet built.
