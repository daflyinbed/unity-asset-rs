//! 自动生成的 Unity 类型与 parser。请勿手动修改。
//!
//! 由 `crates/codegen` 生成（详见 `docs/codegen-design.md`）。
//! 共享原语（UnityVersion / EndianReader / Parse / PPtr）来自 `unity-core` crate。
//!
//! 本 crate 全部为机器生成代码，下列 lint 是 codegen 的已知副产物
//! （多余括号 / 未用版本参数 / 版本门控默认值被覆盖 / 含数字的类型名），
//! 统一在 crate 根静音，避免淹没调用方的真实告警。
#![allow(unused_parens, unused_imports, unused_variables, unused_assignments, non_camel_case_types)]

pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;

// 从 unity-core re-export，使生成代码中的 `crate::UnityVersion`、`crate::Parse`、
// `crate::EndianReader`、`crate::PPtr` 等引用仍然解析正确。
pub use unity_core::{Endian, EndianReader, Parse, PPtr, UnityVersion};
