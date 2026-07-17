//! 对象解析 dispatch：非 MonoBehaviour → typed parse（失败回退到 TypeTree），
//! MonoBehaviour → TypeTree 解释器（shell typed + structure 动态）。
//!
//! 参见 `docs/object-layer.md` §1/§5/§8（dispatch + fallback + MonoBehaviour）。

use crate::cursor::Cursor;
use crate::error::{Error, MonoBehaviourShellSnafu, Result};
use crate::serialized_file::{ObjectInfo, SerializedFile};
use crate::typetree::{interpret, interpret_mono_behaviour, PPtrValue, RefTypeTable, Value};
use crate::Endian;
use generated::{ClassIDType, UnityObject};

/// 解析结果：typed struct、动态 Value、或 MonoBehaviour（typed shell + 动态 structure）。
#[derive(Debug)]
pub enum ParsedObject {
    /// codegen'd typed struct（Texture2D, Mesh, ...）
    Typed(UnityObject),
    /// TypeTree 解释器产出（typed parse 失败回退）
    Dynamic(Value),
    /// MonoBehaviour（classID=114）：typed shell + 动态 structure。
    /// 对齐 AssetRipper MonoBehaviour 壳子 + SerializableStructure。
    MonoBehaviour(MonoBehaviour),
}

/// MonoBehaviour 解析结果：typed shell（引擎基类字段）+ 动态 structure（用户脚本字段）。
///
/// 对齐 AssetRipper 的壳子/结构分离设计：`Read()` 只读 `m_GameObject`/`m_Enabled`/
/// `m_Script`/`m_Name` 等内置字段，reader 停在用户数据起始处，剩下全部交给 `structure`。
/// shell 字段经 `interpret_mono_behaviour` 分段后投影成 typed 字段；`structure` 保持
/// 动态 `Value`（用户字段不在 TPK 里，每次都不同），调用方按名字 pattern-match 访问。
///
/// TypeTree 路径下 MonoScript **不参与**布局推理——布局已在 TypeTree 里。
/// `m_script` 仅作为后续解引用到 MonoScript 资产的寻址入口。
#[derive(Debug)]
pub struct MonoBehaviour {
    /// 指向所属 GameObject 的 PPtr。
    pub m_game_object: PPtrValue,
    /// 组件是否启用。
    pub m_enabled: bool,
    /// 指向 MonoScript 资产的 PPtr（寻址入口，非布局来源）。
    pub m_script: PPtrValue,
    /// 组件名。
    pub m_name: String,
    /// 编辑器类标识符（部分版本有）。
    pub m_editor_class_identifier: Option<String>,
    /// 用户脚本字段（动态）。对齐 AssetRipper `SerializableStructure`。
    pub structure: Value,
}

/// 解析单个对象。
///
/// 由 `SerializedFile` 的调用方逐个调用。
pub fn parse_object(
    sf: &SerializedFile,
    obj: &ObjectInfo,
    refs: &RefTypeTable,
) -> Result<ParsedObject> {
    let data = obj.data(sf)?;
    let class_id = obj.class_id(sf);
    let ver = &sf.unity_version_parsed;
    let endian = sf.endian;

    // 获取该对象类型的 AB 内嵌 TypeTree（如果有）
    let ab_tree = sf.types.get(obj.type_idx).and_then(|t| t.node.as_ref());

    // ── MonoBehaviour (classID=114)：shell typed + structure 动态分段 ──
    // 对齐 AssetRipper：先读引擎基类字段（m_GameObject/m_Enabled/m_Script/m_Name），
    // 再读用户脚本字段。无内嵌 TypeTree 时硬错误（非 TypeTree 路径 = 未来工作）。
    if class_id == 114 {
        let tree = ab_tree.ok_or(Error::MonoBehaviourWithoutTypeTree)?;
        let (shell_fields, structure) = interpret_mono_behaviour(data, tree, ver, endian, refs)?;
        let mb = build_mono_behaviour(shell_fields, structure)?;
        return Ok(ParsedObject::MonoBehaviour(mb));
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

/// 从 shell 字段列表 + structure 构建 typed `MonoBehaviour`。
///
/// 按名字提取引擎基类字段：
/// - `m_GameObject`/`m_Script` → `Value::PPtr`（缺失或类型错 → 强错误，shell 必须存在）；
/// - `m_Enabled` → `Value::Bool`（兼容旧版本的 `Value::U8`）→ bool（缺失 → 默认 false + warn）；
/// - `m_Name` → `Value::String`（缺失 → 空串）；
/// - `m_EditorClassIdentifier` → `Value::String`（缺失 → `None`）。
///
/// `structure`（用户脚本字段）原样存入，不经 typed 投影。
fn build_mono_behaviour(fields: Vec<(String, Value)>, structure: Value) -> Result<MonoBehaviour> {
    // 按名字查找字段值（shell 字段少，线性扫描即可）。
    let find = |key: &str| -> Option<&Value> {
        fields.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    };

    let m_game_object = take_pptr(find("m_GameObject"), "m_GameObject")?;
    let m_script = take_pptr(find("m_Script"), "m_Script")?;

    let m_enabled = match find("m_Enabled") {
        Some(Value::Bool(b)) => *b,
        Some(Value::U8(u)) => *u != 0,
        Some(v) => {
            return Err(MonoBehaviourShellSnafu {
                field: "m_Enabled".to_string(),
                reason: format!("expected Bool or U8, got {:?}", v),
            }
            .build())
        }
        None => {
            log::warn!("MonoBehaviour shell: m_Enabled missing, defaulting to false");
            false
        }
    };

    let m_name = match find("m_Name") {
        Some(Value::String(s)) => s.clone(),
        Some(v) => {
            return Err(MonoBehaviourShellSnafu {
                field: "m_Name".to_string(),
                reason: format!("expected String, got {:?}", v),
            }
            .build())
        }
        None => String::new(),
    };

    let m_editor_class_identifier = match find("m_EditorClassIdentifier") {
        Some(Value::String(s)) => Some(s.clone()),
        Some(v) => {
            return Err(MonoBehaviourShellSnafu {
                field: "m_EditorClassIdentifier".to_string(),
                reason: format!("expected String, got {:?}", v),
            }
            .build())
        }
        None => None,
    };

    Ok(MonoBehaviour {
        m_game_object,
        m_enabled,
        m_script,
        m_name,
        m_editor_class_identifier,
        structure,
    })
}

/// 从 `Value::PPtr` 提取 `PPtrValue`，缺失或类型错则报 `MonoBehaviourShell`。
fn take_pptr(v: Option<&Value>, field: &str) -> Result<PPtrValue> {
    match v {
        Some(Value::PPtr(p)) => Ok(p.clone()),
        Some(other) => Err(MonoBehaviourShellSnafu {
            field: field.to_string(),
            reason: format!("expected PPtr, got {:?}", other),
        }
        .build()),
        None => Err(MonoBehaviourShellSnafu {
            field: field.to_string(),
            reason: "missing".to_string(),
        }
        .build()),
    }
}
