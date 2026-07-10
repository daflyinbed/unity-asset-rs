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
    TextureFormat, decode_astc, texture_pixel_bytes, write_png,
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
