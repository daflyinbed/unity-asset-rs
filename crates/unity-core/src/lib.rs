//! 共享原语：`UnityVersion` + `Endian` + `EndianReader` trait + `Parse` trait + `PPtr`。
//!
//! 主 crate (`unity-asset-rs`) 和 generated crate (`unity-asset-generated`) 共同依赖此 crate，
//! 消除类型重复定义。

mod pptr;
mod reader;
mod value;
mod version;

pub use pptr::PPtr;
pub use reader::{Endian, EndianReader, Parse};
pub use value::{PPtrValue, ToValue, Value, inspect_array, inspect_bytes, opt_field};
pub use version::UnityVersion;
