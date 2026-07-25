#!/usr/bin/env python3
# Writes uncompressed truecolor TGA and DDS source images used to generate the
# Crunch golden fixtures. No third-party dependencies: both containers are
# simple enough to emit by hand. The compressor (`vendor/crunch`) reads them
# directly.
#
# Images are intentionally small and non-multiples of 4/8 so the decoder's
# block-edge handling is exercised. They mix flat regions, gradients, and
# per-texel variation to force the encoder to emit several distinct palette
# entries.
import struct
import sys
from pathlib import Path


def write_tga(path: Path, width: int, height: int, pixels: list[tuple[int, int, int, int]]):
    """Write a 32-bit BGRA uncompressed bottom-up TGA."""
    assert len(pixels) == width * height
    # TGA stores rows bottom-up by default; build from the last row to the first.
    body = bytearray()
    for y in range(height - 1, -1, -1):
        for x in range(width):
            r, g, b, a = pixels[y * width + x]
            body += struct.pack("<BBBB", b, g, r, a)
    header = struct.pack(
        "<BBBHHBHHHHBB",
        0,        # ID length
        0,        # color map type
        2,        # image type: uncompressed truecolor
        0, 0, 0,  # color map spec (unused)
        0, 0,     # x/y origin
        width, height,
        32,       # bits per pixel
        8,        # image descriptor: 8 = alpha bits present, origin bottom-left
    )
    path.write_bytes(header + bytes(body))


def write_dds_cubemap(
    path: Path,
    width: int,
    height: int,
    faces: list[list[tuple[int, int, int, int]]],
):
    """Write a legacy uncompressed A8R8G8B8 DDS cubemap.

    DDS stores cubemap faces in +X, -X, +Y, -Y, +Z, -Z order. Rows are
    top-down, and the chosen bit masks make the byte representation BGRA.
    """
    assert len(faces) == 6
    assert all(len(pixels) == width * height for pixels in faces)

    ddsd_caps = 0x00000001
    ddsd_height = 0x00000002
    ddsd_width = 0x00000004
    ddsd_pitch = 0x00000008
    ddsd_pixel_format = 0x00001000
    ddpf_alpha_pixels = 0x00000001
    ddpf_rgb = 0x00000040
    dds_caps_complex = 0x00000008
    dds_caps_texture = 0x00001000
    dds_caps2_all_cubemap_faces = 0x0000FE00

    # DDSURFACEDESC2 is exactly 31 little-endian dwords after the magic.
    header = [
        124,
        ddsd_caps | ddsd_height | ddsd_width | ddsd_pitch | ddsd_pixel_format,
        height,
        width,
        width * 4,
        0,  # depth
        0,  # mip count: the Crunch CLI generates the chain
        *([0] * 11),
        32,
        ddpf_rgb | ddpf_alpha_pixels,
        0,  # fourcc
        32,
        0x00FF0000,
        0x0000FF00,
        0x000000FF,
        0xFF000000,
        dds_caps_texture | dds_caps_complex,
        dds_caps2_all_cubemap_faces,
        0,
        0,
        0,
    ]
    assert len(header) == 31

    body = bytearray()
    for pixels in faces:
        for r, g, b, a in pixels:
            body += struct.pack("<BBBB", b, g, r, a)
    path.write_bytes(b"DDS " + struct.pack("<31I", *header) + bytes(body))


def make_color(width: int, height: int) -> list[tuple[int, int, int, int]]:
    """A 6x6 RGBA image with distinct color regions and a gradient, no alpha."""
    px = []
    for y in range(height):
        for x in range(width):
            # Quadrant-based flat colors plus a per-texel gradient tweak.
            base = (
                (x * 40) & 0xFF,
                (y * 40) & 0xFF,
                ((x + y) * 25) & 0xFF,
                255,
            )
            px.append(base)
    return px


def make_alpha(width: int, height: int) -> list[tuple[int, int, int, int]]:
    """A 6x6 RGBA image with varying alpha so DXT5 alpha blocks are nontrivial."""
    px = []
    for y in range(height):
        for x in range(width):
            px.append((
                (x * 40) & 0xFF,
                (y * 40) & 0xFF,
                ((x * y) * 7) & 0xFF,
                ((x + 1) * (y + 2) * 6) & 0xFF,
            ))
    return px


def make_cubemap_face(
    width: int,
    height: int,
    face: int,
    with_alpha: bool,
) -> list[tuple[int, int, int, int]]:
    """Make one visually distinct cubemap face with a face-specific alpha pattern."""
    face_colors = [
        (224, 32, 24),   # +X: red
        (24, 208, 48),   # -X: green
        (32, 56, 224),   # +Y: blue
        (224, 192, 24),  # -Y: yellow
        (208, 32, 208),  # +Z: magenta
        (24, 200, 208),  # -Z: cyan
    ]
    base_r, base_g, base_b = face_colors[face]

    px = []
    for y in range(height):
        for x in range(width):
            # Preserve the strong face color while giving every block enough
            # local variation to exercise endpoint and selector prediction.
            r = (base_r + x * 9 + y * 3) & 0xFF
            g = (base_g + x * 4 + y * 11) & 0xFF
            b = (base_b + x * 7 + y * 5) & 0xFF

            if not with_alpha:
                a = 255
            elif face == 0:
                a = 24 + x * 38                 # horizontal ramp
            elif face == 1:
                a = 28 + y * 36                 # vertical ramp
            elif face == 2:
                a = 32 if (x + y) % 2 == 0 else 224  # checkerboard
            elif face == 3:
                a = 16 + (x + y) * 22           # diagonal ramp
            elif face == 4:
                a = 20 + ((x * 37 + y * 19) % 216)  # oblique bands
            else:
                a = 232 - ((x * 17 + y * 31) % 208)  # descending bands

            px.append((r, g, b, a))
    return px


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("tests/data/crunch")
    out.mkdir(parents=True, exist_ok=True)
    w = h = 6
    write_tga(out / "src-color-6x6.tga", w, h, make_color(w, h))
    write_tga(out / "src-alpha-6x6.tga", w, h, make_alpha(w, h))
    write_dds_cubemap(
        out / "src-color-cubemap-6x6.dds",
        w,
        h,
        [make_cubemap_face(w, h, face, False) for face in range(6)],
    )
    write_dds_cubemap(
        out / "src-alpha-cubemap-6x6.dds",
        w,
        h,
        [make_cubemap_face(w, h, face, True) for face in range(6)],
    )
    print(f"wrote CRN source images under {out}/")


if __name__ == "__main__":
    main()
