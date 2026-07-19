//! 运行时状态:已加载 bundle 的 `Environment` 容器。
//!
//! 每个 bundle 对应一个独立的 `Arc<Environment>`(库刻意不做跨 bundle 合并)。
//! 读操作拿读锁 → `Arc::clone` → 立刻释放锁,避免长持锁阻塞并发加载。
//! `Environment` 所有读/导出方法都是 `&self`,无需暴露 `&mut`。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use unity_asset::Environment;
use unity_asset::SourceId;

/// 全局共享状态,放进 axum `State`。
#[derive(Clone)]
pub struct AppState {
    /// 已加载的 bundle:key = 用户 POST /load 时给的 id(或文件名),value = 共享 Environment。
    pub bundles: Arc<RwLock<Bundles>>,
    /// bundle 文件根目录(POST /load 的 `path` 相对它解析)。
    pub root: Arc<std::path::PathBuf>,
    /// 桌面端当前激活的唯一 Environment。project 只存在于 Electron；Rust 只记录
    /// client source id 到运行时 SourceId 的映射。
    pub active: Arc<RwLock<ActiveEnvironment>>,
    /// 串行化 source mutation；解析仍在锁外进行，commit 只短暂持写锁。
    pub mutation: Arc<tokio::sync::Mutex<()>>,
}

pub struct SourceRecord {
    pub runtime_id: SourceId,
    pub path: std::path::PathBuf,
    pub is_directory: bool,
    pub arknights: bool,
    /// 该 source 本次实际通过 Loader 格式嗅探并进入解析的磁盘文件。
    pub files: Vec<std::path::PathBuf>,
}

pub struct ActiveEnvironment {
    pub environment: Environment,
    pub sources: HashMap<String, SourceRecord>,
    pub revision: u64,
}

impl Default for ActiveEnvironment {
    fn default() -> Self {
        Self {
            environment: Environment::empty(),
            sources: HashMap::new(),
            revision: 0,
        }
    }
}

/// 已加载 bundle 容器。
#[derive(Default)]
pub struct Bundles {
    pub map: HashMap<String, Arc<Environment>>,
}

impl Bundles {
    /// 取 bundle 的共享句柄。读锁立即释放,返回独立的 `Arc`。
    pub fn get(&self, id: &str) -> Option<Arc<Environment>> {
        self.map.get(id).cloned()
    }
}

impl AppState {
    /// 按主请求作用域的便捷取法:读锁 → clone Arc → 释放。
    pub fn get_bundle(&self, id: &str) -> Option<Arc<Environment>> {
        self.bundles.read().get(id)
    }
}
