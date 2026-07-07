//! Pass 管线：从 `TpkTypeTree` 到 `Pipeline`（含 `EmitClass` 集合）。
//!
//! 顺序：p01 归一化 → p02 抽 subclass → p03 收 PPtr 目标 → p04 解析继承
//!     → p05 字段并集 → p06 计算对齐。
//! emit 在 `emit.rs` 里。

use std::collections::{BTreeMap, BTreeSet};

use crate::ir::*;
use crate::names::{to_rust_field_name, to_rust_type_name};
use crate::tpk::{TpkTypeTree, TpkUnityNode, UnityVersion};

// ────────────────────────────────────────────────────────────────────
// 入口
// ────────────────────────────────────────────────────────────────────

/// 跑完整 pass 管线（p01-p06）。消耗 `TpkTypeTree`。
pub fn run_pipeline(blob: TpkTypeTree) -> Pipeline {
    // 收集所有 Unity 版本（按升序）
    let all_versions = {
        let mut v: Vec<UnityVersion> = blob.versions.clone();
        v.sort();
        v.dedup();
        v
    };

    let mut state = Pipeline {
        classes: BTreeMap::new(),
        pptr_targets: BTreeSet::new(),
        all_versions,
    };
    let mut blob = blob;

    p01_normalize_types(&mut blob);
    p02_extract_subclasses(&blob, &mut state);
    p03_collect_pptr_targets(&blob, &mut state);
    p04_resolve_inheritance(&blob, &mut state);
    p05_merge_fields(&blob, &mut state);
    p06_plan_alignment(&mut state);

    state
}

// ────────────────────────────────────────────────────────────────────
// p01：类型归一化（`int → SInt32` 等）
// ────────────────────────────────────────────────────────────────────

fn p01_normalize_types(blob: &mut TpkTypeTree) {
    log::info!("p01: 归一化类型名");
    for s in blob.string_pool.iter_mut() {
        *s = normalize_one(s);
    }
}

/// 参考AssetRipper `UniversalNode.cs:131-143` 的 `GetFixedTypeName`。
fn normalize_one(s: &str) -> String {
    match s {
        "short" => "SInt16".into(),
        "ushort" => "UInt16".into(),
        "int" => "SInt32".into(),
        "uint" => "UInt32".into(),
        "long long" => "SInt64".into(),
        "unsigned short" => "UInt16".into(),
        "unsigned int" => "UInt32".into(),
        "unsigned long long" => "UInt64".into(),
        "unsigned char" => "UInt8".into(),
        "signed char" => "SInt8".into(),
        "Utf8String" => "string".into(),
        _ => s.into(),
    }
}

// ────────────────────────────────────────────────────────────────────
// p02：抽取 subclass（节点树里所有 NodeType::Type 的 type_name 集合）
// ────────────────────────────────────────────────────────────────────

fn p02_extract_subclasses(blob: &TpkTypeTree, state: &mut Pipeline) {
    log::info!("p02: 抽取 subclass");

    // 遍历所有节点，收集 NodeType::Type 的 type_name
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for node in &blob.node_pool {
        let type_name = blob
            .string_pool
            .get(node.type_name_index as usize)
            .cloned()
            .unwrap_or_default();
        let nt = NodeType::classify(&type_name, &node.sub_nodes);
        if nt == NodeType::Type {
            seen.insert(type_name);
        }
    }

    for unity_name in seen {
        let rust_name = to_rust_type_name(&unity_name);
        let class = EmitClass {
            key: ClassKey::Subclass(unity_name.clone()),
            name: rust_name,
            unity_name: unity_name.clone(),
            own_fields: Vec::new(),
            is_abstract: false,
            is_mono_behaviour: false,
            version_layouts: Vec::new(),
        };
        state.classes.insert(ClassKey::Subclass(unity_name), class);
    }
}

// ────────────────────────────────────────────────────────────────────
// p03：收集所有 PPtr 目标 class（供 emit 引用）
// ────────────────────────────────────────────────────────────────────

fn p03_collect_pptr_targets(blob: &TpkTypeTree, state: &mut Pipeline) {
    log::info!("p03: 收集 PPtr 目标");
    for node in &blob.node_pool {
        let type_name = blob
            .string_pool
            .get(node.type_name_index as usize)
            .map(String::as_str)
            .unwrap_or("");
        if let Some(target) = parse_pptr_target(type_name) {
            state.pptr_targets.insert(target);
        }
    }
}

/// `"PPtr<Component>" → Some("Component")`；`"PPtr<$GameObject>" → Some("GameObject")`。
fn parse_pptr_target(type_name: &str) -> Option<String> {
    let inner = type_name.strip_prefix("PPtr<").and_then(|s| s.strip_suffix('>'))?;
    let inner = inner.strip_prefix('$').unwrap_or(inner);
    Some(to_rust_type_name(inner))
}

// ────────────────────────────────────────────────────────────────────
// p04：创建 main class 骨架（名字 + abstract/mono_behaviour 标志）
// ────────────────────────────────────────────────────────────────────

fn p04_resolve_inheritance(blob: &TpkTypeTree, state: &mut Pipeline) {
    log::info!("p04: 创建 main class 骨架");

    // 第一遍：创建所有 EmitClass
    for (class_id, info) in &blob.classes {
        // 第一个 class 的 name/abstract 标志
        let first_with_class = info.classes.iter().find_map(|e| e.class.as_ref());
        let (name, is_abstract) = if let Some(c) = first_with_class {
            let n = blob
                .string_pool
                .get(c.name_index as usize)
                .cloned()
                .unwrap_or_else(|| format!("Class{class_id}"));
            (n, c.flags.is_abstract())
        } else {
            (format!("Class{class_id}"), false)
        };

        let rust_name = to_rust_type_name(&name);
        let is_mono_behaviour = *class_id == 114 || name == "MonoBehaviour";

        let class = EmitClass {
            key: ClassKey::Main(*class_id),
            name: rust_name,
            unity_name: name,
            own_fields: Vec::new(),
            is_abstract,
            is_mono_behaviour,
            version_layouts: Vec::new(),
        };
        state.classes.insert(ClassKey::Main(*class_id), class);
    }

    // 第二遍：检测重名（不同 classID 同名，Unity 历史 ID 复用），重名的加 _ID 后缀
    let mut name_count: BTreeMap<String, usize> = BTreeMap::new();
    for class in state.classes.values() {
        if class.is_main() {
            *name_count.entry(class.name.clone()).or_default() += 1;
        }
    }
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for class in state.classes.values_mut() {
        if !class.is_main() {
            continue;
        }
        if name_count.get(&class.name).copied().unwrap_or(0) > 1 {
            let n = seen.entry(class.name.clone()).or_insert(0);
            if *n > 0 && let ClassKey::Main(id) = class.key {
                class.name = format!("{}_{}", class.name, id);
            }
            *n += 1;
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// p05：字段并集 + 版本门控推断（核心）
// ────────────────────────────────────────────────────────────────────

fn p05_merge_fields(blob: &TpkTypeTree, state: &mut Pipeline) {
    log::info!("p05: 字段并集 + 版本门控");

    // 1. 对每个 main class 计算字段（扁平，含继承链；TPK 已把继承链平铺到 release_root）
    let mut self_cache: BTreeMap<i32, Vec<EmitField>> = BTreeMap::new();
    let mut layouts_cache: BTreeMap<i32, Vec<VersionLayout>> = BTreeMap::new();
    let main_ids: Vec<i32> = blob.classes.keys().copied().collect();
    for class_id in &main_ids {
        compute_main_fields(
            *class_id,
            blob,
            &mut self_cache,
            &mut layouts_cache,
        );
    }

    // 写回 state
    for (class_id, own) in &self_cache {
        if let Some(class) = state.classes.get_mut(&ClassKey::Main(*class_id)) {
            class.own_fields = own.clone();
        }
    }
    for (class_id, layouts) in &layouts_cache {
        if let Some(class) = state.classes.get_mut(&ClassKey::Main(*class_id)) {
            // 只在版本间顺序不同时才存 layouts；顺序一致则留空（用 own_fields 即可）
            class.version_layouts = dedup_layouts(layouts);
        }
    }

    // 2. 处理 subclass（无继承，直接取第一次出现的子树）
    let sub_keys: Vec<ClassKey> = state
        .classes
        .keys()
        .filter(|k| matches!(k, ClassKey::Subclass(_)))
        .cloned()
        .collect();
    for key in &sub_keys {
        if let ClassKey::Subclass(unity_name) = key {
            let first_idx = blob.node_pool.iter().enumerate().find(|(_, n)| {
                let tn = blob
                    .string_pool
                    .get(n.type_name_index as usize)
                    .map(String::as_str)
                    .unwrap_or("");
                tn == unity_name.as_str()
            });
            if let Some((idx, _)) = first_idx {
                let fields = flatten_root(idx, blob);
                let emit_fields: Vec<EmitField> = fields
                    .into_iter()
                    .map(|f| EmitField {
                        rust_name: f.rust_name,
                        unity_name: f.unity_name,
                        rust_type: f.rust_type,
                        version_gate: Some(VersionGate::Always),
                        align_after: false,
                        unity_meta_flag: f.meta_flag,
                    })
                    .collect();
                if let Some(class) = state.classes.get_mut(key) {
                    class.own_fields = emit_fields;
                }
            }
        }
    }
}

/// 递归计算 main class 的扁平字段 + per-version layouts。
///
/// TPK 把继承链平铺到每个具体类的 release_root 里（例如 Transform 的 release_root
/// 含 `m_GameObject`，来自 Component 基类）。扁平设计下直接取 release_root 的全部
/// 子节点作为该类的字段，无需扣除祖先字段，也不生成父类。
///
/// 同时计算 per-version layouts：每个 TPK 版本变体的 release_root 有自己的字段顺序，
/// 存为 `VersionLayout` 供 emit 生成 version-gated parse 块。
fn compute_main_fields(
    class_id: i32,
    blob: &TpkTypeTree,
    self_cache: &mut BTreeMap<i32, Vec<EmitField>>,
    layouts_cache: &mut BTreeMap<i32, Vec<VersionLayout>>,
) {
    if self_cache.contains_key(&class_id) {
        return;
    }
    let self_fields = compute_main_self_fields(class_id, blob, self_cache);
    self_cache.insert(class_id, self_fields.clone());

    // Per-version layouts：每个版本变体按 release_root 真实顺序（扁平，含继承链字段）
    let info = match blob.classes.get(&class_id) {
        Some(i) => i,
        None => {
            layouts_cache.insert(class_id, Vec::new());
            return;
        }
    };
    let mut layouts: Vec<VersionLayout> = Vec::new();
    for entry in &info.classes {
        if let Some(cls) = entry.class.as_ref()
            && let Some(root_idx) = cls.release_root
        {
            let flat = flatten_root(root_idx as usize, blob);
            let fields: Vec<EmitField> = flat
                .into_iter()
                .map(|f| {
                    let align_after =
                        (f.meta_flag & 0x4000) != 0 || (f.meta_flag & 0x8000) != 0;
                    EmitField {
                        rust_name: f.rust_name,
                        unity_name: f.unity_name,
                        rust_type: f.rust_type,
                        version_gate: Some(VersionGate::Always),
                        align_after,
                        unity_meta_flag: f.meta_flag,
                    }
                })
                .collect();
            layouts.push(VersionLayout {
                since: (entry.version.major, entry.version.minor, entry.version.build),
                fields,
            });
        }
    }
    layouts_cache.insert(class_id, layouts);
}

/// 判定是否需要 version-gated parse 块，需要时返回按版本去重后的 layouts。
///
/// **只有当某两个版本变体的公共字段相对顺序不一致（真重排）时才需要 version-gated**；
/// 字段仅增减（公共字段相对顺序不变）时，线性 parse + per-field `Option<T>` 门控即可正确处理，
/// 无需冗长的 `let mut x = None; if/else` 块。
fn dedup_layouts(layouts: &[VersionLayout]) -> Vec<VersionLayout> {
    if layouts.len() <= 1 {
        return Vec::new(); // 0 或 1 个 = 无需分块
    }

    // 所有版本变体的公共字段相对顺序都一致 → 线性 parse 足够
    let orders: Vec<Vec<&str>> = layouts
        .iter()
        .map(|l| l.fields.iter().map(|f| f.rust_name.as_str()).collect())
        .collect();
    let mut any_reorder = false;
    'outer: for i in 0..orders.len() {
        for j in (i + 1)..orders.len() {
            if !common_order_consistent(&orders[i], &orders[j]) {
                any_reorder = true;
                break 'outer;
            }
        }
    }
    if !any_reorder {
        return Vec::new();
    }

    // 有真重排：合并连续相同顺序的 layouts，减少分支数
    let mut merged: Vec<VersionLayout> = Vec::new();
    for layout in layouts {
        let order: Vec<&str> = layout.fields.iter().map(|f| f.rust_name.as_str()).collect();
        if let Some(last) = merged.last() {
            let last_order: Vec<&str> = last.fields.iter().map(|f| f.rust_name.as_str()).collect();
            if last_order == order {
                continue; // 顺序相同，合并
            }
        }
        merged.push(layout.clone());
    }
    if merged.len() <= 1 {
        Vec::new()
    } else {
        merged
    }
}

/// 两组字段名的公共元素在两边的相对顺序是否一致。
/// 一致 → 可用线性 parse（公共字段顺序不变，差异字段用 Option 门控）。
/// 不一致（存在真重排）→ 必须 version-gated。
fn common_order_consistent(a: &[&str], b: &[&str]) -> bool {
    let a_common: Vec<&str> = a.iter().copied().filter(|x| b.contains(x)).collect();
    let b_common: Vec<&str> = b.iter().copied().filter(|x| a.contains(x)).collect();
    a_common == b_common
}

/// class 的 self_fields（含祖先字段，因为 TPK 把继承链平铺到 release_root）。
fn compute_main_self_fields(
    class_id: i32,
    blob: &TpkTypeTree,
    cache: &mut BTreeMap<i32, Vec<EmitField>>,
) -> Vec<EmitField> {
    if let Some(v) = cache.get(&class_id) {
        return v.clone();
    }
    let info = match blob.classes.get(&class_id) {
        Some(i) => i,
        None => return Vec::new(),
    };

    let mut version_fields: Vec<(UnityVersion, Vec<FlatField>)> = Vec::new();
    for entry in &info.classes {
        if let Some(cls) = entry.class.as_ref()
            && let Some(root_idx) = cls.release_root
        {
            let fields = flatten_root(root_idx as usize, blob);
            version_fields.push((entry.version, fields));
        }
    }
    let self_fields = merge_fields_across_versions(&version_fields);
    cache.insert(class_id, self_fields.clone());
    self_fields
}

/// 展平字段（root 的直接子节点 = 字段列表）。
struct FlatField {
    unity_name: String,
    rust_name: String,
    rust_type: RustType,
    meta_flag: u32,
}

fn flatten_root(root_idx: usize, blob: &TpkTypeTree) -> Vec<FlatField> {
    let root = &blob.node_pool[root_idx];
    root.sub_nodes
        .iter()
        .map(|&child_idx| {
            let node = &blob.node_pool[child_idx as usize];
            let unity_name = blob
                .string_pool
                .get(node.name_index as usize)
                .cloned()
                .unwrap_or_default();
            let rust_name = to_rust_field_name(&unity_name);
            let rust_type = node_to_rust_type(node, blob);
            FlatField {
                unity_name,
                rust_name,
                rust_type,
                meta_flag: node.meta_flag,
            }
        })
        .collect()
}

/// 递归地把 TpkUnityNode 转成 RustType。
fn node_to_rust_type(node: &TpkUnityNode, blob: &TpkTypeTree) -> RustType {
    let type_name = blob
        .string_pool
        .get(node.type_name_index as usize)
        .map(String::as_str)
        .unwrap_or("");
    let nt = NodeType::classify(type_name, &node.sub_nodes);

    match nt {
        NodeType::Boolean => RustType::Bool,
        NodeType::UInt8 => RustType::U8,
        NodeType::Int8 => RustType::I8,
        NodeType::UInt16 => RustType::U16,
        NodeType::Int16 => RustType::I16,
        NodeType::UInt32 => RustType::U32,
        NodeType::Int32 => RustType::I32,
        NodeType::UInt64 => RustType::U64,
        NodeType::Int64 => RustType::I64,
        NodeType::Single => RustType::F32,
        NodeType::Double => RustType::F64,
        NodeType::String => RustType::String,
        NodeType::TypelessData => RustType::Bytes,
        NodeType::PPtr => {
            let target = parse_pptr_target(type_name).unwrap_or_else(|| "Unknown".into());
            RustType::Pptr { target_class: target }
        }
        NodeType::Array | NodeType::Vector => {
            let data_idx = find_array_data_child(node, blob, nt);
            if let Some(idx) = data_idx {
                let elem = node_to_rust_type(&blob.node_pool[idx as usize], blob);
                RustType::List(Box::new(elem))
            } else {
                RustType::List(Box::new(RustType::Bytes))
            }
        }
        NodeType::Map => {
            // map → Array → pair (data)
            let pair_idx = find_pair_in_map(node, blob);
            if let Some(idx) = pair_idx {
                let pair_node = &blob.node_pool[idx as usize];
                if pair_node.sub_nodes.len() >= 2 {
                    let k = node_to_rust_type(&blob.node_pool[pair_node.sub_nodes[0] as usize], blob);
                    let v = node_to_rust_type(&blob.node_pool[pair_node.sub_nodes[1] as usize], blob);
                    let pair_t = RustType::Pair(Box::new(k), Box::new(v));
                    return RustType::List(Box::new(pair_t));
                }
            }
            RustType::List(Box::new(RustType::Pair(
                Box::new(RustType::Bytes),
                Box::new(RustType::Bytes),
            )))
        }
        NodeType::Pair => {
            if node.sub_nodes.len() >= 2 {
                let k = node_to_rust_type(&blob.node_pool[node.sub_nodes[0] as usize], blob);
                let v = node_to_rust_type(&blob.node_pool[node.sub_nodes[1] as usize], blob);
                RustType::Pair(Box::new(k), Box::new(v))
            } else {
                RustType::Pair(Box::new(RustType::Bytes), Box::new(RustType::Bytes))
            }
        }
        NodeType::Type => {
            let rust_name = to_rust_type_name(type_name);
            RustType::Subclass(rust_name)
        }
    }
}

/// Array / vector 节点里找 `data` 子节点的索引（即元素类型节点）。
fn find_array_data_child(node: &TpkUnityNode, blob: &TpkTypeTree, nt: NodeType) -> Option<u16> {
    // vector: sub_nodes = [Array] → Array.sub_nodes = [size, data]
    // Array 直接: sub_nodes = [size, data]
    let array_node = if nt == NodeType::Vector {
        let arr_idx = node.sub_nodes.first()?;
        &blob.node_pool[*arr_idx as usize]
    } else {
        node
    };
    // 找名为 "data" 的子节点
    for &child_idx in &array_node.sub_nodes {
        let child = &blob.node_pool[child_idx as usize];
        let name = blob
            .string_pool
            .get(child.name_index as usize)
            .map(String::as_str)
            .unwrap_or("");
        if name == "data" {
            return Some(child_idx);
        }
    }
    // 兜底：第二个子节点（size 之后）
    array_node.sub_nodes.get(1).copied()
}

/// map 节点里找 `pair` 子节点的索引。
fn find_pair_in_map(node: &TpkUnityNode, blob: &TpkTypeTree) -> Option<u16> {
    // map → Array → pair (data)
    let array_idx = node.sub_nodes.first()?;
    let array_node = &blob.node_pool[*array_idx as usize];
    for &child_idx in &array_node.sub_nodes {
        let child = &blob.node_pool[child_idx as usize];
        let tn = blob
            .string_pool
            .get(child.type_name_index as usize)
            .map(String::as_str)
            .unwrap_or("");
        if tn == "pair" {
            return Some(child_idx);
        }
    }
    array_node.sub_nodes.get(1).copied()
}

/// 把多个版本变体的字段列表合并成一份并集，每字段带 VersionGate。
fn merge_fields_across_versions(
    versions: &[(UnityVersion, Vec<FlatField>)],
) -> Vec<EmitField> {
    if versions.is_empty() {
        return Vec::new();
    }

    // 收集所有出现过的字段（按 rust_name 去重，因为 unity_name 不同可能映射到同一 rust_name）
    let mut all_names_index: BTreeMap<String, usize> = BTreeMap::new();
    let mut all_names_ordered: Vec<String> = Vec::new();
    for (_, fields) in versions {
        for f in fields {
            if !all_names_index.contains_key(&f.rust_name) {
                all_names_index.insert(f.rust_name.clone(), all_names_ordered.len());
                all_names_ordered.push(f.rust_name.clone());
            }
        }
    }

    let total = versions.len();
    let mut result: Vec<EmitField> = Vec::with_capacity(all_names_ordered.len());

    for rust_name in &all_names_ordered {
        // 该字段在哪些版本里出现？（按 rust_name 匹配）
        let present_versions: Vec<UnityVersion> = versions
            .iter()
            .filter_map(|(v, fields)| {
                if fields.iter().any(|f| &f.rust_name == rust_name) {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();

        // 取**最新版本**的字段定义作为类型基准（而非最老的）。
        // 原因：某些字段跨版本会改类型（如 GameObject.m_Component 从 pair<SInt32,PPtr>
        // 变为 ComponentPair{PPtr}）。取最新版本的类型，因为我们主要面向现代 Unity 版本。
        let field_def = versions
            .iter()
            .rev() // 最新版本优先
            .flat_map(|(_, fields)| fields.iter())
            .find(|f| &f.rust_name == rust_name)
            .expect("字段应在某版本出现");

        let gate = if present_versions.len() == total {
            VersionGate::Always
        } else if let Some(earliest) = present_versions.iter().min() {
            VersionGate::Since(earliest.major, earliest.minor, earliest.build)
        } else {
            VersionGate::Always
        };

        let rust_type = if matches!(gate, VersionGate::Always) {
            field_def.rust_type.clone()
        } else {
            RustType::Option(Box::new(field_def.rust_type.clone()))
        };

        result.push(EmitField {
            rust_name: field_def.rust_name.clone(),
            unity_name: field_def.unity_name.clone(),
            rust_type,
            version_gate: Some(gate),
            align_after: false,
            unity_meta_flag: field_def.meta_flag,
        });
    }

    // 按首次出现位置排序（保持字段在字节流里的顺序）
    result.sort_by_key(|f| {
        versions
            .iter()
            .filter_map(|(_, fields)| fields.iter().position(|x| x.rust_name == f.rust_name))
            .min()
            .unwrap_or(usize::MAX)
    });

    result
}

// ────────────────────────────────────────────────────────────────────
// p06：计算对齐（meta_flag & (0x4000 | 0x8000) → align_after）
// ────────────────────────────────────────────────────────────────────

fn p06_plan_alignment(state: &mut Pipeline) {
    log::info!("p06: 计算对齐");
    for class in state.classes.values_mut() {
        for field in &mut class.own_fields {
            // AlignBytes (0x4000) 或 AnyChildUsesAlignBytes (0x8000) 任一置位 → 该字段后对齐。
            // 参考 AssetRipper SerializablePrimitive.cs:66。
            field.align_after = (field.unity_meta_flag & 0x4000) != 0
                || (field.unity_meta_flag & 0x8000) != 0;
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 单元测试
// ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pptr_simple() {
        assert_eq!(parse_pptr_target("PPtr<Component>"), Some("Component".into()));
        assert_eq!(parse_pptr_target("PPtr<Texture2D>"), Some("Texture2D".into()));
    }

    #[test]
    fn parse_pptr_dollar_prefix() {
        // MonoBehaviour 字段会写成 PPtr<$GameObject>
        assert_eq!(parse_pptr_target("PPtr<$GameObject>"), Some("GameObject".into()));
    }

    #[test]
    fn parse_pptr_not_pptr() {
        assert_eq!(parse_pptr_target("int"), None);
        assert_eq!(parse_pptr_target("vector"), None);
    }
}
