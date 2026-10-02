//! 动态值树与 `ToValue` 转换层。
//!
//! `Value` 是 typetree 解释器的解码产物（MonoBehaviour / Dynamic 回退），也是
//! JSON dump 等导出的统一中间表示——typed struct 经 codegen 发射的
//! `impl ToValue` 汇入同一棵树。放在 unity-core 是依赖方向决定的：
//! `unity-core ← generated`，generated 的发射代码必须能引用它。
//!
//! `ToValue` 采用 serde 式架构（trait + 容器 blanket impl），但目标类型是
//! 本 crate 的 `Value` 而非 serde 的数据模型——库保持零 serde。

/// 动态值树（MonoBehaviour / 所有 class 的解码产物，及导出 dump 的中间表示）。
///
/// 派生 `PartialEq` 便于测试与 dump 对比（含 f32/f64，不派生 `Eq`）。
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Pair(Box<Value>, Box<Value>),
    Array(Vec<Value>),
    Struct(Vec<(String, Value)>),
    PPtr(PPtrValue),
}

/// 动态值树里的跨引用指针（`Value::PPtr` 载荷）。
#[derive(Debug, Clone, PartialEq)]
pub struct PPtrValue {
    pub file_id: i32,
    pub path_id: i64,
}

/// serde 式转换 trait：typed struct → `Value`。
///
/// codegen 为全部生成类型发射 `impl ToValue`（见 `crates/codegen/src/emit.rs`），
/// 容器（Vec/Option/Box/元组）与标量在此处 blanket 实现。
pub trait ToValue {
    fn to_value(&self) -> Value;

    /// 按字段路径检查值。数组仅在显式指定 limit 时返回该页元素；嵌套数组
    /// 只返回长度，避免检查视图先构建全量值树。完整导出仍走 to_value。
    fn inspect_value(&self, path: &[String], _offset: usize, _limit: usize) -> Option<Value> {
        path.is_empty().then(|| self.to_value())
    }
}

macro_rules! impl_to_value_scalar {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl ToValue for $ty {
                fn to_value(&self) -> Value {
                    Value::$variant(*self)
                }
            }
        )*
    };
}

impl_to_value_scalar!(
    bool => Bool,
    i8 => I8,
    u8 => U8,
    i16 => I16,
    u16 => U16,
    i32 => I32,
    u32 => U32,
    i64 => I64,
    u64 => U64,
    f32 => F32,
    f64 => F64,
);

impl ToValue for String {
    fn to_value(&self) -> Value {
        Value::String(self.clone())
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        self.as_str().inspect_value(path, offset, limit)
    }
}

impl ToValue for str {
    fn to_value(&self) -> Value {
        Value::String(self.to_owned())
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        if !path.is_empty() {
            return None;
        }
        let len = self.chars().count();
        let limit = if limit == 0 { 256 } else { limit };
        if offset > len {
            return None;
        }
        if offset == 0 && len <= limit {
            return Some(self.to_value());
        }
        let end = offset.saturating_add(limit).min(len);
        Some(inspection_page(
            "string",
            len,
            offset,
            end,
            "text",
            Value::String(self.chars().skip(offset).take(end - offset).collect()),
        ))
    }
}

// 注意：不给 Vec<u8> 单独 impl（与下方 blanket 一致性冲突）。字节数组
// （TypelessData 字段）由 codegen 特判直接构造 Value::Bytes。
impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        Value::Array(self.iter().map(ToValue::to_value).collect())
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        inspect_array(self.len(), path, offset, limit, |i, path, offset, limit| {
            self[i].inspect_value(path, offset, limit)
        })
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        self.as_ref()?.inspect_value(path, offset, limit)
    }

    fn to_value(&self) -> Value {
        match self {
            Some(v) => v.to_value(),
            // 结构体字段级不调用本 impl（codegen 走 opt_field 跳过 None），
            // 这里仅为直接持有 Option 的调用方提供合理回退。
            None => Value::Struct(Vec::new()),
        }
    }
}

impl<T: ToValue> ToValue for Box<T> {
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        self.as_ref().inspect_value(path, offset, limit)
    }

    fn to_value(&self) -> Value {
        self.as_ref().to_value()
    }
}

impl<A: ToValue, B: ToValue> ToValue for (A, B) {
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        match path.split_first() {
            Some((i, rest)) if i == "0" => self.0.inspect_value(rest, offset, limit),
            Some((i, rest)) if i == "1" => self.1.inspect_value(rest, offset, limit),
            Some(_) => None,
            None => Some(Value::Array(vec![
                self.0.inspect_value(&[], 0, 0)?,
                self.1.inspect_value(&[], 0, 0)?,
            ])),
        }
    }

    fn to_value(&self) -> Value {
        Value::Pair(Box::new(self.0.to_value()), Box::new(self.1.to_value()))
    }
}

impl ToValue for crate::PPtr {
    fn to_value(&self) -> Value {
        Value::PPtr(PPtrValue {
            file_id: self.m_file_id,
            path_id: self.m_path_id,
        })
    }
}

impl ToValue for PPtrValue {
    fn to_value(&self) -> Value {
        self.clone().into()
    }
}

impl From<PPtrValue> for Value {
    fn from(v: PPtrValue) -> Value {
        Value::PPtr(v)
    }
}

/// 结构体字段级辅助：`Option` 字段为 None 时跳过（版本门控字段在该 Unity
/// 版本不序列化，dump 里不应出现）。
///
/// codegen 发射的 `impl ToValue` 里，门控字段一行是
/// `crate::opt_field("m_X", &self.m_x)`，非门控字段是
/// `Some(("m_X".to_string(), self.m_x.to_value()))`，最后
/// `.into_iter().flatten().collect()`。
pub fn opt_field<T: ToValue>(name: &str, v: &Option<T>) -> Option<(String, Value)> {
    v.as_ref().map(|v| (name.to_owned(), v.to_value()))
}

/// 生成代码与动态值共用的数组检查器。路径中的数组下标采用原始绝对下标。
#[doc(hidden)]
pub fn inspect_array(
    len: usize,
    path: &[String],
    offset: usize,
    limit: usize,
    mut item: impl FnMut(usize, &[String], usize, usize) -> Option<Value>,
) -> Option<Value> {
    if let Some((index, rest)) = path.split_first() {
        let index = index.parse::<usize>().ok()?;
        return (index < len)
            .then(|| item(index, rest, offset, limit))
            .flatten();
    }
    if offset > len {
        return None;
    }
    let end = offset.saturating_add(limit).min(len);
    let items = (offset..end)
        .map(|i| item(i, &[], 0, 0))
        .collect::<Option<Vec<_>>>()?;
    Some(inspection_page(
        "array",
        len,
        offset,
        end,
        "items",
        Value::Array(items),
    ))
}

/// TypelessData 的检查视图：最多复制请求的字节页，不走 Vec<u8> 的数组语义。
#[doc(hidden)]
pub fn inspect_bytes(bytes: &[u8], path: &[String], offset: usize, limit: usize) -> Option<Value> {
    if !path.is_empty() || offset > bytes.len() {
        return None;
    }
    let limit = if limit == 0 { 256 } else { limit };
    let end = offset.saturating_add(limit).min(bytes.len());
    let hex = bytes[offset..end]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    Some(inspection_page(
        "bytes",
        bytes.len(),
        offset,
        end,
        "_bytes_hex",
        Value::String(hex),
    ))
}

fn inspection_page(
    kind: &str,
    len: usize,
    offset: usize,
    end: usize,
    key: &str,
    value: Value,
) -> Value {
    let mut fields = vec![
        ("_kind".into(), Value::String(kind.into())),
        ("len".into(), Value::U64(len as u64)),
        ("offset".into(), Value::U64(offset as u64)),
        ("_truncated".into(), Value::Bool(offset > 0 || end < len)),
        (key.into(), value),
    ];
    if end < len {
        fields.push(("next_offset".into(), Value::U64(end as u64)));
    }
    Value::Struct(fields)
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<Value> {
        match self {
            Value::Struct(fields) => {
                if let Some((name, rest)) = path.split_first() {
                    return fields
                        .iter()
                        .find(|(k, _)| k == name)?
                        .1
                        .inspect_value(rest, offset, limit);
                }
                Some(Value::Struct(
                    fields
                        .iter()
                        .map(|(k, v)| Some((k.clone(), v.inspect_value(&[], 0, 0)?)))
                        .collect::<Option<_>>()?,
                ))
            }
            Value::Array(items) => items.inspect_value(path, offset, limit),
            Value::Bytes(bytes) => inspect_bytes(bytes, path, offset, limit),
            Value::String(text) => text.inspect_value(path, offset, limit),
            Value::Pair(a, b) => match path.split_first() {
                Some((i, rest)) if i == "0" => a.inspect_value(rest, offset, limit),
                Some((i, rest)) if i == "1" => b.inspect_value(rest, offset, limit),
                Some(_) => None,
                None => Some(Value::Array(vec![
                    a.inspect_value(&[], 0, 0)?,
                    b.inspect_value(&[], 0, 0)?,
                ])),
            },
            _ => path.is_empty().then(|| self.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field<'a>(value: &'a Value, name: &str) -> &'a Value {
        let Value::Struct(fields) = value else {
            panic!("应为摘要")
        };
        &fields.iter().find(|(k, _)| k == name).unwrap().1
    }

    #[test]
    fn inspection_only_visits_requested_array_elements() {
        struct Item;
        impl ToValue for Item {
            fn to_value(&self) -> Value {
                panic!("未请求的元素不能转换")
            }
        }
        let items = vec![Item, Item];
        let summary = items.inspect_value(&[], 0, 0).unwrap();
        assert_eq!(field(&summary, "len"), &Value::U64(2));
        assert_eq!(field(&summary, "items"), &Value::Array(vec![]));
        assert_eq!(field(&summary, "next_offset"), &Value::U64(0));
    }

    #[test]
    fn pages_preserve_absolute_paths_and_bound_nested_arrays() {
        let values = Value::Struct(vec![(
            "a/b~c".into(),
            Value::Array(vec![
                Value::Array(vec![Value::I32(11)]),
                Value::Array(vec![Value::I32(22)]),
            ]),
        )]);
        let path = vec!["a/b~c".into()];
        let page = values.inspect_value(&path, 1, 256).unwrap();
        let Value::Array(items) = field(&page, "items") else {
            panic!()
        };
        assert_eq!(items.len(), 1);
        assert_eq!(field(&items[0], "items"), &Value::Array(vec![]));
        assert_eq!(
            values.inspect_value(&["a/b~c".into(), "1".into(), "0".into()], 0, 256),
            Some(Value::I32(22))
        );
        assert!(values.inspect_value(&path, 3, 256).is_none());
        assert!(
            values
                .inspect_value(&["a/b~c".into(), "2".into()], 0, 256)
                .is_none()
        );
    }

    #[test]
    fn bytes_and_unicode_text_have_lossless_pages() {
        let bytes = (0..=255).cycle().take(600).collect::<Vec<u8>>();
        let page = inspect_bytes(&bytes, &[], 512, 256).unwrap();
        assert_eq!(
            field(&page, "_bytes_hex"),
            &Value::String(bytes[512..].iter().map(|b| format!("{b:02x}")).collect())
        );
        assert_eq!(field(&page, "offset"), &Value::U64(512));
        let text = "你好🦀".repeat(200);
        let page = text.inspect_value(&[], 255, 3).unwrap();
        assert_eq!(field(&page, "text"), &Value::String("你好🦀".into()));
        assert_eq!(field(&page, "next_offset"), &Value::U64(258));
    }
}
