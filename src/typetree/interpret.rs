//! TypeTree 解释器：按 `TypeNode` 树递归读字节，产出 `Value`。
//!
//! 对应 UnityPy `TypeTreeHelper.read_value`。

use crate::cursor::{Cursor, Endian};
use crate::error::{NegativeArrayLenSnafu, RefTypeNotFoundSnafu, Result, TypeTreeSizeMismatchSnafu};
use crate::typetree::{PPtrValue, RefTypeTable, TypeKind, TypeNode, Value};
use crate::version::UnityVersion;

const ALIGN_MASK: u32 = 0x4000;

/// 解释器上下文（Copy，因为只含不可变引用 + 一个 bool 标志）。
#[derive(Clone, Copy)]
struct Ctx<'a> {
    ver: &'a UnityVersion,
    refs: &'a RefTypeTable,
    has_registry: bool,
}

/// 按 `root` 树解释 `data`，返回动态值树。
///
/// 走完后校验 `cursor.pos() == data.len()`，不匹配报 `TypeTreeSizeMismatch`
/// （版本不匹配的强信号）。
pub fn interpret(
    data: &[u8],
    root: &TypeNode,
    ver: &UnityVersion,
    endian: Endian,
    refs: &RefTypeTable,
) -> Result<Value> {
    let mut r = Cursor::new(data, endian);
    let ctx = Ctx {
        ver,
        refs,
        has_registry: false,
    };
    let v = walk(&mut r, root, ctx)?;
    if r.pos() != data.len() {
        return Err(TypeTreeSizeMismatchSnafu {
            expected: data.len(),
            actual: r.pos(),
        }
        .build());
    }
    Ok(v)
}

fn walk(r: &mut Cursor, node: &TypeNode, ctx: Ctx) -> Result<Value> {
    let mut align = node.meta_flag & ALIGN_MASK != 0;

    let v = match &node.kind {
        TypeKind::Bool => Value::Bool(r.read_bool()?),
        TypeKind::U8 => Value::U8(r.read_u8()?),
        TypeKind::I8 => Value::I8(r.read_i8()?),
        TypeKind::U16 => Value::U16(r.read_u16("u16")?),
        TypeKind::I16 => Value::I16(r.read_i16("i16")?),
        TypeKind::U32 => Value::U32(r.read_u32("u32")?),
        TypeKind::I32 => Value::I32(r.read_i32("i32")?),
        TypeKind::U64 => Value::U64(r.read_u64("u64")?),
        TypeKind::I64 => Value::I64(r.read_i64("i64")?),
        TypeKind::F32 => Value::F32(r.read_f32("f32")?),
        TypeKind::F64 => Value::F64(r.read_f64("f64")?),

        TypeKind::String => {
            let n = r.read_i32("string len")? as usize;
            let b = r.read_bytes(n, "string data")?;
            let s = String::from_utf8_lossy(b).into_owned();
            r.align(4);
            Value::String(s)
        }
        TypeKind::TypelessData => {
            let n = r.read_i32("typeless len")? as usize;
            Value::Bytes(r.read_bytes(n, "typeless data")?.to_vec())
        }
        TypeKind::Pair => {
            let first = walk(r, &node.children[0], ctx)?;
            let second = walk(r, &node.children[1], ctx)?;
            Value::Pair(Box::new(first), Box::new(second))
        }
        TypeKind::PPtr(_) => {
            let file_id = r.read_i32("pptr file_id")?;
            let path_id = if *ctx.ver >= UnityVersion::new(5, 0, 0) {
                r.read_i64("pptr path_id")?
            } else {
                r.read_i32("pptr path_id")? as i64
            };
            Value::PPtr(PPtrValue { file_id, path_id })
        }

        TypeKind::Struct(_) => {
            if node.is_sequence() {
                // vector / staticvector / set / map / Array：读 size + N 元素
                let array_wrapper = &node.children[0];
                if array_wrapper.meta_flag & ALIGN_MASK != 0 {
                    align = true;
                }
                let size = r.read_i32("array size")?;
                if size < 0 {
                    return Err(NegativeArrayLenSnafu { len: size }.build());
                }
                let elem = &array_wrapper.children[1];
                let mut out = Vec::with_capacity((size as usize).min(4096));
                for _ in 0..size {
                    out.push(walk(r, elem, ctx)?);
                }
                Value::Array(out)
            } else {
                read_struct(r, node, ctx)?
            }
        }
    };

    if align {
        r.align(4);
    }
    Ok(v)
}

/// 读 struct（或含 ReferencedObject / ManagedReferencesRegistry 的特例）。
fn read_struct(r: &mut Cursor, node: &TypeNode, ctx: Ctx) -> Result<Value> {
    let mut fields: Vec<(String, Value)> = Vec::with_capacity(node.children.len());
    let mut child_ctx = ctx;
    for child in &node.children {
        match &child.kind {
            // ManagedReferencesRegistry 同一对象只读一次
            TypeKind::Struct(s) if s == "ManagedReferencesRegistry" => {
                if child_ctx.has_registry {
                    continue;
                }
                child_ctx.has_registry = true;
            }
            // [SerializeReference]：用已读的 type{class,ns,asm} 兄弟字段查 ref_types
            TypeKind::Struct(s) if s == "ReferencedObjectData" => {
                let (class, ns, asm) = extract_ref_type(&fields)?;
                let ref_node = ctx.refs.lookup(&class, &ns, &asm).ok_or_else(|| {
                    RefTypeNotFoundSnafu {
                        class,
                        ns,
                        asm,
                    }
                    .build()
                })?;
                let val = walk(r, ref_node, ctx)?;
                fields.push((child.name.clone(), val));
                continue;
            }
            _ => {}
        }
        fields.push((child.name.clone(), walk(r, child, child_ctx)?));
    }
    Ok(Value::Struct(fields))
}

/// 从已累积的字段中取 "type" 子结构里的 class/ns/asm。
fn extract_ref_type(fields: &[(String, Value)]) -> Result<(String, String, String)> {
    let missing = || {
        RefTypeNotFoundSnafu {
            class: String::new(),
            ns: String::new(),
            asm: String::new(),
        }
        .build()
    };
    let type_val = fields
        .iter()
        .find(|(k, _)| k == "type")
        .map(|(_, v)| v)
        .ok_or_else(missing)?;
    let type_fields = match type_val {
        Value::Struct(f) => f,
        _ => return Err(missing()),
    };
    let find_str = |key: &str| -> String {
        type_fields
            .iter()
            .find(|(k, _)| k == key)
            .and_then(|(_, v)| {
                if let Value::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    };
    Ok((find_str("class"), find_str("ns"), find_str("asm")))
}
