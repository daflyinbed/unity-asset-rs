#!/usr/bin/env python3
# Generates src/bcn/tables.rs from the vendor/compressonator submodule.
#
# Extracts the BC6H and BC7 constant tables (partitions, fixups, weights) and
# the mode descriptors, emitting them as Rust static arrays.  Run:
#
#     python3 tools/gen_bcn_tables.py
#
# The output file is committed; this script only needs to be re-run when the
# reference tables change.
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
COMP = os.path.join(ROOT, "vendor", "compressonator")


def read(path):
    with open(path) as f:
        return f.read()


def strip(src):
    src = re.sub(r"//[^\n]*", "", src)
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    return src


def extract_block(src, marker):
    """Return the textual contents between the outermost { } of `marker = { ... }`."""
    start = src.index(marker)
    depth = 0
    end = None
    for j in range(start, len(src)):
        c = src[j]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                end = j + 1
                break
    if end is None:
        raise RuntimeError(f"unterminated block for {marker!r}")
    block = src[start:end]
    first = block.index("{")
    return block[first + 1 : end - 1]


def nums(text):
    return [int(v) for v in re.findall(r"-?\d+", text)]


def fmt_array(values, per_row, indent="    "):
    lines = []
    for i in range(0, len(values), per_row):
        chunk = values[i : i + per_row]
        lines.append(indent + ", ".join(str(v) for v in chunk) + ",")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# BC6H partitions and fixups (cmp_core, the bit-decode path).
# ---------------------------------------------------------------------------
bc6_src = strip(read(os.path.join(COMP, "cmp_core/shaders/bc6_encode_kernel.cpp")))
bc6_part = nums(extract_block(bc6_src, "BC6_PARTITIONS[MAX_BC6H_PARTITIONS][MAX_SUBSET_SIZE] = {"))
assert len(bc6_part) == 32 * 16, len(bc6_part)

defs_src = strip(read(os.path.join(COMP, "cmp_compressonatorlib/bc6h/bc6h_definitions.h")))
index_fixups = nums(extract_block(defs_src, "g_indexfixups[32] = {"))
assert len(index_fixups) == 32, len(index_fixups)

# BC6H weights (shared by BC6H and BC7).
# g_aWeights3 = {0,9,18,27,37,46,55,64}; g_aWeights4 = 16-entry table.
weights3 = [0, 9, 18, 27, 37, 46, 55, 64]
weights4 = [0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64]
weights2 = [0, 21, 43, 64]

# ---------------------------------------------------------------------------
# BC7 partitions + fixups.
# ---------------------------------------------------------------------------
bc7_src = strip(read(os.path.join(COMP, "cmp_compressonatorlib/bc7/bc7_partitions.cpp")))
bc7_part = nums(extract_block(bc7_src, "BC7_PARTITIONS_CPU[MAX_SUBSETS][MAX_PARTITIONS][MAX_SUBSET_SIZE]"))
# Layout: subset0 has 1 partition (16), subset1/2 have 64 each (16) -> 1*16+64*16+64*16
assert len(bc7_part) == 16 + 64 * 16 + 64 * 16, len(bc7_part)
# Re-pack so subset 0 has 64 partitions (index 1..63 are copies of partition 0).
bc7_part_packed = []
# subset 0: a single all-zero partition replicated 64x
zero16 = [0] * 16
for _ in range(64):
    bc7_part_packed.extend(zero16)
# subset 1 (two subsets): values 16..16+64*16
bc7_part_packed.extend(bc7_part[16 : 16 + 64 * 16])
# subset 2 (three subsets): next 64*16
bc7_part_packed.extend(bc7_part[16 + 64 * 16 :])
assert len(bc7_part_packed) == 3 * 64 * 16

bc7_fix = nums(extract_block(bc7_src, "BC7_FIXUPINDICES[MAX_SUBSETS][MAX_PARTITIONS][3]"))
# Layout: subset0 {0,0,0} (3), subset1 64x{0,X} (128), subset2 64x{0,X,Y} (192)
assert len(bc7_fix) == 3 + 64 * 2 + 64 * 3, len(bc7_fix)
# Pack into [3][64][3] with subset0 replicated 64x as {0,0,0}.
bc7_fix_packed = []
for _ in range(64):
    bc7_fix_packed.extend([0, 0, 0])
bc7_fix_packed.extend(bc7_fix[3 : 3 + 64 * 2])  # subset1: pad each {0,X} -> {0,X,0}
# subset1 entries are only 2 values each; pad to 3.
sub1 = bc7_fix[3 : 3 + 64 * 2]
sub1_padded = []
for i in range(64):
    sub1_padded.extend([sub1[i * 2], sub1[i * 2 + 1], 0])
sub2 = bc7_fix[3 + 64 * 2 :]
assert len(sub2) == 64 * 3
bc7_fix_final = []
for _ in range(64):
    bc7_fix_final.extend([0, 0, 0])
bc7_fix_final.extend(sub1_padded)
bc7_fix_final.extend(sub2)
assert len(bc7_fix_final) == 3 * 64 * 3

# ---------------------------------------------------------------------------
# BC7 mode descriptors (bti table).
# ---------------------------------------------------------------------------
# {encodingType, partitionBits, rotationBits, indexModeBits, scalarBits,
#  vectorBits, pBitType, subsetCount, {index0Bits, index1Bits}}
bc7_modes = [
    # encoding(0=NO_ALPHA,1=COMBINED,2=SEPARATE), partitionBits, rotationBits,
    # indexModeBits, scalarBits, vectorBits, pbit(0=NONE,1=ONE,2=TWO),
    # subsetCount, index0Bits, index1Bits
    (0, 4, 0, 0, 0, 12, 2, 3, 3, 0),   # mode 0
    (0, 6, 0, 0, 0, 18, 1, 2, 3, 0),   # mode 1
    (0, 6, 0, 0, 0, 15, 0, 3, 2, 0),   # mode 2
    (0, 6, 0, 0, 0, 21, 2, 2, 2, 0),   # mode 3
    (2, 0, 2, 1, 6, 15, 0, 1, 2, 3),   # mode 4
    (2, 0, 2, 0, 8, 21, 0, 1, 2, 2),   # mode 5
    (1, 0, 0, 0, 0, 28, 2, 1, 4, 0),   # mode 6
    (1, 6, 0, 0, 0, 20, 2, 2, 2, 0),   # mode 7
]

# ---------------------------------------------------------------------------
# BC6H mode descriptors derived from extract_format / ModePartition table.
# Fields: w_bits, t_bits_r, t_bits_g, t_bits_b, transformed, two_region
# (mode index 1..14; mode 0 is reserved/invalid).
# ---------------------------------------------------------------------------
bc6_modes = [
    # (w_bits, t_bits[3], transformed, two_region) -- index by m_mode (1..14)
    None,                       # 0: invalid
    (10, 5, 5, 5, 1, 1),        # mode 1
    (7, 6, 6, 6, 1, 1),         # mode 2
    (11, 5, 4, 4, 1, 1),        # mode 3
    (11, 4, 5, 4, 1, 1),        # mode 4
    (11, 4, 4, 5, 1, 1),        # mode 5
    (9, 5, 5, 5, 1, 1),         # mode 6
    (8, 6, 5, 5, 1, 1),         # mode 7
    (8, 5, 6, 5, 1, 1),         # mode 8
    (8, 5, 5, 6, 1, 1),         # mode 9
    (6, 6, 6, 6, 0, 1),         # mode 10
    (10, 10, 10, 10, 0, 0),     # mode 11
    (11, 9, 9, 9, 1, 0),        # mode 12
    (12, 8, 8, 8, 1, 0),        # mode 13
    (16, 4, 4, 4, 1, 0),        # mode 14
]

# ---------------------------------------------------------------------------
# Emit Rust.
# ---------------------------------------------------------------------------
out = []
out.append("//! BC6H and BC7 constant tables.")
out.append("//!")
out.append("//! Generated by `tools/gen_bcn_tables.py` from the adjacent Compressonator")
out.append("//! reference (MIT licensed; see `LICENSE-MIT-COMPRESSONATOR`). The tables")
out.append("//! cover BC6H two-region partitions and index fixups, the shared 2/3/4-bit")
out.append("//! interpolation weights, and the BC7 mode descriptors, partition patterns,")
out.append("//! and fixup anchors.")
out.append("")
out.append("/// Description of one BC6H mode (indexed by decoded mode value 1..14).")
out.append("///")
out.append("/// `(w_bits, [t_bits_r, t_bits_g, t_bits_b], transformed, two_region)`.")
out.append("/// Entry 0 is `None` and marks the reserved mode.")
out.append("pub type Bc6Mode = Option<(u32, [u32; 3], bool, bool)>;")
out.append("")
out.append("pub static BC6_MODES: [Bc6Mode; 15] = [")
for i, m in enumerate(bc6_modes):
    if m is None:
        out.append("    None,")
    else:
        wb, tr, tg, tb, tf, two = m
        out.append(f"    Some(({wb}, [{tr}, {tg}, {tb}], {'true' if tf else 'false'}, {'true' if two else 'false'})),")
out.append("];")
out.append("")
out.append("/// BC6H two-region partition patterns: `[partition][pixel] -> subset`.")
out.append("pub static BC6_PARTITIONS: [[u8; 16]; 32] = [")
for p in range(32):
    vals = bc6_part[p * 16 : p * 16 + 16]
    out.append("    [" + ", ".join(str(v) for v in vals) + "],")
out.append("];")
out.append("")
out.append("/// Anchor index for the second subset of each BC6H two-region partition.")
out.append("pub static BC6_INDEX_FIXUPS: [usize; 32] = [")
for i in range(0, 32, 16):
    out.append("    " + ", ".join(str(v) for v in index_fixups[i : i + 16]) + ",")
out.append("];")
out.append("")
out.append("/// 3-bit (denominator 7) interpolation weights, shared by BC6H and BC7.")
out.append("pub static WEIGHTS3: [u32; 8] = [" + ", ".join(str(v) for v in weights3) + "];")
out.append("/// 4-bit (denominator 15) interpolation weights, shared by BC6H and BC7.")
out.append("pub static WEIGHTS4: [u32; 16] = [")
out.append("    " + ", ".join(str(v) for v in weights4[:8]) + ",")
out.append("    " + ", ".join(str(v) for v in weights4[8:]) + ",")
out.append("];")
out.append("")
out.append("/// 2-bit (denominator 3) interpolation weights, used by BC7.")
out.append("pub static WEIGHTS2: [u32; 4] = [" + ", ".join(str(v) for v in weights2) + "];")
out.append("")

# BC7 modes
out.append("/// BC7 mode descriptor.")
out.append("///")
out.append("/// `(encoding, partition_bits, rotation_bits, index_mode_bits, scalar_bits,")
out.append("/// vector_bits, p_bit_type, subset_count, [index0_bits, index1_bits])`.")
out.append("/// `encoding`: 0 = NO_ALPHA, 1 = COMBINED_ALPHA, 2 = SEPARATE_ALPHA.")
out.append("/// `p_bit_type`: 0 = NONE, 1 = ONE (shared), 2 = TWO (per subset).")
out.append("pub static BC7_MODES: [Bc7Mode; 8] = [")
for m in bc7_modes:
    enc, pb, rb, imb, sb, vb, pbit, sub, i0, i1 = m
    out.append(
        f"    Bc7Mode {{ encoding: {enc}, partition_bits: {pb}, rotation_bits: {rb}, "
        f"index_mode_bits: {imb}, scalar_bits: {sb}, vector_bits: {vb}, p_bit_type: {pbit}, "
        f"subset_count: {sub}, index_bits: [{i0}, {i1}] }},"
    )
out.append("];")
out.append("")
out.append("/// One BC7 mode descriptor entry.")
out.append("#[derive(Clone, Copy)]")
out.append("pub struct Bc7Mode {")
out.append("    pub encoding: u32,")
out.append("    pub partition_bits: u32,")
out.append("    pub rotation_bits: u32,")
out.append("    pub index_mode_bits: u32,")
out.append("    pub scalar_bits: u32,")
out.append("    pub vector_bits: u32,")
out.append("    pub p_bit_type: u32,")
out.append("    pub subset_count: u32,")
out.append("    pub index_bits: [u32; 2],")
out.append("}")
out.append("")
out.append("/// BC7 partition patterns: `[subset_count - 1][partition][pixel] -> subset`.")
out.append("///")
out.append("/// `subset_count == 1` blocks never read a partition pattern (all pixels are")
out.append("/// subset 0), so the `[0]` plane is filled with zeros for uniformity.")
out.append("pub static BC7_PARTITIONS: [[[u8; 16]; 64]; 3] = [")
for plane in range(3):
    out.append("    [")
    for p in range(64):
        vals = bc7_part_packed[plane * 64 * 16 + p * 16 : plane * 64 * 16 + p * 16 + 16]
        out.append("        [" + ", ".join(str(v) for v in vals) + "],")
    out.append("    ],")
out.append("];")
out.append("")
out.append("/// BC7 fixup anchors: `[subset_count - 1][partition][subset_index]`.")
out.append("///")
out.append("/// Fixup index 0 is always texel 0; entries give the anchor for subsets 1 and 2.")
out.append("pub static BC7_FIXUP_INDICES: [[[usize; 3]; 64]; 3] = [")
for plane in range(3):
    out.append("    [")
    for p in range(64):
        vals = bc7_fix_final[plane * 64 * 3 + p * 3 : plane * 64 * 3 + p * 3 + 3]
        out.append("        [" + ", ".join(str(v) for v in vals) + "],")
    out.append("    ],")
out.append("];")
out.append("")

dest = os.path.join(ROOT, "src", "bcn", "tables.rs")
with open(dest, "w") as f:
    f.write("\n".join(out))
print(f"wrote {dest}")
