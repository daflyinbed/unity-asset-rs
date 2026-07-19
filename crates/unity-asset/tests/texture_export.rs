//! 纹理导出层端到端测试：从真实 bundle 加载 Texture2D，解码 ASTC + 写 PNG。
//!
//! 使用 `example/DownloadCommonUI.unity3d`（标准 Unity bundle，2 个 ASTC_RGB_6x6 纹理，
//! 像素走 `.resS` streaming）——无需 `--arknights` 特殊 codec。
//! 明日方舟 `chararts` bundle 因其 LZ4 变体 codec 在 CLI 层（非库），这里不测。
//!
//! 受 `export` feature 门控（默认开启）。

#![cfg(feature = "export")]

use std::io::Cursor;
use std::path::Path;

use generated::{ClassIDType, UnityObject};
use unity_asset::texture_export::{
    TextureFormat, decode_astc, decode_crunch, decode_uncompressed, texture_pixel_bytes, write_png,
};
use unity_asset::{Environment, Loader, ParsedObject, UnityVersion};

/// 含 ASTC 纹理的标准 Unity bundle fixture（仓库根 example/；本 crate 回溯两级）。
const UI_BUNDLE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../example/DownloadCommonUI.unity3d"
);

/// 加载 Environment（fixture 不存在时跳过，对齐 environment.rs 测试约定）。
fn load_env(path: &str) -> Environment {
    if !Path::new(path).exists() {
        eprintln!("跳过：{path} 不存在");
        std::process::exit(0);
    }
    Loader::builder()
        .path(path)
        .build()
        .unwrap_or_else(|e| panic!("加载 {path} 失败: {e}"))
        .into_environment()
}

/// 取该 bundle 里全部 typed Texture2D（path_id, 宽, 高, 格式）。
fn collect_textures(env: &Environment) -> Vec<(i64, u32, u32, i32)> {
    env.objects_of_class(ClassIDType::Texture2D)
        .filter_map(|o| match &o.object {
            ParsedObject::Typed(UnityObject::Texture2D(t)) => {
                Some((o.path_id, t.m_width as u32, t.m_height as u32, t.m_format))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn bundle_has_astc_textures() {
    let env = load_env(UI_BUNDLE);
    let tex = collect_textures(&env);
    assert!(!tex.is_empty(), "应至少有一个 Texture2D");
    // 全部都是 ASTC_RGB_6x6 (format=50)，且为已知格式。
    for &(_, _, _, fmt) in &tex {
        assert_eq!(fmt, 50, "预期 ASTC_RGB_6x6 (format=50)，实得 format={fmt}");
        assert_eq!(
            TextureFormat::from_i32(fmt),
            Some(TextureFormat::AstcRgb6x6),
        );
        assert!(TextureFormat::from_i32(fmt).unwrap().is_supported());
    }
}

#[test]
fn streaming_pixel_bytes_resolve() {
    let env = load_env(UI_BUNDLE);
    for o in env.objects_of_class(ClassIDType::Texture2D) {
        let ParsedObject::Typed(UnityObject::Texture2D(t)) = &o.object else {
            continue;
        };
        // 该 bundle 全部纹理 image_data 为空、走 .resS streaming。
        let bytes = texture_pixel_bytes(t, &env).expect("streaming 像素数据应能取到");
        // ASTC 6x6：每块 16B / 6×6 texel。1024×1024 → ceil(1024/6)^2 块。
        // 块数 = 171*171 = 29241，字节 = 29241*16 = 467856。至少应非空。
        assert!(!bytes.is_empty(), "像素数据不应为空");
        assert_eq!(bytes.len(), 467856, "1024x1024 astc6x6 应为 467856 字节");
    }
}

#[test]
fn decode_produces_correct_sized_rgba() {
    let env = load_env(UI_BUNDLE);
    for o in env.objects_of_class(ClassIDType::Texture2D) {
        let ParsedObject::Typed(UnityObject::Texture2D(t)) = &o.object else {
            continue;
        };
        let fmt = TextureFormat::from_i32(t.m_format).unwrap();
        let bytes = texture_pixel_bytes(t, &env).expect("像素数据应能取到");
        let (w, h) = (t.m_width as u32, t.m_height as u32);
        let rgba = decode_astc(&bytes, w, h, fmt).expect("ASTC 解码应成功");
        assert_eq!(
            rgba.len(),
            (w as usize) * (h as usize) * 4,
            "解码输出应为 width*height*4 RGBA8",
        );
    }
}

#[test]
fn decode_output_is_not_blank() {
    let env = load_env(UI_BUNDLE);
    for o in env.objects_of_class(ClassIDType::Texture2D) {
        let ParsedObject::Typed(UnityObject::Texture2D(t)) = &o.object else {
            continue;
        };
        let fmt = TextureFormat::from_i32(t.m_format).unwrap();
        let bytes = texture_pixel_bytes(t, &env).expect("像素数据应能取到");
        let (w, h) = (t.m_width as u32, t.m_height as u32);
        let rgba = decode_astc(&bytes, w, h, fmt).expect("ASTC 解码应成功");
        // 至少应有非零像素（不是全黑 / 全透明）。统计 alpha>0 的像素占比。
        let non_transparent = rgba.chunks_exact(4).filter(|px| px[3] > 0).count();
        assert!(
            non_transparent > 0,
            "解码结果不应全透明（atlas 纹理必有可见像素）",
        );
    }
}

#[test]
fn write_png_produces_valid_signature() {
    let env = load_env(UI_BUNDLE);
    for o in env.objects_of_class(ClassIDType::Texture2D) {
        let ParsedObject::Typed(UnityObject::Texture2D(t)) = &o.object else {
            continue;
        };
        let fmt = TextureFormat::from_i32(t.m_format).unwrap();
        let bytes = texture_pixel_bytes(t, &env).expect("像素数据应能取到");
        let (w, h) = (t.m_width as u32, t.m_height as u32);
        let rgba = decode_astc(&bytes, w, h, fmt).expect("ASTC 解码应成功");

        let mut buf = Cursor::new(Vec::new());
        write_png(rgba, w, h, &mut buf).expect("PNG 编码应成功");
        let out = buf.into_inner();
        // PNG 文件签名：89 50 4E 47 0D 0A 1A 0A。
        assert_eq!(
            &out[..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            "输出应以 PNG 签名开头",
        );
    }
}

// ── 统一入口 decode_texture / Environment::decode_image ──

#[test]
fn decode_texture_and_env_decode_image_produce_same_pixels() {
    // 阶段 7b:export::decode_texture(统一 5 路分派)与 Environment::decode_image
    // (高层入口)对同一 Texture2D 应产出一致像素。
    if !Path::new(UI_BUNDLE).exists() {
        eprintln!("跳过：缺少 fixture {UI_BUNDLE}");
        return;
    }
    let env = load_env(UI_BUNDLE);
    let tex_obj = env
        .objects_of_class(ClassIDType::Texture2D)
        .next()
        .expect("应含 Texture2D");
    let ParsedObject::Typed(UnityObject::Texture2D(t)) = &tex_obj.object else {
        panic!("应为 Texture2D");
    };
    let ver = env.collections()[tex_obj.collection.to_index()].unity_version;

    // 底层统一入口。
    let img_direct =
        unity_asset::export::decode_texture(t, &env, ver).expect("decode_texture 应成功");
    // 高层入口(自动取版本)。
    let img_high = env
        .decode_image(tex_obj, &unity_asset::export::DecodeOptions::default())
        .expect("decode_image 应成功");

    assert_eq!(img_direct.width, img_high.width);
    assert_eq!(img_direct.height, img_high.height);
    assert_eq!(img_direct.rgba, img_high.rgba, "两入口像素应一致");
    assert_eq!(
        img_direct.rgba.len(),
        (img_direct.width as usize) * (img_direct.height as usize) * 4
    );

    // DecodedImage::write_png 落盘应产出有效 PNG。
    let tmp = std::env::temp_dir().join("unity_stage7b_decode_image_test.png");
    img_direct
        .write_png_path(&tmp)
        .expect("write_png_path 应成功");
    let bytes = std::fs::read(&tmp).expect("应能读回");
    assert_eq!(&bytes[..4], &[0x89, 0x50, 0x4E, 0x47], "应以 PNG 签名开头");
    let _ = std::fs::remove_file(&tmp);
}

// ── 未压缩格式字节级单测 ──
//
// 不依赖 fixture，构造确定性输入字节 → 断言输出 RGBA 精确匹配。
// 公式对齐 AssetStudio `Texture2DConverter.cs`（其输出 BGRA，这里期望 RGBA）。
// 每个测试用 1×1 图像，避免 Y 翻转干扰（单行翻转是 no-op）。

#[test]
fn uncompressed_alpha8() {
    // 输入 1 字节 alpha=0x80 → RGBA [0,0,0,0x80]
    let out = decode_uncompressed(&[0x80], 1, 1, TextureFormat::Alpha8).unwrap();
    assert_eq!(out, [0, 0, 0, 0x80]);
}

#[test]
fn uncompressed_r8() {
    // 输入 1 字节 R=0x80 → RGBA [0x80,0,0,255]
    let out = decode_uncompressed(&[0x80], 1, 1, TextureFormat::R8).unwrap();
    assert_eq!(out, [0x80, 0, 0, 255]);
}

#[test]
fn uncompressed_rg16() {
    // 输入 [R=0x11, G=0x22] → RGBA [0x11,0x22,0,255]
    let out = decode_uncompressed(&[0x11, 0x22], 1, 1, TextureFormat::Rg16).unwrap();
    assert_eq!(out, [0x11, 0x22, 0, 255]);
}

#[test]
fn uncompressed_rgb24() {
    // 输入 [R,G,B]=[0x11,0x22,0x33] → RGBA [0x11,0x22,0x33,255]（原序 + A=255）
    let out = decode_uncompressed(&[0x11, 0x22, 0x33], 1, 1, TextureFormat::Rgb24).unwrap();
    assert_eq!(out, [0x11, 0x22, 0x33, 255]);
}

#[test]
fn uncompressed_bgr24() {
    // 输入 [B,G,R]=[0x33,0x22,0x11] → RGBA [0x11,0x22,0x33,255]（R/B 交换）
    let out = decode_uncompressed(&[0x33, 0x22, 0x11], 1, 1, TextureFormat::Bgr24).unwrap();
    assert_eq!(out, [0x11, 0x22, 0x33, 255]);
}

#[test]
fn uncompressed_rgba32() {
    // 输入 [R,G,B,A]=[0x11,0x22,0x33,0x44] → RGBA 原序
    let out = decode_uncompressed(&[0x11, 0x22, 0x33, 0x44], 1, 1, TextureFormat::Rgba32).unwrap();
    assert_eq!(out, [0x11, 0x22, 0x33, 0x44]);
}

#[test]
fn uncompressed_argb32() {
    // 输入 [A,R,G,B]=[0x44,0x11,0x22,0x33] → RGBA [0x11,0x22,0x33,0x44]
    let out = decode_uncompressed(&[0x44, 0x11, 0x22, 0x33], 1, 1, TextureFormat::Argb32).unwrap();
    assert_eq!(out, [0x11, 0x22, 0x33, 0x44]);
}

#[test]
fn uncompressed_bgra32() {
    // 输入 [B,G,R,A]=[0x11,0x22,0x33,0x44] → RGBA [0x33,0x22,0x11,0x44]
    let out = decode_uncompressed(&[0x11, 0x22, 0x33, 0x44], 1, 1, TextureFormat::Bgra32).unwrap();
    assert_eq!(out, [0x33, 0x22, 0x11, 0x44]);
}

#[test]
fn uncompressed_rgb565_bit_replication() {
    // 16-bit 布局 RRRRRGGGGGGBBBBB（LE）。位复制扩展（非 *255//31 缩放）。
    // 0xF800 LE = bytes [0x00, 0xF8] → R=31→255, G=0, B=0
    let out = decode_uncompressed(&[0x00, 0xF8], 1, 1, TextureFormat::Rgb565).unwrap();
    assert_eq!(out, [255, 0, 0, 255], "0xF800 应为纯红");
    // 0x07E0 LE = bytes [0xE0, 0x07] → G=63→255
    let out = decode_uncompressed(&[0xE0, 0x07], 1, 1, TextureFormat::Rgb565).unwrap();
    assert_eq!(out, [0, 255, 0, 255], "0x07E0 应为纯绿");
    // 0x001F LE = bytes [0x1F, 0x00] → B=31→255
    let out = decode_uncompressed(&[0x1F, 0x00], 1, 1, TextureFormat::Rgb565).unwrap();
    assert_eq!(out, [0, 0, 255, 255], "0x001F 应为纯蓝");
    // 0xFFFF → 全白
    let out = decode_uncompressed(&[0xFF, 0xFF], 1, 1, TextureFormat::Rgb565).unwrap();
    assert_eq!(out, [255, 255, 255, 255], "0xFFFF 应为白");
}

#[test]
fn uncompressed_argb4444_nibble_expand() {
    // LE bytes [0xCD, 0xAB] → u16 = 0xABCD。
    // 16-bit = AAAARRRRGGGGBBBB（A=15-12, R=11-8, G=7-4, B=3-0）。
    // RGBA = [R=v>>8, G=v>>4, B=v, A=v>>12]，nibble×17。
    // v=0xABCD: A=0xA→0xAA=170, R=0xB→0xBB=187, G=0xC→0xCC=204, B=0xD→0xDD=221
    let out = decode_uncompressed(&[0xCD, 0xAB], 1, 1, TextureFormat::Argb4444).unwrap();
    assert_eq!(out, [187, 204, 221, 170]);
}

#[test]
fn uncompressed_rgba4444_nibble_expand() {
    // v=0xABCD: 16-bit = RRRRGGGGBBBBAAAA（R=15-12, G=11-8, B=7-4, A=3-0）。
    // RGBA = [R=v>>12, G=v>>8, B=v>>4, A=v]，nibble×17。
    let out = decode_uncompressed(&[0xCD, 0xAB], 1, 1, TextureFormat::Rgba4444).unwrap();
    assert_eq!(out, [170, 187, 204, 221]);
}

#[test]
fn uncompressed_r16_downscale() {
    // downscale16 = (v*255+32895)>>16。LE bytes。
    // v=0x00FF=255 → (255*255+32895)>>16 = (65025+32895)>>16 = 97920>>16 = 1
    let out = decode_uncompressed(&[0xFF, 0x00], 1, 1, TextureFormat::R16).unwrap();
    assert_eq!(out, [1, 0, 0, 255]);
    // v=0xFFFF=65535 → (65535*255+32895)>>16 = 255
    let out = decode_uncompressed(&[0xFF, 0xFF], 1, 1, TextureFormat::R16).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // v=0x8000=32768 → ≈ 128
    let out = decode_uncompressed(&[0x00, 0x80], 1, 1, TextureFormat::R16).unwrap();
    assert_eq!(out, [128, 0, 0, 255]);
}

#[test]
fn uncompressed_signed_xor_signbit_8bit() {
    // 8-bit SIGNED：XOR 0x80（offset-binary / SNORM → unorm），对齐 AssetRipper ChangeSign。
    // R8_SIGNED: byte 0xFF (i8=-1) → 0xFF ^ 0x80 = 0x7F = 127
    let out = decode_uncompressed(&[0xFF], 1, 1, TextureFormat::R8Sign).unwrap();
    assert_eq!(out, [127, 0, 0, 255]);
    // byte 0x00 (i8=0) → 0x00 ^ 0x80 = 128
    let out = decode_uncompressed(&[0x00], 1, 1, TextureFormat::R8Sign).unwrap();
    assert_eq!(out, [128, 0, 0, 255]);
    // byte 0x7F (i8=127, 最大正值) → 0x7F ^ 0x80 = 0xFF = 255
    let out = decode_uncompressed(&[0x7F], 1, 1, TextureFormat::R8Sign).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // byte 0x80 (i8=-128, 最小负值) → 0x80 ^ 0x80 = 0
    let out = decode_uncompressed(&[0x80], 1, 1, TextureFormat::R8Sign).unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
    // RG16_SIGNED: [0xFF, 0x80] → [127, 0, 0, 255]
    let out = decode_uncompressed(&[0xFF, 0x80], 1, 1, TextureFormat::Rg16Sign).unwrap();
    assert_eq!(out, [127, 0, 0, 255]);
    // RGBA32_SIGNED: [0xFF,0x80,0x01,0x7F] → 各 ^0x80 = [127,0,129,255]
    let out =
        decode_uncompressed(&[0xFF, 0x80, 0x01, 0x7F], 1, 1, TextureFormat::Rgba32Sign).unwrap();
    assert_eq!(out, [127, 0, 129, 255]);
    // RGB24_SIGNED: [0xFF,0x80,0x01] → [127,0,129] + A=255
    let out = decode_uncompressed(&[0xFF, 0x80, 0x01], 1, 1, TextureFormat::Rgb24Sign).unwrap();
    assert_eq!(out, [127, 0, 129, 255]);
}

#[test]
fn uncompressed_signed_xor_signbit_16bit() {
    // 16-bit SIGNED：XOR 0x8000 再 downscale16，对齐 AssetRipper ChangeSign + ConvertUInt16<byte>。
    // R16_SIGNED: bytes [0xFF,0xFF] = 0xFFFF ^ 0x8000 = 0x7FFF = 32767
    //   downscale16(32767) = (32767*255+32895)>>16 = (8355585+32895)>>16 = 8388480>>16 = 127
    let out = decode_uncompressed(&[0xFF, 0xFF], 1, 1, TextureFormat::R16Sign).unwrap();
    assert_eq!(out, [127, 0, 0, 255]);
    // bytes [0x00,0x00] = 0x0000 ^ 0x8000 = 0x8000 = 32768
    //   downscale16(32768) = (32768*255+32895)>>16 = 8388735>>16 = 128（signed 0 → 中灰）
    let out = decode_uncompressed(&[0x00, 0x00], 1, 1, TextureFormat::R16Sign).unwrap();
    assert_eq!(out, [128, 0, 0, 255]);
    // bytes [0xFF,0x7F] = 0x7FFF ^ 0x8000 = 0xFFFF = 65535 → downscale16 = 255（最大正值）
    let out = decode_uncompressed(&[0xFF, 0x7F], 1, 1, TextureFormat::R16Sign).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // bytes [0x00,0x80] = 0x8000 ^ 0x8000 = 0x0000 = 0 → downscale16 = 0（最小负值）
    let out = decode_uncompressed(&[0x00, 0x80], 1, 1, TextureFormat::R16Sign).unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
    // RGBA64_SIGNED: 全 0xFFFF → 各 ^0x8000=0x7FFF → downscale16=127 → RGBA(127,127,127,127)
    let out = decode_uncompressed(
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        1,
        1,
        TextureFormat::Rgba64Sign,
    )
    .unwrap();
    assert_eq!(out, [127, 127, 127, 127]);
    // RGB48_SIGNED: 0xFFFF×3 → (127,127,127,255)
    let out = decode_uncompressed(
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        1,
        1,
        TextureFormat::Rgb48Sign,
    )
    .unwrap();
    assert_eq!(out, [127, 127, 127, 255]);
    // RG32_SIGNED: 0xFFFF×2 → (127,127,0,255)
    let out =
        decode_uncompressed(&[0xFF, 0xFF, 0xFF, 0xFF], 1, 1, TextureFormat::Rg32Sign).unwrap();
    assert_eq!(out, [127, 127, 0, 255]);
}

// ── 16-bit 多通道 unsigned（72-74）单测 ──
// 与对应 SIGNED 变体共用 downscale16，但不 XOR 符号位。

#[test]
fn uncompressed_rg32_downscale() {
    // RG32: 2× LE u16，各 downscale16。bytes [0xFF,0xFF, 0x00,0x80]
    //   R=0xFFFF→255, G=0x8000→128
    let out = decode_uncompressed(&[0xFF, 0xFF, 0x00, 0x80], 1, 1, TextureFormat::Rg32).unwrap();
    assert_eq!(out, [255, 128, 0, 255]);
}

#[test]
fn uncompressed_rgb48_downscale() {
    // RGB48: 3× LE u16。全 0xFFFF → (255,255,255,255)
    let out = decode_uncompressed(
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        1,
        1,
        TextureFormat::Rgb48,
    )
    .unwrap();
    assert_eq!(out, [255, 255, 255, 255]);
    // 全 0x0000 → (0,0,0,255)
    let out = decode_uncompressed(&[0, 0, 0, 0, 0, 0], 1, 1, TextureFormat::Rgb48).unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
}

#[test]
fn uncompressed_rgba64_downscale() {
    // RGBA64: 4× LE u16。全 0xFFFF → (255,255,255,255)
    let out = decode_uncompressed(
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        1,
        1,
        TextureFormat::Rgba64,
    )
    .unwrap();
    assert_eq!(out, [255, 255, 255, 255]);
    // 全 0x8000 → 各 downscale16(32768)=128 → (128,128,128,128)
    let out = decode_uncompressed(
        &[0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80],
        1,
        1,
        TextureFormat::Rgba64,
    )
    .unwrap();
    assert_eq!(out, [128, 128, 128, 128]);
}

// ── HDR 浮点（6, 15-20, 22, 23）单测 ──
// 线性 ×255 饱和，无 tone mapping。f32 as u8 为饱和截断。

#[test]
fn uncompressed_hdr_float_basic() {
    // 辅助：构造 LE f32 字节。
    let f32_bytes = |v: f32| v.to_le_bytes();

    // RFloat: 1.0 → 255
    let inp = f32_bytes(1.0);
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::RFloat).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // RFloat: 0.0 → 0
    let out = decode_uncompressed(&f32_bytes(0.0), 1, 1, TextureFormat::RFloat).unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
    // RFloat: 0.5 → 127 (0.5*255=127.5 截断为 127)
    let out = decode_uncompressed(&f32_bytes(0.5), 1, 1, TextureFormat::RFloat).unwrap();
    assert_eq!(out, [127, 0, 0, 255]);
    // RFloat: 2.0 (HDR >1) → 255 饱和
    let out = decode_uncompressed(&f32_bytes(2.0), 1, 1, TextureFormat::RFloat).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // RFloat: -1.0 (负值) → 0 饱和
    let out = decode_uncompressed(&f32_bytes(-1.0), 1, 1, TextureFormat::RFloat).unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
}

#[test]
fn uncompressed_hdr_rgfloat() {
    let f32_bytes = |v: f32| v.to_le_bytes();
    // RGFloat: R=1.0, G=0.5 → (255,127,0,255)
    let mut inp = Vec::new();
    inp.extend_from_slice(&f32_bytes(1.0));
    inp.extend_from_slice(&f32_bytes(0.5));
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::RgFloat).unwrap();
    assert_eq!(out, [255, 127, 0, 255]);
}

#[test]
fn uncompressed_hdr_rgbafloat() {
    let f32_bytes = |v: f32| v.to_le_bytes();
    // RGBAFloat: R=1.0,G=0.5,B=0.25,A=0.0 → (255,127,63,0)
    let mut inp = Vec::new();
    inp.extend_from_slice(&f32_bytes(1.0));
    inp.extend_from_slice(&f32_bytes(0.5));
    inp.extend_from_slice(&f32_bytes(0.25));
    inp.extend_from_slice(&f32_bytes(0.0));
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::RgbaFloat).unwrap();
    assert_eq!(out, [255, 127, 63, 0]);
}

#[test]
fn uncompressed_hdr_argbfloat() {
    let f32_bytes = |v: f32| v.to_le_bytes();
    // ARGBFloat: 输入 [A,R,G,B]=[0.0,1.0,0.5,0.25] → 输出 [R,G,B,A]=[255,127,63,0]
    let mut inp = Vec::new();
    inp.extend_from_slice(&f32_bytes(0.0)); // A
    inp.extend_from_slice(&f32_bytes(1.0)); // R
    inp.extend_from_slice(&f32_bytes(0.5)); // G
    inp.extend_from_slice(&f32_bytes(0.25)); // B
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::ArgbFloat).unwrap();
    assert_eq!(out, [255, 127, 63, 0]);
}

#[test]
fn uncompressed_hdr_rgbfloat() {
    let f32_bytes = |v: f32| v.to_le_bytes();
    // RGBFloat: R=1.0,G=0.5,B=0.25 → (255,127,63,255)
    let mut inp = Vec::new();
    inp.extend_from_slice(&f32_bytes(1.0));
    inp.extend_from_slice(&f32_bytes(0.5));
    inp.extend_from_slice(&f32_bytes(0.25));
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::RgbFloat).unwrap();
    assert_eq!(out, [255, 127, 63, 255]);
}

#[test]
fn uncompressed_hdr_half_basic() {
    // 辅助：构造 LE fp16 bits。
    let f16_bits = |v: half::f16| v.to_le_bytes();
    // RHalf: 1.0 → 255
    let out = decode_uncompressed(
        &f16_bits(half::f16::from_f32(1.0)),
        1,
        1,
        TextureFormat::RHalf,
    )
    .unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // RHalf: 0.5 → 127
    let out = decode_uncompressed(
        &f16_bits(half::f16::from_f32(0.5)),
        1,
        1,
        TextureFormat::RHalf,
    )
    .unwrap();
    assert_eq!(out, [127, 0, 0, 255]);
    // RHalf: 0.0 → 0
    let out = decode_uncompressed(
        &f16_bits(half::f16::from_f32(0.0)),
        1,
        1,
        TextureFormat::RHalf,
    )
    .unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
    // RHalf: -1.0 (负值) → 0 饱和
    let out = decode_uncompressed(
        &f16_bits(half::f16::from_f32(-1.0)),
        1,
        1,
        TextureFormat::RHalf,
    )
    .unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
}

#[test]
fn uncompressed_hdr_rgbandrgbahalf() {
    let f16_bits = |v: half::f16| v.to_le_bytes();
    // RGHalf: R=1.0, G=0.5 → (255,127,0,255)
    let mut inp = Vec::new();
    inp.extend_from_slice(&f16_bits(half::f16::from_f32(1.0)));
    inp.extend_from_slice(&f16_bits(half::f16::from_f32(0.5)));
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::RgHalf).unwrap();
    assert_eq!(out, [255, 127, 0, 255]);
    // RGBAHalf: R=1.0,G=0.5,B=0.25,A=0.0 → (255,127,63,0)
    let mut inp = Vec::new();
    inp.extend_from_slice(&f16_bits(half::f16::from_f32(1.0)));
    inp.extend_from_slice(&f16_bits(half::f16::from_f32(0.5)));
    inp.extend_from_slice(&f16_bits(half::f16::from_f32(0.25)));
    inp.extend_from_slice(&f16_bits(half::f16::from_f32(0.0)));
    let out = decode_uncompressed(&inp, 1, 1, TextureFormat::RgbaHalf).unwrap();
    assert_eq!(out, [255, 127, 63, 0]);
}

#[test]
fn uncompressed_hdr_rgb9e5float() {
    // RGB9e5Float: 32-bit LE packed = [5-bit exp@27-31][9-bit B@18-26][9-bit G@9-17][9-bit R@0-8]。
    // value = mantissa9 * 2^(exp - 24)，alpha 硬编码 255。
    // 构造黑色：全 0 → (0,0,0,255)
    let out = decode_uncompressed(&[0, 0, 0, 0], 1, 1, TextureFormat::Rgb9e5Float).unwrap();
    assert_eq!(out, [0, 0, 0, 255]);
    // 构造 exp=24（scale=1），R=255(0xFF)... 但 9-bit 尾数最大 511。
    // 取 exp=24, R=511 → 511*1.0*255 = 130305 → 饱和 255。
    // packed = (24 << 27) | (511) = 0xC0000000 | 0x1FF = 0xC00001FF
    let packed: u32 = (24u32 << 27) | 511;
    let out = decode_uncompressed(&packed.to_le_bytes(), 1, 1, TextureFormat::Rgb9e5Float).unwrap();
    assert_eq!(out, [255, 0, 0, 255], "exp=24 R=511 应饱和到 255");
    // exp=24, R=256 → 256*1.0*255 = 65280 → 饱和 255
    let packed: u32 = (24u32 << 27) | 256;
    let out = decode_uncompressed(&packed.to_le_bytes(), 1, 1, TextureFormat::Rgb9e5Float).unwrap();
    assert_eq!(out, [255, 0, 0, 255], "exp=24 R=256 应饱和到 255");
    // exp=24, R=1 → 1*1.0*255 = 255
    let packed: u32 = (24u32 << 27) | 1;
    let out = decode_uncompressed(&packed.to_le_bytes(), 1, 1, TextureFormat::Rgb9e5Float).unwrap();
    assert_eq!(out, [255, 0, 0, 255], "exp=24 R=1 应为 255");
    // exp=23 (scale=0.5), R=511 → 511*0.5*255 = 65152.25 → 截断 65152 → 饱和 255
    // exp=23, R=1 → 1*0.5*255 = 127.5 → 截断 127
    let packed: u32 = (23u32 << 27) | 1;
    let out = decode_uncompressed(&packed.to_le_bytes(), 1, 1, TextureFormat::Rgb9e5Float).unwrap();
    assert_eq!(out, [127, 0, 0, 255], "exp=23 R=1 应为 127");
    // 三通道：exp=24, R=1,G=1,B=1 → (255,255,255,255)
    let packed: u32 = (24u32 << 27) | (1 << 18) | (1 << 9) | 1;
    let out = decode_uncompressed(&packed.to_le_bytes(), 1, 1, TextureFormat::Rgb9e5Float).unwrap();
    assert_eq!(out, [255, 255, 255, 255], "exp=24 RGB=1,1,1 应为白");
}

#[test]
fn uncompressed_input_too_short_errors() {
    // RGB24 需要 3 字节/像素；给 2 字节应报 InputTooShort。
    let res = decode_uncompressed(&[0x11, 0x22], 1, 1, TextureFormat::Rgb24);
    assert!(res.is_err(), "输入过短应报错");
    let err = res.unwrap_err();
    assert!(
        matches!(
            err,
            unity_asset::texture_export::TextureExportError::Decode {
                err: texture_decoder_rs::DecodeError::InputTooShort
            }
        ),
        "应为 InputTooShort，实得 {err:?}"
    );
}

#[test]
fn uncompressed_multi_row_y_flip() {
    // 2×2 RGB24，验证 Y 翻转：输入底行(R1)应出现在输出顶行。
    // 行序（Unity bottom-up）：[B1,G1,R1, B2,G2,R2, B3,G3,R3, B4,G4,R4]
    //                    = 像素 (0,1),(1,1) 在前，(0,0),(1,0) 在后
    // 输出 top-down：像素 (0,0),(1,0),(0,1),(1,1)
    let input = [
        0x00, 0x00, 0xFF, // 像素(0,1) 蓝
        0x00, 0xFF, 0x00, // 像素(1,1) 绿
        0xFF, 0x00, 0x00, // 像素(0,0) 红
        0xFF, 0xFF, 0xFF, // 像素(1,0) 白
    ];
    let out = decode_uncompressed(&input, 2, 2, TextureFormat::Rgb24).unwrap();
    // 输出顺序：行0=[红,白]，行1=[蓝,绿]
    assert_eq!(
        out,
        [
            0xFF, 0x00, 0x00, 0xFF, // 红
            0xFF, 0xFF, 0xFF, 0xFF, // 白
            0x00, 0x00, 0xFF, 0xFF, // 蓝
            0x00, 0xFF, 0x00, 0xFF, // 绿
        ]
    );
}

// ── Crunch（原始 CRN v1.04 + Unity 变体；DXT1/DXT5/ETC1/ETC2A crunched）解码测试 ──
//
// 用 sibling crate `../texture-decoder-rs/tests/data/{crunch,crunch_unity}/` 下的 golden
// fixture（由参考 C++ transcoder 生成的 `.crn`/`.bc` 对）验证 `decode_crunch`：版本分水岭
// 分派（原始 vs Unity 变体）+ level-0 选取 + BC1/BC3/ETC1/ETC2_RGBA8 分派（由 CRN 头 format
// 决定）+ cubemap 6 面纵向拼接 + Y 翻转规则（2D 翻、cubemap 不翻，对齐 AssetRipper）。
// golden `.bc` 是独立 ground truth（crunch 走 `crn_decomp.h`，crunch_unity 走 Unity fork）。

/// 读 sibling crate 的 crunch fixture（`.crn` + `.bc`）；不存在返回 None（测试跳过）。
/// `subdir` = `crunch`（原始）或 `crunch_unity`（Unity 变体）。
fn crunch_fixture(subdir: &str, name: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    // sibling crate：crates/unity-asset → crates → unity-asset-rs → unity(dir) → texture-decoder-rs（4 级）
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../texture-decoder-rs/tests/data")
        .join(subdir);
    let crn = std::fs::read(base.join(format!("{name}.crn"))).ok()?;
    let bc = std::fs::read(base.join(format!("{name}.bc"))).ok()?;
    Some((crn, bc))
}

/// 独立实现的逐行翻转（与 lib 的 `flip_vertical_in_place` 各自实现，避免共享 bug）。
fn flip_rows(rgba: &mut [u8], w: usize, h: usize) {
    let stride = w * 4;
    for top in 0..h / 2 {
        let bottom = h - 1 - top;
        let (a, b) = rgba.split_at_mut(bottom * stride);
        a[top * stride..top * stride + stride].swap_with_slice(&mut b[..stride]);
    }
}

/// 第二段 block → RGBA8 解码器签名（bcn / etc 家族共用同一签名）。
type BlockDecode =
    fn(&[u8], usize, usize, &mut [u8]) -> Result<(), texture_decoder_rs::DecodeError>;

/// 2D（单面）crunch：decode_crunch(data, fmt, ver) 应等于 golden `.bc` 经 block_decode 后
/// Y 翻转。`fmt`+`ver` 决定走原始还是 Unity 变体路径（由调用方按 fixture 来源配对）。
fn check_crunch_2d(
    subdir: &str,
    name: &str,
    block_decode: BlockDecode,
    fmt: TextureFormat,
    ver: UnityVersion,
) {
    let (crn, bc) = match crunch_fixture(subdir, name) {
        Some(x) => x,
        None => {
            eprintln!("跳过：crunch fixture {subdir}/{name} 不存在（sibling crate 缺失）");
            return;
        }
    };
    let (w, h) = (6usize, 6usize);
    let mut expected = vec![0u8; w * h * 4];
    block_decode(&bc, w, h, &mut expected).unwrap();
    flip_rows(&mut expected, w, h); // 2D 走 Y 翻转。

    let (rgba, ow, oh) = decode_crunch(&crn, fmt, ver).expect("2D crunch decode");
    assert_eq!((ow, oh), (w as u32, h as u32), "2D 输出尺寸 = 单面尺寸");
    assert_eq!(rgba, expected, "2D crunch 应 = golden bc 解码 + Y 翻转");
}

/// Cubemap（6 面）crunch：decode_crunch 应纵向拼接 6 个面，每面 = golden 该面 level-0
/// block 经 block_decode；不 Y 翻转。golden `.bc` 布局 = face-major、每面 mip-major，
/// 3 mip [(6,6),(3,3),(1,1)]，故 level 0 是每面前 `level0_len` 字节。
fn check_crunch_cubemap(
    subdir: &str,
    name: &str,
    block_decode: BlockDecode,
    block_bytes: usize,
    fmt: TextureFormat,
    ver: UnityVersion,
) {
    let (crn, bc) = match crunch_fixture(subdir, name) {
        Some(x) => x,
        None => {
            eprintln!("跳过：crunch fixture {subdir}/{name} 不存在（sibling crate 缺失）");
            return;
        }
    };
    let (fw, fh) = (6usize, 6usize);
    // 每面字节数 = (mip0: 2×2 + mip1: 1×1 + mip2: 1×1) × block_bytes = 6×bb。
    let face_total = 6 * block_bytes;
    let level0_len = 4 * block_bytes; // 2×2 block
    assert_eq!(
        bc.len(),
        6 * face_total,
        "fixture 应为 6 面 × {face_total} 字节"
    );

    let out_w = fw;
    let out_h = fh * 6;
    let mut expected = vec![0u8; out_w * out_h * 4];
    for i in 0..6 {
        let face_blocks = &bc[i * face_total..i * face_total + level0_len];
        let slot = &mut expected[i * fw * fh * 4..(i + 1) * fw * fh * 4];
        block_decode(face_blocks, fw, fh, slot).unwrap();
    }
    // cubemap 不 Y 翻转。

    let (rgba, ow, oh) = decode_crunch(&crn, fmt, ver).expect("cubemap crunch decode");
    assert_eq!(
        (ow, oh),
        (out_w as u32, out_h as u32),
        "cubemap 输出高 = 面高×6"
    );
    assert_eq!(rgba, expected, "cubemap 应 6 面纵向拼接 + 不 Y 翻转");
}

// ── 原始 Crunch（CRN v1.04；Unity <2017.3 的 DXT crunched）──
// 版本 <2017.3 触发 is_unity_crunch=false → 走 crn 模块。

#[test]
fn crunch_dxt1_2d_matches_golden() {
    check_crunch_2d(
        "crunch",
        "dxt1-6x6",
        texture_decoder_rs::bcn::decode_bc1,
        TextureFormat::Dxt1Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

#[test]
fn crunch_dxt5_2d_matches_golden() {
    check_crunch_2d(
        "crunch",
        "dxt5-6x6",
        texture_decoder_rs::bcn::decode_bc3,
        TextureFormat::Dxt5Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

#[test]
fn crunch_dxt1_cubemap_stitches_six_faces() {
    check_crunch_cubemap(
        "crunch",
        "dxt1-cubemap-6x6",
        texture_decoder_rs::bcn::decode_bc1,
        8,
        TextureFormat::Dxt1Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

#[test]
fn crunch_dxt5_cubemap_stitches_six_faces() {
    check_crunch_cubemap(
        "crunch",
        "dxt5-cubemap-6x6",
        texture_decoder_rs::bcn::decode_bc3,
        16,
        TextureFormat::Dxt5Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

// ── Unity 变体 Crunch（Unity fork；Unity ≥2017.3 的 DXT crunched + 任意版本 ETC crunch）──
// 版本 ≥2017.3（DXT）或 ETC 格式（64/65）触发 is_unity_crunch=true → 走 crn_unity 模块。

#[test]
fn unity_crunch_dxt1_2d_matches_golden() {
    check_crunch_2d(
        "crunch_unity",
        "dxt1-6x6",
        texture_decoder_rs::bcn::decode_bc1,
        TextureFormat::Dxt1Crunched,
        UnityVersion::new(2017, 3, 0),
    );
}

#[test]
fn unity_crunch_dxt5_2d_matches_golden() {
    check_crunch_2d(
        "crunch_unity",
        "dxt5-6x6",
        texture_decoder_rs::bcn::decode_bc3,
        TextureFormat::Dxt5Crunched,
        UnityVersion::new(2017, 3, 0),
    );
}

#[test]
fn unity_crunch_etc1_2d_matches_golden() {
    check_crunch_2d(
        "crunch_unity",
        "etc1-6x6",
        texture_decoder_rs::etc::decode_etc1_rgb8,
        // ETC crunch 任意版本都走 Unity 变体；用 <2017.3 验证 format 触发分支。
        TextureFormat::EtcRgb4Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

#[test]
fn unity_crunch_etc2a_2d_matches_golden() {
    check_crunch_2d(
        "crunch_unity",
        "etc2a-6x6",
        texture_decoder_rs::etc::decode_etc2_rgba8,
        TextureFormat::Etc2Rgba8Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

#[test]
fn unity_crunch_dxt1_cubemap_stitches_six_faces() {
    check_crunch_cubemap(
        "crunch_unity",
        "dxt1-cubemap-6x6",
        texture_decoder_rs::bcn::decode_bc1,
        8,
        TextureFormat::Dxt1Crunched,
        UnityVersion::new(2017, 3, 0),
    );
}

#[test]
fn unity_crunch_dxt5_cubemap_stitches_six_faces() {
    check_crunch_cubemap(
        "crunch_unity",
        "dxt5-cubemap-6x6",
        texture_decoder_rs::bcn::decode_bc3,
        16,
        TextureFormat::Dxt5Crunched,
        UnityVersion::new(2017, 3, 0),
    );
}

#[test]
fn unity_crunch_etc1_cubemap_stitches_six_faces() {
    check_crunch_cubemap(
        "crunch_unity",
        "etc1-cubemap-6x6",
        texture_decoder_rs::etc::decode_etc1_rgb8,
        8,
        TextureFormat::EtcRgb4Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}

#[test]
fn unity_crunch_etc2a_cubemap_stitches_six_faces() {
    check_crunch_cubemap(
        "crunch_unity",
        "etc2a-cubemap-6x6",
        texture_decoder_rs::etc::decode_etc2_rgba8,
        16,
        TextureFormat::Etc2Rgba8Crunched,
        UnityVersion::new(2017, 2, 0),
    );
}
