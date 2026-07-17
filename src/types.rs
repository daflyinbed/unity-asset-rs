//! codegen typed struct 的稳定出口(高级用户用)。
//!
//! 默认入口 [`crate::Loader`] / [`crate::Environment`] 已封装 typed 解析;普通用户
//! 通过 [`crate::EnvironmentObject`] / [`crate::ParsedObject`] 访问即可。本模块提供给
//! 需要直接模式匹配 [`UnityObject`] 变体、或查询 [`ClassIDType`] 的高级用户。

pub use generated::{ClassIDType, UnityObject};
