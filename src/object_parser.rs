//! 对象解析 dispatch：非 MonoBehaviour → typed parse（失败回退到 TypeTree），
//! MonoBehaviour → TypeTree 解释器。
//!
//! 参见 `docs/object-parsing-design.md` §6/§10。

use crate::cursor::Cursor;
use crate::error::{Error, Result};
use crate::serialized_file::{ObjectInfo, SerializedFile};
use crate::typetree::{interpret, RefTypeTable, Value};
use crate::Endian;
use generated::{ClassIDType, UnityObject};
use unity_core::Parse;

/// 解析结果：typed struct 或动态 Value。
#[derive(Debug)]
pub enum ParsedObject {
    /// codegen'd typed struct（Texture2D, Mesh, ...）
    Typed(UnityObject),
    /// TypeTree 解释器产出（MonoBehaviour 或 typed parse 失败回退）
    Dynamic(Value),
}

/// 解析单个对象。
///
/// 由 `SerializedFile` 的调用方逐个调用。
pub fn parse_object(
    sf: &SerializedFile,
    obj: &ObjectInfo,
    refs: &RefTypeTable,
) -> Result<ParsedObject> {
    let data = obj.data(sf);
    let class_id = obj.class_id(sf);
    let ver = &sf.unity_version_parsed;
    let endian = sf.endian;

    // 获取该对象类型的 AB 内嵌 TypeTree（如果有）
    let ab_tree = sf.types.get(obj.type_idx).and_then(|t| t.node.as_ref());

    // ── MonoBehaviour (classID=114)：直接走 TypeTree ──
    if class_id == 114 {
        let tree = ab_tree.ok_or(Error::MonoBehaviourWithoutTypeTree)?;
        let value = interpret(data, tree, ver, endian, refs)?;
        return Ok(ParsedObject::Dynamic(value));
    }

    // ── 其他类型：先试 typed parse ──
    match try_typed(class_id, data, ver, endian) {
        Ok(typed) => Ok(ParsedObject::Typed(typed)),
        Err(typed_err) => {
            // 回退到 TypeTree 解释器
            if let Some(tree) = ab_tree {
                log::warn!(
                    "classID {class_id}: typed parse failed ({typed_err}), \
                     falling back to TypeTree interpreter"
                );
                let value = interpret(data, tree, ver, endian, refs)?;
                Ok(ParsedObject::Dynamic(value))
            } else {
                Err(typed_err)
            }
        }
    }
}

/// 尝试 codegen'd typed parse。内部创建 Cursor。
fn try_typed(
    class_id: i32,
    data: &[u8],
    ver: &unity_core::UnityVersion,
    endian: Endian,
) -> Result<UnityObject> {
    let class = ClassIDType::from_i32(class_id).ok_or(Error::UnknownClassId { id: class_id })?;

    let mut cursor = Cursor::new(data, endian);
    let obj = UnityObject::parse(class, &mut cursor, ver)?;
    Ok(obj)
}
