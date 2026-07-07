//! 自动生成的 Unity 类型与 parser。请勿手动修改。
//!
//! 由 `crates/codegen` 生成（详见 `docs/codegen-design.md`）。
//! 共享原语（UnityVersion / EndianReader / Parse / PPtr）来自 `unity-core` crate。

pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;

// 从 unity-core re-export，使生成代码中的 `crate::UnityVersion`、`crate::Parse`、
// `crate::EndianReader`、`crate::PPtr` 等引用仍然解析正确。
pub use unity_core::{Endian, EndianReader, Parse, PPtr, UnityVersion};
