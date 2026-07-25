// Generates BC1-BC7 golden fixtures by round-tripping known input blocks
// through the adjacent Compressonator block codecs.
//
// Build & run:
//   tools/gen_bcn_golden.sh
//
// The harness writes, for each fixture, a `<name>.bcn` (the raw compressed
// block payload, the decoder input) and a `<name>.rgba` (the tightly packed
// RGBA8 golden output). Tests only consume the data; this program is not a
// runtime dependency.

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <cmath>

#include "cmp_core.h"
#include "bc6h/bc6h_decode.h"

using namespace HDR_Encode;

// IEEE-754 binary16 -> binary32 conversion, matching the `half` crate's
// `f16::from_bits(bits).to_f32()` so the harness reproduces the Rust decoder's
// clamp/round behaviour exactly.
static float half_to_float(unsigned short bits) {
    uint32_t sign = (bits >> 15) & 1;
    uint32_t exp = (bits >> 10) & 0x1f;
    uint32_t mant = bits & 0x3ff;
    uint32_t f;
    if (exp == 0) {
        if (mant == 0) {
            f = sign << 31;
        } else {
            // subnormal: normalize.
            int e = -1;
            do { e++; mant <<= 1; } while ((mant & 0x400) == 0);
            mant &= 0x3ff;
            f = (sign << 31) | ((127 - 15 - e) << 23) | (mant << 13);
        }
    } else if (exp == 0x1f) {
        f = (sign << 31) | (0xff << 23) | (mant << 13); // inf/nan
    } else {
        f = (sign << 31) | ((exp + (127 - 15)) << 23) | (mant << 13);
    }
    float result;
    memcpy(&result, &f, sizeof(result));
    return result;
}

static void write_file(const char* path, const void* data, size_t n) {
    FILE* f = fopen(path, "wb");
    if (!f) { perror(path); exit(1); }
    fwrite(data, 1, n, f);
    fclose(f);
    fprintf(stderr, "wrote %s (%zu bytes)\n", path, n);
}

// 4x4 RGBA8 gradient + solid blocks with varied content across all 16 texels.
static const unsigned char kSrcRGBA[16][4] = {
    {255,   0,   0, 255}, {  0, 255,   0, 255}, {  0,   0, 255, 255}, {255, 255,   0, 255},
    {255,   0, 255, 255}, {  0, 255, 255, 255}, {128,  64, 200, 255}, { 10,  20,  30, 255},
    {200, 100,  50, 255}, { 90, 160,  20, 255}, { 30,  30,  30, 255}, {220, 220, 220, 255},
    { 64, 128,  64, 255}, {180,  90, 240, 255}, { 15,  75, 135, 255}, {250,   5, 195, 255},
};

// A second source texture with smooth gradients for BC6H/BC7 coverage.
static const unsigned char kSrcRGBA2[16][4] = {
    {  0,   0,   0,   0}, { 32,  32,  32,  32}, { 64,  64,  64,  64}, { 96,  96,  96,  96},
    {128, 128, 128, 128}, {160, 160, 160, 160}, {192, 192, 192, 192}, {224, 224, 224, 224},
    { 10,  20,  30, 200}, { 40,  50,  60, 180}, { 70,  80,  90, 160}, {100, 110, 120, 140},
    {255,   0,   0, 255}, {  0, 255,   0, 255}, {  0,   0, 255, 255}, {255, 255, 255, 255},
};

// 16-bit RGB source for BC6H (half-float bits), spanning HDR > 1 values.
static const unsigned short kSrcBC6[16][3] = {
    {0x3c00, 0x3800, 0x0000}, // 1.0, 0.5, 0.0
    {0x4000, 0x3c00, 0x3800}, // 2.0, 1.0, 0.5
    {0x4400, 0x4000, 0x3c00}, // 4.0, 2.0, 1.0  (HDR)
    {0x0000, 0x0000, 0x0000}, // 0,0,0
    {0x3500, 0x3550, 0x3600}, // ~0.3
    {0x3b00, 0x3b80, 0x3c00}, // ~0.7
    {0x4200, 0x4100, 0x4000}, // HDR
    {0x4a00, 0x3800, 0x2c00}, // very HDR
    {0x3c00, 0x3c00, 0x3c00}, // 1,1,1
    {0x3200, 0x3600, 0x3a00}, // gradient
    {0x3e00, 0x3d00, 0x3c00},
    {0x4800, 0x4000, 0x3800},
    {0x3000, 0x3400, 0x3800},
    {0x3a00, 0x3c00, 0x3e00},
    {0x4400, 0x4200, 0x4000},
    {0x3500, 0x3700, 0x3900},
};

// A second BC6H source emphasizing smooth LDR gradients to exercise different
// mode/partition selections than the high-dynamic-range first source.
static const unsigned short kSrcBC6b[16][3] = {
    {0x3000, 0x3200, 0x3400}, {0x3200, 0x3400, 0x3600}, {0x3400, 0x3600, 0x3800},
    {0x3600, 0x3800, 0x3a00}, {0x3800, 0x3a00, 0x3c00}, {0x3a00, 0x3c00, 0x3e00},
    {0x3c00, 0x3c00, 0x3c00}, {0x3800, 0x3800, 0x3900}, {0x3400, 0x3500, 0x3600},
    {0x3000, 0x3200, 0x3400}, {0x2c00, 0x2e00, 0x3000}, {0x2800, 0x2a00, 0x2c00},
    {0x2400, 0x2600, 0x2800}, {0x2000, 0x2200, 0x2400}, {0x1c00, 0x1e00, 0x2000},
    {0x1800, 0x1a00, 0x1c00},
};

// Clamp-map a float to a u8 using the crate's BC6H convention.
static uint8_t clamp_u8(float f) {
    if (f <= 0.0f) return 0;
    if (f >= 1.0f) return 255;
    return (uint8_t)floorf(f * 255.0f + 0.5f);
}

int main(int argc, char** argv) {
    const char* out_dir = argc > 1 ? argv[1] : ".";
    char path[512];

    auto emit = [&](const char* name, const unsigned char* cmp, size_t cmp_len,
                    const unsigned char* rgba, size_t rgba_len) {
        snprintf(path, sizeof(path), "%s/%s.bcn", out_dir, name);
        write_file(path, cmp, cmp_len);
        snprintf(path, sizeof(path), "%s/%s.rgba", out_dir, name);
        write_file(path, rgba, rgba_len);
    };

    unsigned char cmp[16];
    unsigned char out64[64];

    // --- BC1 ---
    CompressBlockBC1(&kSrcRGBA[0][0], 16, cmp, nullptr);
    DecompressBlockBC1(cmp, out64, nullptr);
    emit("bc1-block", cmp, 8, out64, 64);

    // --- BC2 ---
    CompressBlockBC2(&kSrcRGBA[0][0], 16, cmp, nullptr);
    DecompressBlockBC2(cmp, out64, nullptr);
    emit("bc2-block", cmp, 16, out64, 64);

    // --- BC3 ---
    CompressBlockBC3(&kSrcRGBA[0][0], 16, cmp, nullptr);
    DecompressBlockBC3(cmp, out64, nullptr);
    emit("bc3-block", cmp, 16, out64, 64);

    // --- BC4 (single channel -> R only; output is 16 bytes) ---
    {
        unsigned char srcR[16];
        for (int i = 0; i < 16; i++) srcR[i] = kSrcRGBA[i][0];
        unsigned char out16[16];
        CompressBlockBC4(srcR, 4, cmp, nullptr);
        DecompressBlockBC4(cmp, out16, nullptr);
        unsigned char rgba[64];
        for (int i = 0; i < 16; i++) { rgba[i*4+0]=out16[i]; rgba[i*4+1]=0; rgba[i*4+2]=0; rgba[i*4+3]=255; }
        emit("bc4-block", cmp, 8, rgba, 64);
    }

    // --- BC5 (two channels -> R,G) ---
    {
        unsigned char srcR[16], srcG[16];
        for (int i = 0; i < 16; i++) { srcR[i] = kSrcRGBA[i][0]; srcG[i] = kSrcRGBA[i][1]; }
        unsigned char outR[16], outG[16];
        CompressBlockBC5(srcR, 4, srcG, 4, cmp, nullptr);
        DecompressBlockBC5(cmp, outR, outG, nullptr);
        unsigned char rgba[64];
        for (int i = 0; i < 16; i++) { rgba[i*4+0]=outR[i]; rgba[i*4+1]=outG[i]; rgba[i*4+2]=0; rgba[i*4+3]=255; }
        emit("bc5-block", cmp, 16, rgba, 64);
    }

    // --- BC6H UF16 / SF16 ---
    // Encode several source textures so the encoder selects a range of modes
    // and partition shapes, then decode each with both the unsigned and signed
    // reference paths. cmp_core's DecompressBlockBC6 always decodes UF16; the
    // signed path uses Compressonator's float BC6HBlockDecoder.
    auto emit_bc6 = [&](const char* name, const unsigned short src[16][3]) {
        unsigned short half48[48];
        for (int i = 0; i < 16; i++) {
            half48[i*3+0] = src[i][0];
            half48[i*3+1] = src[i][1];
            half48[i*3+2] = src[i][2];
        }
        CompressBlockBC6(half48, 3, cmp, nullptr);

        // UF16: cmp_core integer path (m_bc6signed=false). dec[] are signed
        // integers whose magnitude is the binary16 bit pattern.
        {
            unsigned short dec[48];
            DecompressBlockBC6(cmp, dec, nullptr);
            unsigned char rgba[64];
            for (int i = 0; i < 16; i++) {
                for (int c = 0; c < 3; c++) {
                    int v = (int)(int16_t)dec[i*3+c];
                    unsigned short bits = (unsigned short)(v < 0 ? -v : v);
                    float f = half_to_float(bits);
                    if (v < 0) f = -f;
                    rgba[i*4+c] = clamp_u8(f);
                }
                rgba[i*4+3] = 255;
            }
            char buf[64];
            snprintf(buf, sizeof(buf), "%s-uf16", name);
            emit(buf, cmp, 16, rgba, 64);
        }

        // SF16: Compressonator float decoder with bc6signed=true.
        {
            BC6HBlockDecoder dec;
            dec.bc6signed = true;
            float fout[16][4];
            dec.DecompressBlock(fout, cmp);
            unsigned char rgba[64];
            for (int i = 0; i < 16; i++) {
                for (int c = 0; c < 3; c++) rgba[i*4+c] = clamp_u8(fout[i][c]);
                rgba[i*4+3] = 255;
            }
            char buf[64];
            snprintf(buf, sizeof(buf), "%s-sf16", name);
            emit(buf, cmp, 16, rgba, 64);
        }
    };
    emit_bc6("bc6h-1", kSrcBC6);
    emit_bc6("bc6h-2", kSrcBC6b);

    // --- BC6H per-mode coverage ---
    // Craft one block per mode (only the mode bits set, rest zero) so every mode
    // 1..14 is exercised regardless of encoder choice. The all-zero payload only
    // proves mode *dispatch*; endpoint assembly, partition, and fixup index bugs
    // all happen to produce the same zero output, so they go undetected. We add a
    // second "-nz" (non-zero) block per mode that keeps the mode byte but fills
    // every endpoint/index/partition bit (bytes 1..15) with 0xFF. Any dropped or
    // mis-shifted data bit then changes the decoded palette and is caught.
    {
        const int decvals[15] = {0, 0x00, 0x01, 0x02, 0x06, 0x0a, 0x0e, 0x12,
                                 0x16, 0x1a, 0x1e, 0x03, 0x07, 0x0b, 0x0f};
        for (int mode = 1; mode <= 14; mode++) {
            // all-zero payload
            unsigned char zero[16] = {0};
            zero[0] = (unsigned char)decvals[mode];
            // non-zero payload: keep the mode byte, fill all data bits.
            unsigned char nz[16];
            memcpy(nz, zero, sizeof(zero));
            for (int i = 1; i < 16; i++) nz[i] = 0xFF;
            // Decode both payloads. Lambda decodes a single block to UF16/SF16
            // fixtures, named by `suffix` ("uf16"/"sf16") and `tag` (""/"-nz").
            auto decode_bc6_block = [&](const unsigned char block[16],
                                        const char* tag) {
                memcpy(cmp, block, 16);
                // UF16
                {
                    unsigned short dec[48];
                    DecompressBlockBC6(cmp, dec, nullptr);
                    unsigned char rgba[64];
                    for (int i = 0; i < 16; i++) {
                        for (int c = 0; c < 3; c++) {
                            int v = (int)(int16_t)dec[i*3+c];
                            unsigned short bits = (unsigned short)(v < 0 ? -v : v);
                            float f = half_to_float(bits);
                            if (v < 0) f = -f;
                            rgba[i*4+c] = clamp_u8(f);
                        }
                        rgba[i*4+3] = 255;
                    }
                    char buf[64];
                    snprintf(buf, sizeof(buf), "bc6h-mode%d%s-uf16", mode, tag);
                    emit(buf, cmp, 16, rgba, 64);
                }
                // SF16
                {
                    BC6HBlockDecoder dec;
                    dec.bc6signed = true;
                    float fout[16][4];
                    dec.DecompressBlock(fout, cmp);
                    unsigned char rgba[64];
                    for (int i = 0; i < 16; i++) {
                        for (int c = 0; c < 3; c++) rgba[i*4+c] = clamp_u8(fout[i][c]);
                        rgba[i*4+3] = 255;
                    }
                    char buf[64];
                    snprintf(buf, sizeof(buf), "bc6h-mode%d%s-sf16", mode, tag);
                    emit(buf, cmp, 16, rgba, 64);
                }
            };
            decode_bc6_block(zero, "");
            decode_bc6_block(nz, "-nz");
        }
    }

    // --- BC7 ---
    // Several source textures to exercise modes 0-7, p-bits, and partitions.
    auto emit_bc7 = [&](const char* name, const unsigned char src[16][4]) {
        CompressBlockBC7(&src[0][0], 16, cmp, nullptr);
        DecompressBlockBC7(cmp, out64, nullptr);
        emit(name, cmp, 16, out64, 64);
    };
    emit_bc7("bc7-block-1", kSrcRGBA);
    emit_bc7("bc7-block-2", kSrcRGBA2);

    // --- BC7 per-mode coverage ---
    // One crafted block per mode (mode bit set, rest zero) so all 8 modes are
    // exercised regardless of encoder choice. As with BC6H the all-zero payload
    // only proves dispatch; a "-nz" variant fills every data bit after the mode
    // bits with 1s so endpoint, partition, p-bit, and index bugs surface.
    for (unsigned mode = 0; mode < 8; mode++) {
        // all-zero payload
        unsigned char zero[16] = {0};
        zero[0] = (unsigned char)(1u << mode);
        DecompressBlockBC7(zero, out64, nullptr);
        {
            char buf[64];
            snprintf(buf, sizeof(buf), "bc7-mode%u", mode);
            emit(buf, zero, 16, out64, 64);
        }
        // non-zero payload: keep the mode bit, fill all remaining bits.
        unsigned char nz[16];
        memcpy(nz, zero, sizeof(zero));
        // Clear any extra mode bits in byte 0 that could flip the mode, then
        // fill byte 0's upper bits plus all of bytes 1..15.
        for (unsigned i = mode + 1; i < 8; i++) nz[0] |= (unsigned char)(1u << i);
        for (int i = 1; i < 16; i++) nz[i] = 0xFF;
        DecompressBlockBC7(nz, out64, nullptr);
        {
            char buf[64];
            snprintf(buf, sizeof(buf), "bc7-mode%u-nz", mode);
            emit(buf, nz, 16, out64, 64);
        }
    }

    return 0;
}
