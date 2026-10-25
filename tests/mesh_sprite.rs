//! CLI codec 边界上的真实样本回归。可选游戏样本不加入仓库。
#![cfg(feature = "export")]
use unity_asset::{
    CompressionType, Decompressor, Error, Loader, ParsedObject, Registry, types::UnityObject,
};
struct Ak;
impl Decompressor for Ak {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>, Error> {
        let mut out = vec![0; expected];
        lz4inv::decompress(input, &mut out).map_err(|e| Error::Decompress {
            reason: e.to_string(),
        })?;
        Ok(out)
    }
}
#[test]
fn mesh_and_sprite_samples() {
    let mut registry = Registry::defaults();
    registry.register(CompressionType(4), std::sync::Arc::new(Ak));
    registry.register(CompressionType(5), std::sync::Arc::new(Ak));
    let mut meshes = 0;
    let mut sprites = 0;
    let mut tight = 0;
    let mut visible = 0;
    for name in [
        "chararts_char_002_amiya.ab",
        "battlecommon.ab",
        "enm_art_21.ab",
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("example")
            .join(name);
        if !path.exists() {
            continue;
        }
        let report = Loader::builder()
            .registry(&registry)
            .path(path)
            .build()
            .unwrap();
        let env = report.environment();
        for obj in env.objects() {
            match &obj.object {
                ParsedObject::Typed(UnityObject::Mesh(m)) => {
                    let decoded = env
                        .decode_mesh(obj)
                        .unwrap_or_else(|e| panic!("{name}/{}: {e}", m.m_name));
                    assert!(!decoded.positions.is_empty());
                    meshes += 1;
                }
                ParsedObject::Typed(UnityObject::Sprite(s)) => {
                    let ver = env.collections()[obj.collection.to_index()].unity_version;
                    let (rgba, w, h) =
                        unity_asset::sprite_export::sprite_pixel_bytes(env, obj, s, ver)
                            .unwrap_or_else(|e| panic!("{name}/{}: {e}", s.m_name));
                    assert_eq!(rgba.len(), w as usize * h as usize * 4);
                    assert!(w > 0 && h > 0);
                    sprites += 1;
                    if s.m_rd.settings_raw & 2 == 0 {
                        tight += 1;
                    }
                    if rgba.chunks_exact(4).any(|p| p[3] != 0) {
                        visible += 1;
                    }
                }
                _ => {}
            }
        }
    }
    eprintln!(
        "真实样本解码: {meshes} Mesh / {sprites} Sprite / {tight} Tight / {visible} visible（样本缺失时依赖自包含单测覆盖）"
    );
}
