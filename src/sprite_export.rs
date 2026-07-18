//! Sprite → PNG 导出（受 `export` feature 门控）。
//!
//! 把 Sprite 从其所在 atlas Texture2D 上裁下对应区域（textureRect），可选应用 packing
//! rotation 和 downscale multiplier，输出 PNG。算法对齐 AssetStudio `SpriteHelper.CutImage`
//! 与 UnityPy `SpriteHelper.get_image_from_sprite`（两者核心一致）。
//!
//! # 覆盖范围（第一阶段）
//!
//! - **Rectangle packing**：`SpriteRenderData` / `SpriteAtlasData` 的 `texture_rect` 裁剪。
//! - **standalone sprite**：直接用 `Sprite.m_rd`（`m_rd.texture` 指向 atlas Texture2D）。
//! - **SpriteAtlas sprite**：优先 `Sprite.m_sprite_atlas`（PPtr → SpriteAtlas），在
//!   `atlas.m_render_data_map` 里按 `Sprite.m_render_data_key` 查找 `SpriteAtlasData`；
//!   PPtr 为空但 `m_atlas_tags` 非空时按名称扫描同 collection 的 SpriteAtlas（兜底）。
//! - **packing rotation**：`settings_raw` bits 2-5（None / FlipH / FlipV / Rotate180 /
//!   Rotate90），在裁剪后 top-down 空间应用。
//! - **downscale multiplier**：若 `> 0 && != 1`，整图 resize 到 `w/mul × h/mul`（最近邻）。
//!
//! # 不做（留坑）
//!
//! - **Tight mesh packing**（`packingMode == Tight`）：需移植 UnityPy `MeshHelper` 顶点 /
//!   SubMesh / index buffer 解析 + 三角光栅化，工作量 ≈ 本模块全部。第一阶段跳过并报
//!   [`SpriteExportError::UnsupportedTight`]。
//! - **alpha texture 合并**（`m_rd.alpha_texture` / `SpriteAtlasData.alpha_texture`）：
//!   需解码两张图 merge RGBA 通道；第一阶段忽略 alpha（输出 RGB 部分）。
//!
//! # Y 翻转策略（top-down 空间裁剪）
//!
//! 参考实现是「decode 不翻转 → bottom-up 空间裁剪 → R → 最后 flipV」。我们的 `decode_*`
//! 已内置翻转返回 **top-down** RGBA（= `flipV ∘ decode_bu`），故采用共轭策略：所有计算
//! 在 top-down 空间进行，把 Unity bottom-up 的 `textureRect.y` 一次性换算成 top-down 行号。
//!
//! 设参考管线为 `flipV ∘ R ∘ crop_bu ∘ decode_bu`，本模块为 `R' ∘ crop_td ∘ flipV ∘ decode_bu`。
//! 由 `crop_td ∘ flipV == flipV ∘ crop_bu`（翻转后再裁 top-down = 在 bottom-up 裁后再翻转），
//! 推出 `R' = flipV ∘ R ∘ flipV`（共轭）。代入验证：
//! - R=None / FlipH / FlipV / Rotate180 → R' = R（不变）
//! - R=Rotate90（参考用 PIL `ROTATE_270` = rot90cw）→ R' = **rot90ccw**（共轭翻转方向）
//!
//! 裁剪框换算（bottom-up rect → top-down 行号）：
//! ```text
//! crop_x      = floor(rect.x)
//! crop_right  = ceil(rect.x + rect.width)
//! crop_y      = atlas_h - ceil(rect.y + rect.height)   // top-down 上边界
//! crop_bottom = atlas_h - floor(rect.y)                // top-down 下边界
//! ```
//! 越界部分 clamp 到 atlas 边界（Unity 偶有浮点 rect 超出 1 像素）。

use std::path::{Path, PathBuf};

use snafu::Snafu;

use crate::environment::EnvironmentObject;
use crate::texture_export::{flip_vertical_in_place, write_png, TextureExportError};
use crate::{ClassId, Environment, PPtr, ParsedObject, UnityVersion};
use generated::{ClassIDType, UnityObject};
use generated::classes::sprite::Sprite;
use generated::classes::sprite_atlas::SpriteAtlas;
use generated::subclasses::guid::Guid;
use generated::subclasses::rectf::Rectf;
use generated::subclasses::sprite_atlas_data::SpriteAtlasData;
use generated::subclasses::sprite_render_data::SpriteRenderData;

/// Sprite 导出错误。包装下游纹理错误 + sprite 专属失败。
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum SpriteExportError {
    /// 下游纹理解码 / IO 错误透传。
    #[snafu(display("texture decode/io error: {source}"))]
    Texture {
        source: TextureExportError,
    },

    /// `m_rd.texture` / atlas 的 `texture` PPtr 解引用失败（跨 bundle 没载入 / path_id 漂移）。
    #[snafu(display("sprite texture PPtr 解引用失败"))]
    TextureNotFound,

    /// atlas Texture2D 的格式不支持（未在 `TextureFormat` enum / `is_supported()`）。
    #[snafu(display("atlas texture 格式不支持: format_id={format_id}"))]
    UnsupportedAtlasFormat { format_id: i32 },

    /// Tight mesh packing 暂未实现（`packingMode == Tight`）。
    #[snafu(display("tight packing 暂不支持（第一阶段）"))]
    UnsupportedTight,

    /// 在 `m_render_data_map` 里按 `m_render_data_key` 找不到对应 `SpriteAtlasData`。
    #[snafu(display("sprite atlas render data map 未命中 render_data_key"))]
    AtlasKeyNotFound,

    /// textureRect 完全越界（w/h ≤ 0 或起点超出 atlas 尺寸）。
    #[snafu(display(
        "textureRect 完全越界: atlas={atlas_w}x{atlas_h} rect=({rect_x},{rect_y},{rect_w},{rect_h})"
    ))]
    RectOutOfBounds {
        atlas_w: u32,
        atlas_h: u32,
        rect_x: i64,
        rect_y: i64,
        rect_w: i64,
        rect_h: i64,
    },
}

// 把下游 `TextureExportError` 统一包成 `SpriteExportError::Texture`。
impl From<TextureExportError> for SpriteExportError {
    fn from(source: TextureExportError) -> Self {
        Self::Texture { source }
    }
}

// ── SpriteSettings bitfield（对齐 AssetStudio / UnityPy）────────────────────

/// `SpriteRenderData.settings_raw` / `SpriteAtlasData.settings_raw` 的 bitfield 解码。
///
/// bit 布局（两参考实现一致）：
/// - bit 0：`packed`
/// - bit 1：`packingMode`（0 = Tight, 1 = Rectangle）
/// - bits 2-5：`packingRotation`（0=None 1=FlipH 2=FlipV 3=Rotate180 4=Rotate90）
/// - bit 6：`meshType`（0 = FullRect, 1 = Tight）
#[derive(Clone, Copy)]
struct SpriteSettings(u32);

impl SpriteSettings {
    fn packed(self) -> bool {
        self.0 & 1 == 1
    }
    /// `true` = Tight（多边形 mesh，第一阶段不支持），`false` = Rectangle。
    fn packing_mode_tight(self) -> bool {
        (self.0 >> 1) & 1 == 0
    }
    fn packing_rotation(self) -> PackingRotation {
        match (self.0 >> 2) & 0xF {
            0 => PackingRotation::None,
            1 => PackingRotation::FlipHorizontal,
            2 => PackingRotation::FlipVertical,
            3 => PackingRotation::Rotate180,
            4 => PackingRotation::Rotate90,
            _ => PackingRotation::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackingRotation {
    None,
    FlipHorizontal,
    FlipVertical,
    Rotate180,
    Rotate90,
}

// ── 解析 render data 来源 ────────────────────────────────────────────────────

/// 裁剪所需的 atlas 元数据（texture PPtr + textureRect + settings + downscale）。
///
/// 所有引用的生命周期绑定到 `env` 的借用（实际调用中 `s` 也派生自 `env`，故统一为 `'e`）。
struct ResolvedRenderData<'e> {
    texture: &'e PPtr,
    texture_rect: &'e Rectf,
    settings_raw: u32,
    downscale_multiplier: Option<f32>,
}

/// 按「SpriteAtlas PPtr → AtlasTags 名称兜底 → standalone m_RD」三级解析 render data。
///
/// 对齐 UnityPy `get_image_from_sprite` 的查找顺序。
fn resolve_render_data<'e>(
    env: &'e Environment,
    sprite_obj: &'e EnvironmentObject,
    s: &'e Sprite,
) -> Result<ResolvedRenderData<'e>, SpriteExportError> {
    // (1) m_sprite_atlas PPtr → SpriteAtlas.m_render_data_map[m_render_data_key]
    if let Some(atlas_pptr) = s.m_sprite_atlas.as_ref()
        && let Some(atlas_obj) = env.resolve_from(sprite_obj, atlas_pptr)
        && let ParsedObject::Typed(UnityObject::SpriteAtlas(atlas)) = &atlas_obj.object
        && let Some(rd) = find_atlas_data_by_key(atlas, s.m_render_data_key.as_ref())
    {
        return Ok(ResolvedRenderData {
            texture: &rd.texture,
            texture_rect: &rd.texture_rect,
            settings_raw: rd.settings_raw,
            downscale_multiplier: Some(rd.downscale_multiplier),
        });
        // 任一层未命中（PPtr 漂移 / 非 SpriteAtlas / key 不在 map）→ 落到 (2)/(3) 兜底。
    }

    // (2) m_atlas_tags 名称扫描同 Environment 的 SpriteAtlas（兜底，对齐 UnityPy）
    if let Some(tags) = s.m_atlas_tags.as_ref()
        && let Some(tag0) = tags.first()
        && let Some(rd) = scan_atlas_by_name(env, tag0, s.m_render_data_key.as_ref())
    {
        return Ok(ResolvedRenderData {
            texture: &rd.texture,
            texture_rect: &rd.texture_rect,
            settings_raw: rd.settings_raw,
            downscale_multiplier: Some(rd.downscale_multiplier),
        });
    }

    // (3) standalone：直接用 m_rd
    let rd: &SpriteRenderData = &s.m_rd;
    Ok(ResolvedRenderData {
        texture: &rd.texture,
        texture_rect: &rd.texture_rect,
        settings_raw: rd.settings_raw,
        downscale_multiplier: rd.downscale_multiplier,
    })
}

/// 在 `SpriteAtlas.m_render_data_map` 里按 `(Guid, i64)` key 查找 `SpriteAtlasData`。
///
/// `Guid` 未派生 `PartialEq`，按 4 个 u32 字段 + i64 手动比对。
fn find_atlas_data_by_key<'e>(
    atlas: &'e SpriteAtlas,
    key: Option<&(Guid, i64)>,
) -> Option<&'e SpriteAtlasData> {
    let (want_guid, want_idx) = key?;
    atlas
        .m_render_data_map
        .iter()
        .find(|((g, i), _)| guid_eq(g, want_guid) && *i == *want_idx)
        .map(|(_, d)| d)
}

/// `Guid` 字段级相等（未派生 `PartialEq`）。
fn guid_eq(a: &Guid, b: &Guid) -> bool {
    a.data_0 == b.data_0 && a.data_1 == b.data_1 && a.data_2 == b.data_2 && a.data_3 == b.data_3
}

/// 按 `name` 扫描全部 SpriteAtlas，再按 key 查 render data（生命周期绑定 env）。
fn scan_atlas_by_name<'e>(
    env: &'e Environment,
    name: &str,
    key: Option<&(Guid, i64)>,
) -> Option<&'e SpriteAtlasData> {
    env.objects().iter().find_map(|obj| {
        if obj.class_id != ClassId::from(ClassIDType::SpriteAtlas) {
            return None;
        }
        let ParsedObject::Typed(UnityObject::SpriteAtlas(atlas)) = &obj.object else {
            return None;
        };
        if atlas.m_name != name {
            return None;
        }
        find_atlas_data_by_key(atlas, key)
    })
}

// ── atlas Texture2D 解码 ─────────────────────────────────────────────────────

/// 解引用 PPtr → Texture2D → 解码整图为 top-down RGBA8。
///
/// 复用 `export::decode_texture` 的统一 5 路分派(astc / bcn / crunch / etc / uncompressed)。
fn decode_atlas_texture(
    env: &Environment,
    sprite_obj: &EnvironmentObject,
    texture_pptr: &PPtr,
    ver: UnityVersion,
) -> Result<(Vec<u8>, u32, u32), SpriteExportError> {
    let tex_obj = env
        .resolve_from(sprite_obj, texture_pptr)
        .ok_or(SpriteExportError::TextureNotFound)?;
    let ParsedObject::Typed(UnityObject::Texture2D(t)) = &tex_obj.object else {
        return Err(SpriteExportError::TextureNotFound);
    };
    // 统一调 export::decode_texture(消除与 main.rs / texture_export 的 5 路分派重复)。
    let img = crate::export::decode_texture(t, env, ver)?;
    Ok((img.rgba, img.width, img.height))
}

// ── 像素变换（downscale / crop / rotation）──────────────────────────────────

/// 整图最近邻下采样（`out = atlas / mul`）。
///
/// 对齐 AssetStudio `CutImage` 的 resize 语义：若 `mul > 0 && != 1`，输出尺寸
/// `round(atlas_w/mul) × round(atlas_h/mul)`，最近邻取样。AssetStudio 用 SixLabors
/// ResizeKernel 但 atlas downscale 通常为整数倍，最近邻足够且确定性。
fn apply_downscale(rgba: Vec<u8>, w: u32, h: u32, mul: f32) -> (Vec<u8>, u32, u32) {
    if !(mul > 0.0 && (mul - 1.0).abs() > f32::EPSILON) {
        return (rgba, w, h);
    }
    let nw = ((w as f32) / mul).round().max(1.0) as u32;
    let nh = ((h as f32) / mul).round().max(1.0) as u32;
    if nw == w && nh == h {
        return (rgba, w, h);
    }
    let mut out = vec![0u8; (nw as usize) * (nh as usize) * 4];
    let sx = (w as f32) / (nw as f32);
    let sy = (h as f32) / (nh as f32);
    for oy in 0..nh as usize {
        for ox in 0..nw as usize {
            let sx0 = ((ox as f32) * sx).round() as usize;
            let sy0 = ((oy as f32) * sy).round() as usize;
            let sx0 = sx0.min(w as usize - 1);
            let sy0 = sy0.min(h as usize - 1);
            let src = (sy0 * w as usize + sx0) * 4;
            let dst = (oy * nw as usize + ox) * 4;
            out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
        }
    }
    (out, nw, nh)
}

/// 在 top-down RGBA 上裁剪 `textureRect` 换算出的子区域。
///
/// 返回 `(cropped_rgba, crop_w, crop_h)`。越界部分 clamp 到边界（对齐参考实现：
/// Unity 偶有浮点 rect 超出整图 1 像素）。完全越界（w/h ≤ 0）报
/// [`SpriteExportError::RectOutOfBounds`]。
fn crop_top_down(
    rgba: &[u8],
    atlas_w: u32,
    atlas_h: u32,
    rect: &Rectf,
) -> Result<(Vec<u8>, u32, u32), SpriteExportError> {
    let aw = atlas_w as i64;
    let ah = atlas_h as i64;

    // bottom-up rect → top-down 行号换算（见模块 doc 推导）。
    let left = rect.x.floor() as i64;
    let right = (rect.x + rect.width).ceil() as i64;
    let td_top = ah - (rect.y + rect.height).ceil() as i64;
    let td_bottom = ah - rect.y.floor() as i64;

    let crop_x = left.max(0).min(aw);
    let crop_right = right.max(0).min(aw);
    let crop_y = td_top.max(0).min(ah);
    let crop_bottom = td_bottom.max(0).min(ah);

    let crop_w = crop_right - crop_x;
    let crop_h = crop_bottom - crop_y;
    if crop_w <= 0 || crop_h <= 0 {
        return Err(SpriteExportError::RectOutOfBounds {
            atlas_w,
            atlas_h,
            rect_x: left,
            rect_y: rect.y.floor() as i64,
            rect_w: right - left,
            rect_h: (rect.y + rect.height).ceil() as i64 - rect.y.floor() as i64,
        });
    }

    let cw = crop_w as usize;
    let ch = crop_h as usize;
    let aw_us = atlas_w as usize;
    let mut out = vec![0u8; cw * ch * 4];
    for row in 0..ch {
        let src_y = crop_y as usize + row;
        let src_off = (src_y * aw_us + crop_x as usize) * 4;
        let dst_off = row * cw * 4;
        out[dst_off..dst_off + cw * 4].copy_from_slice(&rgba[src_off..src_off + cw * 4]);
    }
    Ok((out, cw as u32, ch as u32))
}

/// 应用 packing rotation（在裁剪后的 sprite 子图上，top-down 空间）。
///
/// 注意 Rotate90 在 top-down 空间里走 **逆时针**（共轭翻转方向，见模块 doc 推导），
/// 与参考实现 bottom-up 空间里的顺时针 90°（PIL ROTATE_270）输出一致。Rotate90 输出
/// 维度对调（`(w, h) → (h, w)`），其余保持。
fn apply_packing_rotation(
    rgba: Vec<u8>,
    w: u32,
    h: u32,
    rot: PackingRotation,
) -> (Vec<u8>, u32, u32) {
    match rot {
        PackingRotation::None => (rgba, w, h),
        PackingRotation::FlipHorizontal => {
            let mut out = rgba;
            flip_horizontal_in_place(&mut out, w as usize, h as usize);
            (out, w, h)
        }
        PackingRotation::FlipVertical => {
            let mut out = rgba;
            flip_vertical_in_place(&mut out, w as usize, h as usize);
            (out, w, h)
        }
        PackingRotation::Rotate180 => {
            // 180° = 水平 + 垂直双翻（与 FlipH∘FlipV 等价）。
            let mut out = rgba;
            flip_horizontal_in_place(&mut out, w as usize, h as usize);
            flip_vertical_in_place(&mut out, w as usize, h as usize);
            (out, w, h)
        }
        PackingRotation::Rotate90 => {
            // top-down 空间逆时针 90°：new[ox, oy] = old[w-1-oy, ox]。
            // 输出 (nw=h, nh=w)。
            let nw = h as usize;
            let nh = w as usize;
            let w_us = w as usize;
            let mut out = vec![0u8; nw * nh * 4];
            for oy in 0..nh {
                for ox in 0..nw {
                    let sx = w_us - 1 - oy;
                    let sy = ox;
                    let src = (sy * w_us + sx) * 4;
                    let dst = (oy * nw + ox) * 4;
                    out[dst..dst + 4].copy_from_slice(&rgba[src..src + 4]);
                }
            }
            (out, nw as u32, nh as u32)
        }
    }
}

/// 行内水平翻转（in-place，4 字节像素为单位）。
fn flip_horizontal_in_place(buf: &mut [u8], width: usize, height: usize) {
    let stride = width * 4;
    for row in 0..height {
        let r = &mut buf[row * stride..row * stride + stride];
        let mut lo = 0usize;
        let mut hi = width;
        while lo < hi {
            hi -= 1;
            if lo == hi {
                break;
            }
            for k in 0..4 {
                r.swap(lo * 4 + k, hi * 4 + k);
            }
            lo += 1;
        }
    }
}

// ── 公共 API ─────────────────────────────────────────────────────────────────

/// 把 Sprite 的像素切成 RGBA8（top-down）。
///
/// 返回 `(rgba, width, height)`——packing rotation 后维度可能对调。复用
/// [`crate::texture_export`] 的全部解码原语；PPtr 解引用走 `Environment::resolve_from`。
pub fn sprite_pixel_bytes(
    env: &Environment,
    sprite_obj: &EnvironmentObject,
    s: &Sprite,
    ver: UnityVersion,
) -> Result<(Vec<u8>, u32, u32), SpriteExportError> {
    // 1) 解析 render data 来源（atlas / standalone）。
    let rd = resolve_render_data(env, sprite_obj, s)?;

    // 2) 解码 atlas Texture2D（top-down RGBA）。
    let (mut rgba, mut atlas_w, mut atlas_h) =
        decode_atlas_texture(env, sprite_obj, rd.texture, ver)?;

    // 3) downscale multiplier（整图 resize，对齐参考实现顺序：先 resize 再 crop）。
    if let Some(mul) = rd.downscale_multiplier {
        let (r, w, h) = apply_downscale(rgba, atlas_w, atlas_h, mul);
        rgba = r;
        atlas_w = w;
        atlas_h = h;
    }

    // 4) tight packing 检查（第一阶段跳过）。
    let settings = SpriteSettings(rd.settings_raw);
    if settings.packed() && settings.packing_mode_tight() {
        return Err(SpriteExportError::UnsupportedTight);
    }

    // 5) 裁剪 textureRect（top-down 空间换算）。
    let (cropped, cw, ch) = crop_top_down(&rgba, atlas_w, atlas_h, rd.texture_rect)?;

    // 6) packing rotation（top-down 空间应用，共轭方向见模块 doc）。
    let rot = if settings.packed() {
        settings.packing_rotation()
    } else {
        PackingRotation::None
    };
    let (final_rgba, fw, fh) = apply_packing_rotation(cropped, cw, ch, rot);

    Ok((final_rgba, fw, fh))
}

/// 解码 + 写出单个 Sprite 到 `<out>/<name>.png`，返回实际文件路径。
pub fn export_sprite(
    env: &Environment,
    sprite_obj: &EnvironmentObject,
    s: &Sprite,
    name: &str,
    ver: UnityVersion,
    out: &Path,
) -> Result<PathBuf, SpriteExportError> {
    let (rgba, w, h) = sprite_pixel_bytes(env, sprite_obj, s, ver)?;
    let file = out.join(sanitize_filename(name)).with_extension("png");
    let mut f = std::fs::File::create(&file).map_err(|source| SpriteExportError::Texture {
        source: TextureExportError::PngIo { source },
    })?;
    write_png(rgba, w, h, &mut f)?;
    Ok(file)
}

/// 文件名非法字符替换（与 CLI 层 `sanitize_filename` 一致）。
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '[' | ']' => '_',
            _ => c,
        })
        .collect()
}

// ── 单元测试 ─────────────────────────────────────────────────────────────────
//
// 策略与 `texture_export.rs` 的字节级单测一致：不依赖 fixture，构造确定性输入断言
// RGBA 精确匹配。测试对象是私有像素变换函数（crop / rotation / downscale / clamp），
// 故放 `#[cfg(test)] mod` 以便访问内部。
#[cfg(test)]
mod tests {
    use super::*;
    use generated::subclasses::rectf::Rectf;

    /// 构造一个 `w×h` 的 RGBA 缓冲，每像素 RGBA = (x, y, 0, 255)（位置可辨）。
    fn pos_buf(w: u32, h: u32) -> Vec<u8> {
        let mut buf = vec![0u8; (w as usize) * (h as usize) * 4];
        for y in 0..h as usize {
            for x in 0..w as usize {
                let i = (y * w as usize + x) * 4;
                buf[i] = x as u8;
                buf[i + 1] = y as u8;
                buf[i + 2] = 0;
                buf[i + 3] = 255;
            }
        }
        buf
    }

    fn rect(x: f32, y: f32, w: f32, h: f32) -> Rectf {
        Rectf { x, y, width: w, height: h }
    }

    /// 读 (x, y) 处像素的 RGBA 元组。
    fn px(buf: &[u8], w: usize, x: usize, y: usize) -> (u8, u8, u8, u8) {
        let i = px_idx(w, x, y);
        (buf[i], buf[i + 1], buf[i + 2], buf[i + 3])
    }

    /// (x, y) 像素在 RGBA flat 缓冲里的字节索引。
    fn px_idx(w: usize, x: usize, y: usize) -> usize {
        (y * w + x) * 4
    }

    // ── crop_top_down ──

    #[test]
    fn crop_basic_top_down_y_conversion() {
        // 4×4 atlas，textureRect (x=1, y=1, w=2, h=2)（Unity bottom-up 坐标）。
        // bottom-up: y∈[1,3), x∈[1,3)。换算到 top-down: 行 y=1,2 → td 行 4-3=1, 4-2=2。
        // 即裁 top-down 的 (1,1)-(3,3) 这块 2×2。
        let atlas = pos_buf(4, 4);
        let (out, w, h) = crop_top_down(&atlas, 4, 4, &rect(1.0, 1.0, 2.0, 2.0)).unwrap();
        assert_eq!((w, h), (2, 2));
        // top-down 行 0 对应 atlas 行 1，行 1 对应 atlas 行 2。
        assert_eq!(px(&out, 2, 0, 0), px(&atlas, 4, 1, 1)); // atlas (1,1)
        assert_eq!(px(&out, 2, 1, 0), px(&atlas, 4, 2, 1)); // atlas (2,1)
        assert_eq!(px(&out, 2, 0, 1), px(&atlas, 4, 1, 2)); // atlas (1,2)
        assert_eq!(px(&out, 2, 1, 1), px(&atlas, 4, 2, 2)); // atlas (2,2)
    }

    #[test]
    fn crop_full_atlas() {
        let atlas = pos_buf(3, 3);
        let (out, w, h) = crop_top_down(&atlas, 3, 3, &rect(0.0, 0.0, 3.0, 3.0)).unwrap();
        assert_eq!((w, h), (3, 3));
        // 整图裁出应与原图一致（top-down 空间）。
        assert_eq!(out, atlas);
    }

    #[test]
    fn crop_clamps_out_of_bounds_by_one_pixel() {
        // textureRect 超出 atlas 1 像素（Unity 偶有浮点 rect），应 clamp 不报错。
        let atlas = pos_buf(4, 4);
        let (out, w, h) = crop_top_down(&atlas, 4, 4, &rect(3.0, 3.0, 2.0, 2.0)).unwrap();
        // x∈[3,5]→clamp[3,4]=1px；y bottom-up [3,5]→td [4-5,4-3]=[-1,1]→clamp[0,1]=1px。
        assert_eq!((w, h), (1, 1));
        assert_eq!(px(&out, 1, 0, 0), px(&atlas, 4, 3, 0)); // atlas (3,0)
    }

    #[test]
    fn crop_fully_out_of_bounds_errors() {
        let atlas = pos_buf(4, 4);
        // 完全在 atlas 右下角之外。
        let err = crop_top_down(&atlas, 4, 4, &rect(10.0, 10.0, 2.0, 2.0)).unwrap_err();
        assert!(matches!(err, SpriteExportError::RectOutOfBounds { .. }));
    }

    #[test]
    fn crop_zero_size_errors() {
        let atlas = pos_buf(4, 4);
        let err = crop_top_down(&atlas, 4, 4, &rect(1.0, 1.0, 0.0, 2.0)).unwrap_err();
        assert!(matches!(err, SpriteExportError::RectOutOfBounds { .. }));
    }

    // ── apply_packing_rotation ──

    #[test]
    fn rotation_flip_horizontal() {
        // 2×1: [A | B] → FlipH → [B | A]
        let mut buf = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        flip_horizontal_in_place(&mut buf, 2, 1);
        assert_eq!(&buf, &[5, 6, 7, 8, 1, 2, 3, 4]);
    }

    #[test]
    fn rotation_flip_vertical_swaps_rows() {
        // 1×2 两行 A/B → FlipV 交换两行。
        let buf = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let (out, w, h) = apply_packing_rotation(buf, 1, 2, PackingRotation::FlipVertical);
        assert_eq!((w, h), (1, 2));
        // 行 0 原 [5,6,7,8]，行 1 原 [1,2,3,4]。
        assert_eq!(&out, &[5, 6, 7, 8, 1, 2, 3, 4]);
    }

    #[test]
    fn rotation_rotate180() {
        // 2×1 [A | B] → 180° → [B' | A']（每个像素自身不变，位置翻转）。
        // 单行 + FlipH∘FlipV：FlipV 单行 noop，故等同 FlipH。
        let buf = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let (out, w, h) = apply_packing_rotation(buf, 2, 1, PackingRotation::Rotate180);
        assert_eq!((w, h), (2, 1));
        assert_eq!(&out, &[5, 6, 7, 8, 1, 2, 3, 4]);
    }

    #[test]
    fn rotation_none_is_identity() {
        let buf = pos_buf(2, 2);
        let (out, w, h) = apply_packing_rotation(buf.clone(), 2, 2, PackingRotation::None);
        assert_eq!((w, h), (2, 2));
        assert_eq!(out, buf);
    }

    #[test]
    fn rotation_rotate90_ccw_conjugate() {
        // 2×2 位置编码 atlas（每个像素值 = x*10+y 便于辨识，存进 R 通道）：
        //   top-down:        Rotate90（共轭 = top-down 逆时针 90°）应得：
        //   [[0,1],[2,3]]    [[1,3],[0,2]]
        // 推导：ccw90 new[ox,oy] = old[w-1-oy, ox]（w=2）。
        //   new[0,0]=old[1,0]=1, new[1,0]=old[1,1]=3,
        //   new[0,1]=old[0,0]=0, new[1,1]=old[0,1]=2.
        let mut buf = vec![0u8; 16];
        for (x, y, v) in [(0u8, 0u8, 0u8), (1, 0, 1), (0, 1, 2), (1, 1, 3)] {
            buf[(y as usize * 2 + x as usize) * 4] = v;
            buf[(y as usize * 2 + x as usize) * 4 + 3] = 255;
        }
        let (out, w, h) = apply_packing_rotation(buf, 2, 2, PackingRotation::Rotate90);
        assert_eq!((w, h), (2, 2), "Rotate90 维度对调");
        // 期望 [[1,3],[0,2]]（flat index = (y*w + x)*4，w=2）。
        assert_eq!(out[px_idx(2, 0, 0)], 1); // new[0,0]
        assert_eq!(out[px_idx(2, 1, 0)], 3); // new[1,0]
        assert_eq!(out[px_idx(2, 0, 1)], 0); // new[0,1]
        assert_eq!(out[px_idx(2, 1, 1)], 2); // new[1,1]
    }

    #[test]
    fn rotation_rotate90_dimensions_swap() {
        // 3×2 → Rotate90 → 2×3。
        let buf = pos_buf(3, 2);
        let (_, w, h) = apply_packing_rotation(buf, 3, 2, PackingRotation::Rotate90);
        assert_eq!((w, h), (2, 3));
    }

    // ── apply_downscale ──

    #[test]
    fn downscale_noop_when_mul_is_one() {
        let buf = pos_buf(4, 4);
        let (out, w, h) = apply_downscale(buf.clone(), 4, 4, 1.0);
        assert_eq!((w, h), (4, 4));
        assert_eq!(out, buf);
    }

    #[test]
    fn downscale_noop_when_mul_zero_or_negative() {
        // mul ≤ 0 应视为不缩放（保护）。
        let buf = pos_buf(4, 4);
        let (out, w, h) = apply_downscale(buf.clone(), 4, 4, 0.0);
        assert_eq!((w, h), (4, 4));
        assert_eq!(out, buf);
    }

    #[test]
    fn downscale_by_two_halves_dimensions() {
        // 4×4 → /2 → 2×2，最近邻取样（sx=sy=2.0，取 (0,0)(2,0)(0,2)(2,2)）。
        let buf = pos_buf(4, 4);
        let (out, w, h) = apply_downscale(buf, 4, 4, 2.0);
        assert_eq!((w, h), (2, 2));
        assert_eq!(px(&out, 2, 0, 0), (0, 0, 0, 255)); // atlas (0,0)
        assert_eq!(px(&out, 2, 1, 0), (2, 0, 0, 255)); // atlas (2,0)
        assert_eq!(px(&out, 2, 0, 1), (0, 2, 0, 255)); // atlas (0,2)
        assert_eq!(px(&out, 2, 1, 1), (2, 2, 0, 255)); // atlas (2,2)
    }

    // ── SpriteSettings bitfield ──

    #[test]
    fn settings_bitfield_decode() {
        // 构造一个 settings_raw：packed=1 (bit0), mode=Rectangle (bit1=1),
        // rotation=FlipV (bits2-5 = 2 → 2<<2 = 8)。
        let s = SpriteSettings(1 | (1 << 1) | (2 << 2)); // 1 + 2 + 8 = 11
        assert!(s.packed());
        assert!(!s.packing_mode_tight()); // Rectangle
        assert_eq!(s.packing_rotation(), PackingRotation::FlipVertical);

        // packed=0 → rotation 不应用（None）。
        let s2 = SpriteSettings(2 << 2); // 8, packed=0
        assert!(!s2.packed());

        // Tight mode (bit1=0)。
        let s3 = SpriteSettings(1); // packed=1, bit1=0 → Tight
        assert!(s3.packed());
        assert!(s3.packing_mode_tight());

        // Rotate90 (bits2-5 = 4)。
        let s4 = SpriteSettings(1 | (1 << 1) | (4 << 2)); // 1+2+16=19
        assert_eq!(s4.packing_rotation(), PackingRotation::Rotate90);
    }

    // ── guid_eq（Guid 未派生 PartialEq，手动字段比对）──

    #[test]
    fn guid_equality_fieldwise() {
        use generated::subclasses::guid::Guid;
        let a = Guid { data_0: 1, data_1: 2, data_2: 3, data_3: 4 };
        let b = Guid { data_0: 1, data_1: 2, data_2: 3, data_3: 4 };
        let c = Guid { data_0: 9, data_1: 2, data_2: 3, data_3: 4 };
        assert!(guid_eq(&a, &b));
        assert!(!guid_eq(&a, &c));
    }
}
