//! JSON dump 端到端测试：任意对象 → `Value`（typed 经 codegen ToValue，dynamic/
//! mono 经 TypeTree 解释器），并与 typed 字段交叉验证。
//!
//! 使用 `example/DownloadCommonUI.unity3d`（含 Material/Shader/GameObject/
//! Transform/MonoBehaviour 等无专用导出器的类）。受 `export` feature 门控。

#![cfg(feature = "export")]

use std::path::Path;

use generated::{ClassIDType, UnityObject};
use unity_asset::Value;
use unity_asset::{Environment, Loader, ParsedObject, dump_object};

/// 标准 Unity bundle fixture（仓库根 example/；本 crate 回溯两级）。
const UI_BUNDLE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../example/DownloadCommonUI.unity3d"
);

/// 加载 Environment（fixture 不存在时跳过，对齐 environment.rs 测试约定）。
fn load_env(path: &str) -> Environment {
    if !Path::new(path).exists() {
        eprintln!("跳过：{path} 不存在");
        std::process::exit(0);
    }
    Loader::builder()
        .path(path)
        .build()
        .unwrap_or_else(|e| panic!("加载 {path} 失败: {e}"))
        .into_environment()
}

/// 从 `Value::Struct` 里按键取值。
fn field<'a>(v: &'a Value, key: &str) -> &'a Value {
    let Value::Struct(fields) = v else {
        panic!("顶层应为 Struct，实得 {v:?}");
    };
    fields
        .iter()
        .find(|(k, _)| k.as_str() == key)
        .map(|(_, v)| v)
        .unwrap_or_else(|| panic!("缺字段 {key}，现有键: {fields:?}"))
}

#[test]
fn dump_all_objects_without_panic() {
    let env = load_env(UI_BUNDLE);
    let objects = env.objects();
    assert!(!objects.is_empty(), "应至少有一个对象");
    for obj in objects {
        // 三条解析路径（typed/dynamic/mono）都必须能 dump。
        let v = dump_object(obj);
        assert!(
            matches!(v, Value::Struct(_)),
            "顶层应为 Struct: pid={} class={}",
            obj.path_id,
            obj.class_id
        );
    }
}

#[test]
fn typed_dump_cross_checks_against_struct_fields() {
    let env = load_env(UI_BUNDLE);
    // Texture2D 虽有专用导出器，dump 仍可用——字段最丰富（标量/String/Option/
    // 嵌套 subclass/Bytes），做 to_value 映射的交叉验证对象。
    let obj = env
        .objects_of_class(ClassIDType::Texture2D)
        .find_map(|o| match &o.object {
            ParsedObject::Typed(UnityObject::Texture2D(t)) => Some((o, t)),
            _ => None,
        })
        .expect("应有一个 typed Texture2D");
    let (obj, t) = obj;
    let v = dump_object(obj);

    assert_eq!(field(&v, "m_Width"), &Value::I32(t.m_width));
    assert_eq!(field(&v, "m_Height"), &Value::I32(t.m_height));
    let Value::String(name) = field(&v, "m_Name") else {
        panic!("m_Name 应为 String");
    };
    // typed 的 m_name 是版本变体枚举（Default/V4_2/V5_0），内层 String 应一致。
    let typed_name: &str = match &t.m_name {
        generated::classes::texture2_d::Texture2DMNameVariant::Default(s) => s,
        generated::classes::texture2_d::Texture2DMNameVariant::V4_2(s) => s,
        generated::classes::texture2_d::Texture2DMNameVariant::V5_0(s) => s,
    };
    assert_eq!(name, typed_name);

    // 嵌套 subclass：m_TextureSettings（GlTextureSettings）应为 Struct。
    assert!(
        matches!(field(&v, "m_TextureSettings"), Value::Struct(_)),
        "m_TextureSettings 应为 Struct"
    );
}

#[test]
fn typed_dump_skips_version_gated_none_fields() {
    let env = load_env(UI_BUNDLE);
    // 此 bundle 为 5.x：m_ForcedFallbackFormat（2019.3 引入的 Option 字段）应缺席。
    let obj = env
        .objects_of_class(ClassIDType::Texture2D)
        .find(|o| {
            matches!(&o.object,
                ParsedObject::Typed(UnityObject::Texture2D(t)) if t.m_forced_fallback_format.is_none())
        })
        .expect("应有一个无 m_ForcedFallbackFormat 的 Texture2D（旧版本）");
    let Value::Struct(fields) = dump_object(obj) else {
        panic!("顶层应为 Struct");
    };
    assert!(
        fields.iter().all(|(k, _)| k != "m_ForcedFallbackFormat"),
        "None 的版本门控字段不应出现在 dump 里: {fields:?}"
    );
}

#[test]
fn non_dedicated_classes_dump_with_m_name() {
    let env = load_env(UI_BUNDLE);
    // GameObject：无专用导出器的 typed 类，dump 应带 m_Name/m_IsActive/m_Components。
    let obj = env
        .objects_of_class(ClassIDType::GameObject)
        .find(|o| matches!(&o.object, ParsedObject::Typed(UnityObject::GameObject(_))))
        .expect("应有一个 typed GameObject");
    let v = dump_object(obj);
    assert!(matches!(field(&v, "m_Name"), Value::String(_)));
    assert!(matches!(field(&v, "m_IsActive"), Value::Bool(_)));
    // m_Components 是 Vec<ComponentPair>，dump 应为数组且元素含 PPtr。
    match field(&v, "m_Components") {
        Value::Array(items) => {
            assert!(!items.is_empty(), "GameObject 应有组件");
            assert!(
                items.iter().all(|it| matches!(
                    it,
                    Value::Struct(fs) if fs.iter().any(|(k, v)| k == "component" && matches!(v, Value::PPtr(_)))
                )),
                "组件元素应含 component PPtr: {items:?}"
            );
        }
        other => panic!("m_Components 应为 Array，实得 {other:?}"),
    }
}

#[test]
fn optional_typeless_data_retains_bytes_and_skips_none() {
    use generated::ToValue;
    use generated::classes::property_modifications_target_test_object::PropertyModificationsTargetTestObject;
    use generated::subclasses::property_modifications_target_test_native_object::PropertyModificationsTargetTestNativeObject;
    let mut object = PropertyModificationsTargetTestObject {
        m_data: PropertyModificationsTargetTestNativeObject {
            m_integer_value: 0,
            m_float_value: 0.0,
        },
        m_array: vec![],
        m_float_test_value: 0.0,
        m_bytes: Some(vec![104, 105]),
        m_floats: None,
        m_bytes_size: Some(2),
        byte_data: Some(vec![104, 105]),
    };
    let value = object.to_value();
    assert_eq!(field(&value, "byte data"), &Value::Bytes(vec![104, 105]));
    assert_eq!(
        field(&value, "m_Bytes"),
        &Value::Array(vec![Value::U8(104), Value::U8(105)])
    );
    let preview = object.inspect_value(&[], 0, 0).unwrap();
    assert_eq!(
        field(field(&preview, "byte data"), "_bytes_hex"),
        &Value::String("6869".into())
    );
    object.byte_data = None;
    let Value::Struct(fields) = object.to_value() else {
        panic!()
    };
    assert!(fields.iter().all(|(k, _)| k != "byte data"));
    assert!(
        object
            .inspect_value(&["byte data".into()], 0, 256)
            .is_none()
    );
}
