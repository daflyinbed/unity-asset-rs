//! unity-asset-codegen：从 TPK 生成类型化 Rust 结构与 Parser。
//!
//! 详见 `docs/codegen.md`。

pub mod emit;
pub mod ir;
pub mod names;
pub mod pipeline;
pub mod rename;
pub mod tpk;

pub use ir::{ClassKey, EmitClass, Pipeline};
pub use pipeline::run_pipeline;
pub use tpk::{TpkFile, TpkTypeTree, UnityVersion};
