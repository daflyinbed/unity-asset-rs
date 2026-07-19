//! bundle 文件类型嗅探。目录扫描时判断一个文件是否为可加载的 Unity 资产文件。
//!
//! 对齐 AssetStudio `FileReader.CheckFileType`。嗅探 UnityFS + gzip + brotli 三种
//! 最常见外层格式（gzip / brotli bundle 解压后内层仍是 UnityFS）。`UnityWeb` /
//! `UnityRaw` / `UnityArchive` 等签名需全量读后 sniff（见 `format::sniff`），目录
//! 扫描为避免全量读大文件暂只走头部 magic。
//!
//! 头部缓冲区大小取 64 字节：UnityFS + gzip magic 各只需前几字节，brotli 验证需
//! 帧头前缀（2 字节）+ 变长 size_bytes（最多 3 字节）+ 35 字节签名 ≈ 40 字节，
//! 64 字节留足余量。对 brotli 而言读不足 40 字节的小文件会被
//! [`crate::containers::compressed::looks_like_brotli`] 判否（内部有长度检查），
//! 行为正确。

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::containers::compressed::looks_like_brotli;
use crate::error::{Error, Result};

/// 目录扫描嗅探时读取的头部字节数（详见模块 doc）。
const HEAD_LEN: usize = 64;

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

/// 读文件头部，判断是否为可加载格式（UnityFS / gzip / brotli）。
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
    let mut head = [0u8; HEAD_LEN];
    match f.read(&mut head) {
        Ok(n) => {
            // UnityFS: 前 7 字节 == "UnityFS"
            if n >= UNITYFS_MAGIC.len() && &head[..UNITYFS_MAGIC.len()] == UNITYFS_MAGIC {
                return Ok(true);
            }
            // gzip: 前 2 字节 == 0x1F 0x8B
            if n >= GZIP_MAGIC.len() && head[..GZIP_MAGIC.len()] == GZIP_MAGIC {
                return Ok(true);
            }
            // brotli: 解析帧头 + 元数据块签名（需 ~40 字节，不足由内部长度检查判否）。
            if n >= 4 && looks_like_brotli(&head[..n]) {
                return Ok(true);
            }
            Ok(false)
        }
        Err(e) => Err(Error::Io { source: e }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 构造一个合法的 Unity brotli 流头部（仅供测试 `looks_like_brotli` 判真）。
    ///
    /// 帧布局（对齐 `compressed::looks_like_brotli` 的位运算）：
    /// - bytes[0]：WBITS 字节（任意，取 `0x01`）
    /// - bytes[1]：`bt`，`size_bytes = bt & 0x3 == 1`，且 `(bt >> 2)` 为签名长度低 6 位。
    ///   签名长 36 → `bt = (36 << 2) | 1 = 0x91`
    /// - bytes[2]：`nbt`，要求 `nbt & 0x3 == 0`（高 2 位贡献为 0），取 `0x00`
    /// - bytes[3..]：36 字节签名 `b"UnityWeb Compressed Content (brotli)"`
    fn make_brotli_head() -> Vec<u8> {
        let mut v = vec![0x01, 0x91, 0x00];
        v.extend_from_slice(b"UnityWeb Compressed Content (brotli)");
        v
    }

    /// 写临时文件（`std::env::temp_dir()` + 唯一名），返回路径。调用方负责清理。
    fn write_temp(name: &str, content: &[u8]) -> std::path::PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let n = SEQ.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let path = std::env::temp_dir().join(format!("unity-asset-rs-detect-{pid}-{n}-{name}"));
        std::fs::write(&path, content).expect("写临时文件失败");
        path
    }

    #[test]
    fn unityfs_head_detected() {
        let mut content = b"UnityFS".to_vec();
        content.extend_from_slice(&[0; 20]); // 随便补点尾部
        let path = write_temp("unityfs.bin", &content);
        assert!(
            looks_like_loadable_path(&path).unwrap(),
            "UnityFS 头部应被识别为可加载"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn gzip_head_detected() {
        // gzip magic 0x1F 0x8B + 随便补尾部
        let content = [0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];
        let path = write_temp("gzip.bin", &content);
        assert!(
            looks_like_loadable_path(&path).unwrap(),
            "gzip 头部应被识别为可加载"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn brotli_head_detected() {
        let content = make_brotli_head();
        let path = write_temp("brotli.bin", &content);
        assert!(
            looks_like_loadable_path(&path).unwrap(),
            "brotli 头部应被识别为可加载"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn non_bundle_file_rejected() {
        let content = b"hello world, not a bundle";
        let path = write_temp("plain.txt", content);
        assert!(
            !looks_like_loadable_path(&path).unwrap(),
            "普通文本不应被识别为可加载"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn empty_file_rejected() {
        let path = write_temp("empty.bin", b"");
        assert!(
            !looks_like_loadable_path(&path).unwrap(),
            "空文件不应被识别为可加载"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn nonexistent_file_returns_false() {
        let path = std::env::temp_dir().join("unity-asset-rs-does-not-exist-xyz");
        assert!(
            !looks_like_loadable_path(&path).unwrap(),
            "不存在的文件应返回 Ok(false)，不报错"
        );
    }
}
