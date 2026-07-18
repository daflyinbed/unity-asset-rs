//! Sprite 导出集成测试（受 `export` feature 门控）。
//!
//! 与 `texture_export.rs` 的策略一致：**字节级单测在 `src/sprite_export.rs` 的 `#[cfg(test)]
//! mod tests` 里**（覆盖 crop / rotation / downscale / clamp / settings bitfield），这里
//! 只做公共 API 表面存在性的冒烟（确保 `pub` 项在 release 中不被意外改名/移除）。
//!
//! 真正的端到端（加载含 Sprite 的 bundle → `export_sprite` → 校验 PNG）需要带 Sprite
//! (classID 213) 的 fixture；现有 `example/` 不含 Sprite 资产（只有 SpriteRenderer
//! 引用 + GameObject），按 AGENTS.md 约定不新增/修改 fixture，故端到端暂缺，TODO 待补。

#![cfg(feature = "export")]

/// 公共 API 表面冒烟：确保 `pub` 项存在且签名稳定（编译期检查为主）。
///
/// 只做 `use` 导入 + 触达，不调用（无 Environment / Sprite 构造）。函数指针别名形式避免
/// clippy `type_complexity`。
#[test]
fn public_api_surface_compiles() {
    use unity_asset::sprite_export::{SpriteExportError, export_sprite, sprite_pixel_bytes};

    // 触达符号，确保未被 rename / 移除；取地址强转为函数项类型即可。
    let _ = sprite_pixel_bytes as fn(_, _, _, _) -> Result<(Vec<u8>, u32, u32), SpriteExportError>;
    let _ = export_sprite as fn(_, _, _, _, _, _) -> Result<std::path::PathBuf, SpriteExportError>;
}
