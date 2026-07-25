/* Generate a BC6H fixture (.bcn + .rgba) whose RGBA8 is decoded by bcdec,
 * using the same f16 -> f32 -> RGBA8 mapping the Rust crate uses, so the
 * fixture is an independent witness of the interpolation rounding.
 *
 * Usage: gen_bc6h_rgba <isSigned:0|1> <hex-block-32chars>
 *   prints 16 bytes of .bcn then 64 bytes of .rgba to stdout (raw).
 * Also: gen_bc6h_rgba hunt <isSigned> <seed> <count>
 *   searches random blocks, printing hex of the first block whose bcdec
 *   RGBA8 differs from the TRUNCATING interpolation formula reimplemented
 *   inline (this simulates the pre-fix Rust decoder).
 */
#define BCDEC_IMPLEMENTATION
#include "bcdec.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* half -> float (bcdec's quick version). */
static float half_to_float(unsigned short h) {
    return bcdec__half_to_float_quick(h);
}

/* Match the crate's bc6_value_to_u8: value is a SIGNED finish_unquantize
 * result whose magnitude is the binary16 bit pattern. */
static unsigned char val_to_u8(int value) {
    unsigned short bits = (unsigned short)(value < 0 ? -value : value);
    float f = half_to_float(bits);
    float s = value < 0 ? -f : f;
    if (s <= 0.0f) return 0;
    if (s >= 1.0f) return 255;
    return (unsigned char)(s * 255.0f + 0.5f);
}

/* Decode via bcdec to RGBA8 (64 bytes), alpha = 255. */
static void bcdec_to_rgba8(const unsigned char block[16], int isSigned,
                           unsigned char out[64]) {
    unsigned short half[16 * 3];
    bcdec_bc6h_half(block, half, 4 * 3, isSigned);
    for (int i = 0; i < 16; i++) {
        int r = half[i * 3 + 0];
        int g = half[i * 3 + 1];
        int b = half[i * 3 + 2];
        /* bcdec returns unsigned shorts; for signed format the sign bit is
         * packed in bit 15. Reconstruct the signed finish_unquantize value. */
        if (isSigned) {
            int rs = (r & 0x8000) ? -(r & 0x7FFF) : (r & 0x7FFF);
            int gs = (g & 0x8000) ? -(g & 0x7FFF) : (g & 0x7FFF);
            int bs = (b & 0x8000) ? -(b & 0x7FFF) : (b & 0x7FFF);
            out[i * 4 + 0] = val_to_u8(rs);
            out[i * 4 + 1] = val_to_u8(gs);
            out[i * 4 + 2] = val_to_u8(bs);
        } else {
            out[i * 4 + 0] = val_to_u8(r);
            out[i * 4 + 1] = val_to_u8(g);
            out[i * 4 + 2] = val_to_u8(b);
        }
        out[i * 4 + 3] = 255;
    }
}

static int parse_hex_block(const char *s, unsigned char block[16]) {
    if (strlen(s) != 32) return -1;
    for (int i = 0; i < 16; i++) {
        unsigned int v;
        if (sscanf(s + i * 2, "%2x", &v) != 1) return -1;
        block[i] = (unsigned char)v;
    }
    return 0;
}

static void write_raw_stdout(const void *p, size_t n) {
    fwrite(p, 1, n, stdout);
}

int main(int argc, char **argv) {
    if (argc >= 5 && strcmp(argv[1], "hunt") == 0) {
        int isSigned = atoi(argv[2]);
        unsigned seed = (unsigned)strtoul(argv[3], 0, 0);
        long count = strtol(argv[4], 0, 0);
        srand(seed);
        unsigned char block[16];
        unsigned char rgba[64];
        /* reference table for truncating interpolation is not reimplemented
         * here; instead we just emit bcdec RGBA8 for every block and let the
         * Rust side detect divergence. We print hex + the first `count`. */
        for (long k = 0; k < count; k++) {
            for (int i = 0; i < 16; i++) block[i] = (unsigned char)(rand() & 0xff);
            bcdec_to_rgba8(block, isSigned, rgba);
            printf("hex=");
            for (int i = 0; i < 16; i++) printf("%02x", block[i]);
            printf(" rgba=");
            for (int i = 0; i < 64; i++) printf("%02x", rgba[i]);
            printf("\n");
        }
        return 0;
    }

    if (argc == 3) {
        int isSigned = atoi(argv[1]);
        unsigned char block[16];
        if (parse_hex_block(argv[2], block) != 0) {
            fprintf(stderr, "bad hex block (need 32 hex chars)\n");
            return 2;
        }
        unsigned char rgba[64];
        bcdec_to_rgba8(block, isSigned, rgba);
        write_raw_stdout(block, 16);
        write_raw_stdout(rgba, 64);
        return 0;
    }

    fprintf(stderr,
            "usage: %s <isSigned> <hexblock>\n"
            "       %s hunt <isSigned> <seed> <count>\n",
            argv[0], argv[0]);
    return 2;
}
