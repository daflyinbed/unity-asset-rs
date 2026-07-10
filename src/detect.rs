//! bundle 文件类型嗅探。目录扫描时判断一个文件是否为可加载的 Unity 资产文件。
//!
//! 对齐 AssetStudio `FileReader.CheckFileType`。当前嗅探 UnityFS + gzip 两种最常见
//! 外层格式（gzip bundle 解压后内层仍是 UnityFS）。brotli / `UnityWeb` / `UnityRaw`
//! / `UnityArchive` 等签名需全量读后 sniff（见 `format::sniff`），目录扫描为避免
//! 全量读大文件暂只走头部 magic。

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{Error, Result};

/// UnityFS bundle 的签名（大端，ASCII）。前 7 字节。
const UNITYFS_MAGIC: &[u8] = b"UnityFS";

/// gzip magic（前 2 字节）。
const GZIP_MAGIC: [u8; 2] = [0x1F, 0x8B];

/// 嗅探字节切片是否以 `UnityFS` 签名开头。
///
/// 保留为字节级原语，供未来字节级扫描入口复用。
#[allow(dead_code)]
pub(crate) fn looks_like_unityfs(bytes: &[u8]) -> bool {
    bytes.len() >= UNITYFS_MAGIC.len() && &bytes[..UNITYFS_MAGIC.len()] == UNITYFS_MAGIC
}

/// 读文件头部，判断是否为可加载格式（UnityFS 或 gzip）。
///
/// 只读头部少量字节（不全量 `fs::read`），避免对大文件做无用 IO。文件不足、打开失败、
/// 或签名不匹配均返回 `Ok(false)`——目录扫描中静默跳过这些文件而非报错。
///
/// 返回 `Err` 仅当读 IO 失败（非「文件太短」，太短视为非可加载）。
pub(crate) fn looks_like_loadable_path(path: &Path) -> Result<bool> {
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
        Ok(n) => {
            // UnityFS: 前 7 字节 == "UnityFS"
            if n >= 7 && &head[..7] == UNITYFS_MAGIC {
                return Ok(true);
            }
            // gzip: 前 2 字节 == 0x1F 0x8B
            if n >= 2 && head[..2] == GZIP_MAGIC {
                return Ok(true);
            }
            Ok(false)
        }
        Err(e) => Err(Error::Io { source: e }),
    }
}
