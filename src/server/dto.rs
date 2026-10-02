//! 对象 DTO + 序列化转换层。
//!
//! 库 `unity-asset` 刻意零 serde(AGENTS.md「公共 API 精简,勿随意扩 pub」),故
//! HTTP 层自己定义 DTO。分层:
//! - [`ObjectMeta`]:默认元数据(path_id / class / 解析方式 / 名),列表和详情共用。
//! - [`ObjectDetail`]:有界检查视图，typed/dynamic/mono 均按字段访问；
//!   数组按页请求，字节和长字符串返回摘要。完整导出不截断。
//!
//! 复用 main.rs 的 [`crate::class_name`] / [`crate::obj_display_name`] / [`crate::parse_status`],
//! 不重复实现这些展示逻辑。

use generated::UnityObject;
use serde::Serialize;
use serde_json::{Map, Value as JsonValue, json};
use unity_asset::{
    ContainerEntry, ContainerKind, EnvironmentObject, MonoBehaviour, ParsedObject, Value,
};
use utoipa::ToSchema;

/// 对象元数据。列表项和详情头部共用。
#[derive(Serialize, ToSchema)]
pub struct ObjectMeta {
    /// 十进制字符串，避免 JavaScript JSON number 丢失 i64 精度。
    pub path_id: String,
    pub class_id: i32,
    pub class_name: String,
    /// "typed" / "mono" / "dynamic"。
    pub parse_status: &'static str,
    /// 对象展示名(取 m_Name;无则空串)。
    pub name: String,
}

impl ObjectMeta {
    pub fn from_obj(path_id: i64, obj: &EnvironmentObject) -> Self {
        let (_, label) = crate::parse_status(&obj.object);
        Self {
            path_id: path_id.to_string(),
            class_id: obj.class_id.get(),
            class_name: crate::class_name(obj.class_id),
            parse_status: label,
            name: crate::obj_display_name(&obj.object),
        }
    }
}

/// 完整对象响应:元数据 + 可选 detail。
#[derive(Serialize, ToSchema)]
pub struct ObjectDto {
    #[serde(flatten)]
    pub meta: ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ObjectDetail>,
}

/// 有界对象检查视图。数组按页读取；Bytes/长 String 最多预览 256 字节/字符。
#[derive(Serialize, ToSchema)]
pub struct ObjectDetail {
    /// "typed" / "dynamic" / "mono"。
    pub kind: &'static str,
    /// 递归 JSON；分页节点带 _kind/len/offset/next_offset 与 items、text 或 _bytes_hex。
    pub value: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mono_shell: Option<MonoShell>,
}

/// MonoBehaviour typed shell 字段(engine 基类:m_GameObject 等)。
#[derive(Serialize, ToSchema)]
pub struct MonoShell {
    pub m_game_object: PPtrJson,
    pub m_enabled: bool,
    pub m_script: PPtrJson,
    pub m_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_editor_class_identifier: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct PPtrJson {
    pub file_id: i32,
    /// 十进制字符串，避免 JavaScript JSON number 丢失 i64 精度。
    pub path_id: String,
}

impl From<&unity_asset::PPtrValue> for PPtrJson {
    fn from(p: &unity_asset::PPtrValue) -> Self {
        Self {
            file_id: p.file_id,
            path_id: p.path_id.to_string(),
        }
    }
}

impl ObjectDetail {
    pub fn from_parsed(obj: &ParsedObject) -> Self {
        let (kind, mono_shell) = match obj {
            ParsedObject::Typed(_) => ("typed", None),
            ParsedObject::Dynamic(_) => ("dynamic", None),
            ParsedObject::MonoBehaviour(mb) => ("mono", Some(mono_shell(mb))),
        };
        Self {
            kind,
            value: inspect_parsed(obj, &[], 0, 0).expect("根对象可检查"),
            mono_shell,
        }
    }
}

fn mono_shell(mb: &MonoBehaviour) -> MonoShell {
    MonoShell {
        m_game_object: PPtrJson::from(&mb.m_game_object),
        m_enabled: mb.m_enabled,
        m_script: PPtrJson::from(&mb.m_script),
        m_name: mb.m_name.clone(),
        m_editor_class_identifier: mb.m_editor_class_identifier.clone(),
    }
}

/// 路径针对 detail.value。typed 直接访问字段/数组页，禁止先转换整棵值树。
pub fn inspect_parsed(
    obj: &ParsedObject,
    path: &[String],
    offset: usize,
    limit: usize,
) -> Option<JsonValue> {
    use unity_asset::ToValue;
    let value = match obj {
        ParsedObject::Typed(uo) => uo.inspect_value(path, offset, limit),
        ParsedObject::Dynamic(v) => v.inspect_value(path, offset, limit),
        ParsedObject::MonoBehaviour(mb) => mb.structure.inspect_value(path, offset, limit),
    }?;
    Some(encode_json(&value, false))
}

/// 动态值的有界检查视图（与 typed inspect_value 同形状）。
#[cfg(test)]
pub fn value_to_json(v: &Value) -> JsonValue {
    use unity_asset::ToValue;
    encode_json(&v.inspect_value(&[], 0, 0).expect("根值可检查"), false)
}

/// 完整导出：Bytes 使用 base64；不截断数组/字符串，PPtr 用 Unity 字段名。
pub fn value_to_json_full(v: &Value) -> JsonValue {
    encode_json(v, true)
}

fn encode_json(v: &Value, full: bool) -> JsonValue {
    use base64::Engine as _;
    match v {
        Value::Bool(x) => json!(x),
        Value::I8(x) => json!(x),
        Value::U8(x) => json!(x),
        Value::I16(x) => json!(x),
        Value::U16(x) => json!(x),
        Value::I32(x) => json!(x),
        Value::U32(x) => json!(x),
        Value::I64(x) => json!(x),
        Value::U64(x) => json!(x),
        Value::F32(x) => float_to_json(*x as f64),
        Value::F64(x) => float_to_json(*x),
        Value::String(x) => json!(x),
        Value::Bytes(x) => json!(base64::engine::general_purpose::STANDARD.encode(x)),
        Value::Pair(a, b) => json!([encode_json(a, full), encode_json(b, full)]),
        Value::Array(items) => {
            JsonValue::Array(items.iter().map(|v| encode_json(v, full)).collect())
        }
        Value::Struct(fields) => {
            let mut map = Map::with_capacity(fields.len());
            for (k, v) in fields {
                map.insert(k.clone(), encode_json(v, full));
            }
            JsonValue::Object(map)
        }
        Value::PPtr(p) if full => json!({"m_FileID": p.file_id, "m_PathID": p.path_id}),
        Value::PPtr(p) => json!({"file_id": p.file_id, "path_id": p.path_id.to_string()}),
    }
}

/// 非有限浮点保留为字符串；f32 展宽后仍可逐位 roundtrip。
fn float_to_json(x: f64) -> JsonValue {
    if x.is_finite() {
        json!(x)
    } else if x.is_nan() {
        json!("NaN")
    } else if x > 0.0 {
        json!("Infinity")
    } else {
        json!("-Infinity")
    }
}

pub fn is_image_class(uo: &UnityObject) -> bool {
    matches!(uo, UnityObject::Texture2D(_) | UnityObject::Sprite(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    fn f64_of(j: &JsonValue) -> f64 {
        j.as_f64().expect("应为 JSON number")
    }

    /// 非有限浮点降级为字符串，两个转换器都不 panic / 不产出非法值。
    #[test]
    fn non_finite_floats_become_strings() {
        for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for conv in [
                value_to_json(&Value::F64(v)),
                value_to_json_full(&Value::F64(v)),
            ] {
                assert!(conv.is_string(), "非有限值应降级为字符串: {conv:?}");
            }
        }
        assert_eq!(value_to_json_full(&Value::F64(f64::NAN)), json!("NaN"));
        assert_eq!(
            value_to_json_full(&Value::F64(f64::INFINITY)),
            json!("Infinity")
        );
        assert_eq!(
            value_to_json_full(&Value::F64(f64::NEG_INFINITY)),
            json!("-Infinity")
        );
    }

    /// f32 经 `as f64` 进 JSON number，parse 回后逐位还原（诚实表示，非精度丢失）。
    #[test]
    fn f32_survives_json_roundtrip_bit_exact() {
        for x in [0.1f32, -3.7f32, 16777216.0f32, f32::MIN_POSITIVE] {
            let j = value_to_json_full(&Value::F32(x));
            let back = f64_of(&j) as f32;
            assert_eq!(back.to_bits(), x.to_bits(), "f32 roundtrip 应逐位还原: {x}");
        }
    }

    /// 全量版 Bytes → base64（不截断），与截断版（hex + 截断标记）并存。
    #[test]
    fn full_version_base64_encodes_bytes() {
        let v = Value::Bytes(b"hi".to_vec());
        let full = value_to_json_full(&v);
        assert_eq!(full, json!("aGk="));
        // 截断版仍是 hex 视图。
        let truncated = value_to_json(&v);
        assert_eq!(truncated["len"], json!(2));
        assert_eq!(truncated["_bytes_hex"], json!("6869"));
    }

    /// 全量版 PPtr 用 Unity 原名 + 数字（serde_json Number 精确持有 i64），
    /// 截断版 path_id 为字符串（防 JS 精度丢失——那是给浏览器的视图）。
    #[test]
    fn pptr_shape_differs_between_full_and_detail_view() {
        let p = Value::PPtr(unity_asset::PPtrValue {
            file_id: 3,
            path_id: -8764833589149381726i64,
        });
        let full = value_to_json_full(&p);
        assert_eq!(full["m_FileID"], json!(3));
        assert_eq!(full["m_PathID"], json!(-8764833589149381726i64));

        let detail = value_to_json(&p);
        assert_eq!(detail["file_id"], json!(3));
        assert_eq!(detail["path_id"], json!("-8764833589149381726"));
    }

    /// 全量版 String 不截断。
    #[test]
    fn full_version_does_not_truncate_strings() {
        let long = "x".repeat(1000);
        assert_eq!(
            value_to_json_full(&Value::String(long.clone())),
            JsonValue::String(long)
        );
    }
}

// ── container DTO ──

/// container 条目(AssetBundle.m_Container / ResourceManager.m_Container 聚合后的一条)。
///
/// 详见 `unity_asset::ContainerEntry`。path 可重复(multidict),每条都独立返回。
#[derive(Serialize, ToSchema)]
pub struct ContainerEntryDto {
    /// path 字符串(原文,未归一化)。
    /// AssetBundle:形如 `"assets/ui/mainmenu.prefab"`。
    /// ResourceManager:形如 `"Prefabs/MainMenu"`(无扩展名)。
    pub path: String,
    /// `"AssetBundle"`(服务于 `bundle.LoadAsset` / Addressables)
    /// 或 `"ResourceManager"`(服务于 `Resources.Load`)。
    pub kind: &'static str,
    /// 主对象引用(`{collection, path_id}`)。解引用失败(跨 bundle 失效引用)时为 `null`。
    pub asset: Option<ObjectRefDto>,
    /// 宿主对象引用(AssetBundle / ResourceManager 对象本身)。
    pub host: ObjectRefDto,
    /// AssetBundle entry 的 preload 切片预解析引用(`[preload_index, +preload_size)`)。
    /// ResourceManager 为 `null`。切片内解引用失败的位置为 `null`(保留下标对应关系)。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<Vec<Option<ObjectRefDto>>>,
}

/// 对象引用(collection 下标 + path_id)。寻址需带 collection,因 path_id 全局不唯一。
#[derive(Serialize, ToSchema)]
pub struct ObjectRefDto {
    /// collection 下标(见 `list_collections` 的 index)。
    pub collection: usize,
    /// 十进制字符串，避免 JavaScript JSON number 丢失 i64 精度。
    pub path_id: String,
}

impl ContainerEntryDto {
    /// 由 [`ContainerEntry`] 构造。`env_collections_len` 仅用于校验 collection 下标合法
    /// (跨 bundle 失效引用理论不会越界,但防御性跳过)。
    pub fn from_entry(e: &ContainerEntry, env_collections_len: usize) -> Self {
        let ref_dto = |id: unity_asset::ObjectId| {
            (id.collection.to_index() < env_collections_len).then_some(ObjectRefDto {
                collection: id.collection.to_index(),
                path_id: id.path_id.to_string(),
            })
        };
        Self {
            path: e.path.clone(),
            kind: match e.kind {
                ContainerKind::AssetBundle => "AssetBundle",
                ContainerKind::ResourceManager => "ResourceManager",
            },
            asset: e.asset.and_then(ref_dto),
            host: ref_dto(e.host).unwrap_or(ObjectRefDto {
                collection: e.host.collection.to_index(),
                path_id: e.host.path_id.to_string(),
            }),
            preload: e
                .preload
                .as_ref()
                .map(|slice| slice.iter().map(|p| p.and_then(ref_dto)).collect()),
        }
    }
}
