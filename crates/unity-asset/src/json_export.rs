//! 任意对象 JSON dump（export feature）：`EnvironmentObject` → `Value`。
//!
//! 对齐 AssetRipper PrimaryContent 的 `JsonContentExtractor` 语义——JSON 是
//! 「无专用导出器对象」的默认兜底格式。三条解析路径统一汇入 `Value`：
//!
//! - `Typed`：codegen 发射的 `impl ToValue`（键 = 原始 PascalCase 字段名）；
//! - `Dynamic`：TypeTree 解释器产物，本身就是 `Value`；
//! - `MonoBehaviour`：shell 五字段 + 用户 structure **扁平合并**——贴近 Unity
//!   真实序列化布局（AssetRipper 的 `m_Structure` 嵌套是其 helper 注入产物，
//!   刻意不模仿）。
//!
//! 产出 `Value` 而非 JSON 文本：库零 serde，文本化（转义/浮点格式化/base64）
//! 由 CLI/HTTP 层的 serde_json 完成。

use crate::Value;
use crate::environment::EnvironmentObject;
use crate::object_parser::MonoBehaviour;
use crate::types::UnityObject;
use unity_core::ToValue;

/// 把任意已解析对象 dump 成 `Value` 树（转换纯函数，无失败路径）。
pub fn dump_object(obj: &EnvironmentObject) -> Value {
    match &obj.object {
        crate::ParsedObject::Typed(u) => dump_typed(u),
        crate::ParsedObject::Dynamic(v) => v.clone(),
        crate::ParsedObject::MonoBehaviour(mb) => dump_mono_behaviour(mb),
    }
}

fn dump_typed(u: &UnityObject) -> Value {
    u.to_value()
}

/// MonoBehaviour：shell 字段 + 用户字段扁平合并为一个 `Value::Struct`。
/// structure 不是 Struct（异常布局）时整体作为 `m_Structure` 键保留，不丢数据。
fn dump_mono_behaviour(mb: &MonoBehaviour) -> Value {
    let mut fields: Vec<(String, Value)> = vec![
        (
            "m_GameObject".to_owned(),
            Value::PPtr(mb.m_game_object.clone()),
        ),
        ("m_Enabled".to_owned(), Value::Bool(mb.m_enabled)),
        ("m_Script".to_owned(), Value::PPtr(mb.m_script.clone())),
        ("m_Name".to_owned(), Value::String(mb.m_name.clone())),
    ];
    if let Some(v) = &mb.m_editor_class_identifier {
        fields.push((
            "m_EditorClassIdentifier".to_owned(),
            Value::String(v.clone()),
        ));
    }
    match mb.structure.clone() {
        Value::Struct(user_fields) => fields.extend(user_fields),
        other => fields.push(("m_Structure".to_owned(), other)),
    }
    Value::Struct(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mono_shell(structure: Value) -> MonoBehaviour {
        MonoBehaviour {
            m_game_object: crate::typetree::PPtrValue {
                file_id: 0,
                path_id: 7,
            },
            m_enabled: true,
            m_script: crate::typetree::PPtrValue {
                file_id: 1,
                path_id: 9,
            },
            m_name: "TestMB".to_owned(),
            m_editor_class_identifier: None,
            structure,
        }
    }

    #[test]
    fn mono_behaviour_flattens_shell_and_structure() {
        let mb = mono_shell(Value::Struct(vec![
            ("m_SomeField".to_owned(), Value::I32(42)),
            ("m_Nested".to_owned(), Value::F32(0.5)),
        ]));
        let v = dump_mono_behaviour(&mb);
        let Value::Struct(fields) = v else {
            panic!("顶层应为 Struct");
        };
        let keys: Vec<&str> = fields.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(
            keys,
            vec![
                "m_GameObject",
                "m_Enabled",
                "m_Script",
                "m_Name",
                "m_SomeField",
                "m_Nested",
            ]
        );
        assert!(matches!(&fields[0].1, Value::PPtr(p) if p.path_id == 7));
        assert!(matches!(&fields[1].1, Value::Bool(true)));
    }

    #[test]
    fn mono_behaviour_non_struct_structure_kept_under_m_structure() {
        let mb = mono_shell(Value::I32(3));
        let v = dump_mono_behaviour(&mb);
        let Value::Struct(fields) = v else {
            panic!("顶层应为 Struct");
        };
        let last = fields.last().expect("非空");
        assert_eq!(last.0, "m_Structure");
        assert!(matches!(last.1, Value::I32(3)));
    }
}
