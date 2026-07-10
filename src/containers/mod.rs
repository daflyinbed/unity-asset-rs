//! 容器格式解析子模块。
//!
//! 每种容器格式一个模块，产出 `Vec<RawEntry>`（原始子字节列表），供递归驱动器
//! `crate::format::load_recursive` 对每个 entry 重新 sniff + parse。
//!
//! - [`rawweb`]：UnityWeb / UnityRaw（旧版 bundle，大端）
//! - [`webfile`]：UnityWebData（WebGL 数据格式，小端）
//! - [`compressed`]：gzip / brotli 外层包裹

pub mod compressed;
pub mod rawweb;
pub mod webfile;
