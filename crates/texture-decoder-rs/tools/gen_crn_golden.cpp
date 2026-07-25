// gen_crn_golden.cpp
//
// Reads a Crunch (.crn) file, decompresses every face and mip level into raw
// DXTn/BC blocks using the reference crn_decomp.h, and writes the block bytes
// in face-major, mip-minor order. This is the golden reference for the Rust
// `crn` module.
//
// Usage:
//   gen_crn_golden <input.crn> <output.bc>
//
// Build (see gen_crn_golden.sh): g++ -O2 -std=c++17 -Ivendor/crunch/inc gen_crn_golden.cpp -o gen_crn_golden
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

// Pulls in the full header-only decompressor. CRND_HEADER_FILE_ONLY is left
// undefined so the implementation is compiled in.
#include "crn_decomp.h"

using namespace crnd;

static bool read_file(const char* path, std::vector<uint8>& out) {
    FILE* fp = std::fopen(path, "rb");
    if (!fp) {
        std::fprintf(stderr, "error: cannot open %s\n", path);
        return false;
    }
    std::fseek(fp, 0, SEEK_END);
    long sz = std::ftell(fp);
    std::fseek(fp, 0, SEEK_SET);
    out.resize(sz);
    if (sz > 0 && std::fread(out.data(), 1, sz, fp) != (size_t)sz) {
        std::fclose(fp);
        return false;
    }
    std::fclose(fp);
    return true;
}

static bool write_file(const char* path, const void* data, size_t size) {
    FILE* fp = std::fopen(path, "wb");
    if (!fp) {
        std::fprintf(stderr, "error: cannot write %s\n", path);
        return false;
    }
    if (size && std::fwrite(data, 1, size, fp) != size) {
        std::fclose(fp);
        return false;
    }
    std::fclose(fp);
    return true;
}

int main(int argc, char** argv) {
    if (argc != 3) {
        std::fprintf(stderr, "usage: %s <input.crn> <output.bc>\n", argv[0]);
        return 2;
    }

    std::vector<uint8> input;
    if (!read_file(argv[1], input)) return 1;

    crnd_unpack_context ctx = crnd_unpack_begin(input.data(), static_cast<uint32>(input.size()));
    if (!ctx) {
        std::fprintf(stderr, "error: crnd_unpack_begin failed\n");
        return 1;
    }

    crn_file_info fi;
    std::memset(&fi, 0, sizeof(fi));
    fi.m_struct_size = sizeof(fi);
    if (!crnd_validate_file(input.data(), static_cast<uint32>(input.size()), &fi)) {
        std::fprintf(stderr, "error: crnd_validate_file failed\n");
        crnd_unpack_end(ctx);
        return 1;
    }

    crn_texture_info ti;
    std::memset(&ti, 0, sizeof(ti));
    ti.m_struct_size = sizeof(ti);
    if (!crnd_get_texture_info(input.data(), static_cast<uint32>(input.size()), &ti)) {
        std::fprintf(stderr, "error: crnd_get_texture_info failed\n");
        crnd_unpack_end(ctx);
        return 1;
    }

    // Keep each public subresource separate while unpacking. The reference API
    // requires all face pointers for one mip level in a single call.
    std::vector<std::vector<uint8> > subresources(ti.m_faces * ti.m_levels);
    size_t total = 0;
    for (uint32 level = 0; level < ti.m_levels; ++level) {
        crn_level_info li;
        std::memset(&li, 0, sizeof(li));
        li.m_struct_size = sizeof(li);
        if (!crnd_get_level_info(
                input.data(), static_cast<uint32>(input.size()), level, &li)) {
            std::fprintf(stderr, "error: crnd_get_level_info failed for level %u\n", level);
            crnd_unpack_end(ctx);
            return 1;
        }

        const size_t face_size = static_cast<size_t>(li.m_blocks_x) *
                                 li.m_blocks_y * li.m_bytes_per_block;
        std::vector<void*> dst(ti.m_faces);
        for (uint32 face = 0; face < ti.m_faces; ++face) {
            std::vector<uint8>& bytes = subresources[face * ti.m_levels + level];
            bytes.assign(face_size, 0);
            dst[face] = bytes.data();
            total += face_size;
        }

        const uint32 row_pitch = li.m_bytes_per_block * li.m_blocks_x;
        if (!crnd_unpack_level(
                ctx, dst.data(), static_cast<uint32>(face_size), row_pitch, level)) {
            std::fprintf(stderr, "error: crnd_unpack_level failed for level %u\n", level);
            crnd_unpack_end(ctx);
            return 1;
        }
    }
    crnd_unpack_end(ctx);

    std::vector<uint8> output;
    output.reserve(total);
    for (uint32 face = 0; face < ti.m_faces; ++face) {
        for (uint32 level = 0; level < ti.m_levels; ++level) {
            const std::vector<uint8>& bytes = subresources[face * ti.m_levels + level];
            output.insert(output.end(), bytes.begin(), bytes.end());
        }
    }

    if (!write_file(argv[2], output.data(), total)) return 1;

    // Print metadata for the shell script to log.
    std::printf(
        "%s: format=%u %ux%u faces=%u levels=%u block_bytes=%u total=%zu\n",
        argv[1], static_cast<unsigned>(ti.m_format), ti.m_width, ti.m_height,
        ti.m_faces, ti.m_levels, ti.m_bytes_per_block, total);
    return 0;
}
