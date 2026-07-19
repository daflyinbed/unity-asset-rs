//! 对象 DTO + 序列化转换层。
//!
//! 库 `unity-asset` 刻意零 serde(AGENTS.md「公共 API 精简,勿随意扩 pub」),故
//! HTTP 层自己定义 DTO。分层:
//! - [`ObjectMeta`]:默认元数据(path_id / class / 解析方式 / 名),列表和详情共用。
//! - [`ObjectDetail`]:`?detail=full` 时附带的原始 dump——typed 走 `{:#?}` Debug,
//!   dynamic/MonoBehaviour structure 走 [`value_to_json`] 递归 JSON。
//!
//! 复用 main.rs 的 [`crate::class_name`] / [`crate::obj_display_name`] / [`crate::parse_status`],
//! 不重复实现这些展示逻辑。

use generated::UnityObject;
use serde::Serialize;
use serde_json::{json, Map, Value as JsonValue};
use unity_asset::{ContainerEntry, ContainerKind, EnvironmentObject, MonoBehaviour, ParsedObject, Value};
use utoipa::ToSchema;

/// bytes/string 在 JSON 里截断到此长度(与 CLI `dump_value` 思路一致,防超大字段刷屏)。
const TRUNCATE_LEN: usize = 256;

/// 对象元数据。列表项和详情头部共用。
#[derive(Serialize, ToSchema)]
pub struct ObjectMeta {
    pub path_id: i64,
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
            path_id,
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

/// `?detail=full` 的原始 dump。typed/dynamic/mono 三路不同表示。
#[derive(Serialize, ToSchema)]
pub struct ObjectDetail {
    /// "typed" / "dynamic" / "mono"。
    pub kind: &'static str,
    /// typed 路径:`{:#?}` Debug 字符串(codegen 大枚举,手写 DTO 成本过高,统一 Debug)。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<String>,
    /// dynamic/MonoBehaviour structure 路径:递归 JSON。
    ///
    /// **Schema 限制**:动态 Value 树结构不固定(每个 MonoBehaviour 用户字段都不同),
    /// 这里用 `serde_json::Value`(自由 object)而非具体 schema。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// MonoBehaviour 的 typed shell 字段(仅 mono 路径)。
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
    pub path_id: i64,
}

impl From<&unity_asset::PPtrValue> for PPtrJson {
    fn from(p: &unity_asset::PPtrValue) -> Self {
        Self {
            file_id: p.file_id,
            path_id: p.path_id,
        }
    }
}

impl ObjectDetail {
    /// 由 [`ParsedObject`] 构造 detail。typed → debug;dynamic → value;mono → shell+value。
    pub fn from_parsed(obj: &ParsedObject) -> Self {
        match obj {
            ParsedObject::Typed(uo) => Self {
                kind: "typed",
                debug: Some(format!("{uo:#?}")),
                value: None,
                mono_shell: None,
            },
            ParsedObject::Dynamic(v) => Self {
                kind: "dynamic",
                debug: None,
                value: Some(value_to_json(v)),
                mono_shell: None,
            },
            ParsedObject::MonoBehaviour(mb) => Self {
                kind: "mono",
                debug: None,
                value: Some(value_to_json(&mb.structure)),
                mono_shell: Some(mono_shell(mb)),
            },
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

/// [`Value`] 树 → `serde_json::Value`。
///
/// 13 变体递归 match。`Bytes`/`String` 超长截断(附 `_truncated: true` 标记)。
/// `Struct` → JSON object,`Array` → JSON array,`Pair` → 2-元素 array,
/// `PPtr` → `{file_id, path_id}`。其余标量直接映射。
pub fn value_to_json(v: &Value) -> JsonValue {
    match v {
        Value::Bool(b) => json!(b),
        Value::I8(x) => json!(x),
        Value::U8(x) => json!(x),
        Value::I16(x) => json!(x),
        Value::U16(x) => json!(x),
        Value::I32(x) => json!(x),
        Value::U32(x) => json!(x),
        Value::I64(x) => json!(x),
        Value::U64(x) => json!(x),
        Value::F32(x) => json!(x),
        Value::F64(x) => json!(x),
        Value::String(s) => JsonValue::String(truncate_string(s)),
        Value::Bytes(b) => truncate_bytes(b),
        Value::Pair(a, bb) => json!([value_to_json(a), value_to_json(bb)]),
        Value::Array(items) => {
            JsonValue::Array(items.iter().map(value_to_json).collect())
        }
        Value::Struct(fields) => {
            let mut m = Map::with_capacity(fields.len());
            for (k, val) in fields {
                m.insert(k.clone(), value_to_json(val));
            }
            JsonValue::Object(m)
        }
        Value::PPtr(p) => json!({ "file_id": p.file_id, "path_id": p.path_id }),
    }
}

/// String 截断到 [`TRUNCATE_LEN`] 字符;超出直接截断(不加省略号,JSON 消费方自行处理)。
fn truncate_string(s: &str) -> String {
    if s.chars().count() <= TRUNCATE_LEN {
        return s.to_string();
    }
    let mut t: String = s.chars().take(TRUNCATE_LEN).collect();
    t.push('…');
    t
}

/// Bytes 转 hex 字符串(截断)。前缀 `0x`,附长度信息避免歧义。
fn truncate_bytes(b: &[u8]) -> JsonValue {
    let truncated = b.len() > TRUNCATE_LEN;
    let head: String = if truncated {
        b.iter().take(TRUNCATE_LEN).map(|x| format!("{x:02x}")).collect()
    } else {
        b.iter().map(|x| format!("{x:02x}")).collect()
    };
    json!({
        "_bytes_hex": head,
        "len": b.len(),
        "_truncated": truncated,
    })
}

/// `UnityObject` 是否是可导出 PNG 的类(Texture2D / Sprite)。
pub fn is_image_class(uo: &UnityObject) -> bool {
    matches!(uo, UnityObject::Texture2D(_) | UnityObject::Sprite(_))
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
    pub path_id: i64,
}

impl ContainerEntryDto {
    /// 由 [`ContainerEntry`] 构造。`env_collections_len` 仅用于校验 collection 下标合法
    /// (跨 bundle 失效引用理论不会越界,但防御性跳过)。
    pub fn from_entry(e: &ContainerEntry, env_collections_len: usize) -> Self {
        let ref_dto = |id: unity_asset::ObjectId| {
            (id.collection.to_index() < env_collections_len).then_some(ObjectRefDto {
                collection: id.collection.to_index(),
                path_id: id.path_id,
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
                path_id: e.host.path_id,
            }),
            preload: e.preload.as_ref().map(|slice| {
                slice.iter().map(|p| p.and_then(ref_dto)).collect()
            }),
        }
    }
}
