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

use unity_asset_rs::texture_export::{
    TextureFormat, decode_astc, decode_uncompressed, texture_pixel_bytes, write_png,
};
use unity_asset_rs::{ClassIDType, Environment, ParsedObject, UnityObject};

/// 含 ASTC 纹理的标准 Unity bundle fixture。
const UI_BUNDLE: &str = "example/DownloadCommonUI.unity3d";

/// 加载 Environment（fixture 不存在时跳过，对齐 environment.rs 测试约定）。
fn load_env(path: &str) -> Environment {
    if !Path::new(path).exists() {
        eprintln!("跳过：{path} 不存在");
        std::process::exit(0);
    }
    Environment::builder()
        .path(path)
        .unwrap_or_else(|e| panic!("加载 {path} 失败: {e}"))
        .build()
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
        let non_transparent = rgba
            .chunks_exact(4)
            .filter(|px| px[3] > 0)
            .count();
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
fn uncompressed_signed_reinterpret() {
    // R8_SIGNED: byte 0xFF (i8=-1) → reinterpret as u8 = 255
    let out = decode_uncompressed(&[0xFF], 1, 1, TextureFormat::R8Signed).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // RG16_SIGNED: [0xFF, 0x80] → [255, 128, 0, 255]
    let out = decode_uncompressed(&[0xFF, 0x80], 1, 1, TextureFormat::Rg16Signed).unwrap();
    assert_eq!(out, [255, 128, 0, 255]);
    // RGBA32_SIGNED: 全 4 字节 reinterpret
    let out = decode_uncompressed(&[0xFF, 0x80, 0x01, 0x7F], 1, 1, TextureFormat::Rgba32Signed).unwrap();
    assert_eq!(out, [255, 128, 1, 127]);
    // RGB24_SIGNED: 3 字节 reinterpret + A=255
    let out = decode_uncompressed(&[0xFF, 0x80, 0x01], 1, 1, TextureFormat::Rgb24Signed).unwrap();
    assert_eq!(out, [255, 128, 1, 255]);
}

#[test]
fn uncompressed_signed_16bit_reinterpret() {
    // R16_SIGNED: i16 bits reinterpret 后走 downscale16。
    // bytes [0xFF,0xFF] = 0xFFFF → downscale16(65535) = 255（同 R16 的 0xFFFF）
    let out = decode_uncompressed(&[0xFF, 0xFF], 1, 1, TextureFormat::R16Signed).unwrap();
    assert_eq!(out, [255, 0, 0, 255]);
    // RGBA64_SIGNED: 4× LE i16 reinterpret + downscale16。全 0xFFFF → RGBA(255,255,255,255)
    let out = decode_uncompressed(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 1, 1, TextureFormat::Rgba64Signed).unwrap();
    assert_eq!(out, [255, 255, 255, 255]);
    // RGB48_SIGNED: 3× LE i16。0xFFFF×3 → (255,255,255,255)
    let out = decode_uncompressed(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 1, 1, TextureFormat::Rgb48Signed).unwrap();
    assert_eq!(out, [255, 255, 255, 255]);
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
            unity_asset_rs::texture_export::TextureExportError::Decode {
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
