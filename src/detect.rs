//! bundle 文件类型嗅探。目录扫描时判断一个文件是否为可加载的 UnityFS bundle。
//!
//! 对齐 AssetStudio `FileReader.CheckFileType`，但本轮只处理 UnityFS 一种 bundle
//! 容器：gzip(`1f 8b`) / brotli / `UnityWeb` / `UnityRaw` / `UnityArchive` 等签名
//! 暂不支持，遇到即跳过（见 gzip 路线图，`docs/environment-layer.md`）。

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{Error, Result};

/// UnityFS bundle 的签名（大端，ASCII）。前 7 字节。
const UNITYFS_MAGIC: &[u8] = b"UnityFS";

/// 嗅探字节切片是否以 `UnityFS` 签名开头。
///
/// 保留为字节级原语（`load_environment_dir` 走 [`looks_like_unityfs_path`] 避免全量读文件），
/// 供未来 `load_environment_bytes`-类入口复用。
#[allow(dead_code)]
pub(crate) fn looks_like_unityfs(bytes: &[u8]) -> bool {
    bytes.len() >= UNITYFS_MAGIC.len() && &bytes[..UNITYFS_MAGIC.len()] == UNITYFS_MAGIC
}

/// 读文件前 7 字节，判断是否为 UnityFS bundle。
///
/// 只读头部（不全量 `fs::read`），避免对大文件做无用 IO。文件不足 7 字节、打开失败、
/// 或签名不匹配均返回 `Ok(false)`——目录扫描中静默跳过这些文件而非报错。
///
/// 返回 `Err` 仅当读 IO 失败（非「文件太短」，太短视为非 bundle）。
pub(crate) fn looks_like_unityfs_path(path: &Path) -> Result<bool> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            // 打不开（权限/符号链接断裂等）：warn 并跳过，不阻断整个目录扫描。
            log::warn!("detect: 打开失败，跳过 {}: {e}", path.display());
            return Ok(false);
        }
    };
    let mut head = [0u8; 7];
    match f.read(&mut head) {
        Ok(n) => Ok(n >= 7 && head.as_slice() == UNITYFS_MAGIC),
        Err(e) => Err(Error::Io { source: e }),
    }
}
