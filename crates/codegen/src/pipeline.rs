//! Pass 管线：从 `TpkTypeTree` 到 `Pipeline`（含 `EmitClass` 集合）。
//!
//! 顺序：p00 过滤引擎类型 → p01 归一化 → p02 抽 subclass → p03 收 PPtr 目标
//!     → p04 解析继承 → p05 字段并集 → p06 计算对齐。
//! emit 在 `emit.rs` 里。

use std::collections::{BTreeMap, BTreeSet};

use heck::ToUpperCamelCase;

use crate::ir::*;
use crate::names::{to_rust_field_name, to_rust_type_name};
use crate::tpk::{TpkTypeTree, TpkUnityNode, UnityVersion};

// ────────────────────────────────────────────────────────────────────
// 入口
// ────────────────────────────────────────────────────────────────────

/// 跑完整 pass 管线（p00-p06）。消耗 `TpkTypeTree`。
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

    p00_filter_engine_types(&mut blob);
    p01_normalize_types(&mut blob);
    p02_extract_subclasses(&blob, &mut state);
    p03_collect_pptr_targets(&blob, &mut state);
    p04_resolve_inheritance(&blob, &mut state);
    p05_merge_fields(&blob, &mut state);
    p06_plan_alignment(&mut state);

    state
}

// ────────────────────────────────────────────────────────────────────
// p00：过滤引擎内部类型（classID 100000-100011）
// ────────────────────────────────────────────────────────────────────

/// 过滤 Unity 引擎内部类型。对齐 AssetRipper `Pass000_ProcessTpk.cs:85` 的
/// `IsUnacceptable(int typeId) => typeId is >= 100000 and <= 100011`。
///
/// 这些 classID（`int`=100000、`bool`=100001、`float`=100002、`MonoObject`、
/// `Collision`、`Vector3f`=100005、`RootMotionData`、`Collision2D`、
/// `AudioMixerLiveUpdateFloat/Bool`、`Polygon2D`=100010、`void`=100011）在 Unity
/// 类型系统里注册了 classID 但**从不作为独立对象序列化**。TPK 里它们的
/// `abstract=false` 但 `release_root=None` —— 字段定义只作为内嵌 Type 节点出现在
/// 其他类的字段里。若不过滤，会生成空 struct + 空 parse，引用它们的类
/// （如 `Transform` 用 `Vector3f`）会读 0 字节导致后续字段错位。
fn p00_filter_engine_types(blob: &mut TpkTypeTree) {
    log::info!("p00: 过滤引擎内部类型 (classID 100000-100011)");
    let engine_type_ids: Vec<i32> = blob
        .classes
        .keys()
        .filter(|&&id| (100000..=100011).contains(&id))
        .copied()
        .collect();
    for id in engine_type_ids {
        log::debug!("p00: 移除 classID={id}");
        blob.classes.remove(&id);
    }
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

    // 收集所有非抽象 main class 的 Unity 名字。p00 已过滤 classID 100000-100011，
    // 所以这里不再需要 release_root 检查 —— 剩下的非抽象 main class 都有 release_root。
    //
    // 排除 main class 名字是因为：每个 main class 的 release_root 节点本身也是
    // NodeType::Type（type_name = 类名，且有子节点），若不排除会被误判成 subclass，
    // 导致同名文件在 classes/ 和 subclasses/ 各生成一份。
    //
    // 注意：这里不扫 release_root 字段树（AssetRipper Pass007 的做法），因为 p05 的
    // subclass 字段提取取「node_pool 首次出现」，可能拿到 editor 树节点，引用了 release
    // 树里没有的 subclass（如 PackingParameters 只出现在 editor 树的 SpriteAtlasEditorData
    // 里）。全面对齐 Pass007 需要同时重写 p05 的版本变体合并逻辑。
    let main_names: BTreeSet<String> = blob
        .classes
        .values()
        .flat_map(|info| info.classes.iter())
        .filter_map(|e| e.class.as_ref())
        .filter(|c| !c.flags.is_abstract())
        .filter_map(|c| blob.string_pool.get(c.name_index as usize).cloned())
        .collect();

    // 遍历所有节点，收集 NodeType::Type 的 type_name
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for node in &blob.node_pool {
        let type_name = blob
            .string_pool
            .get(node.type_name_index as usize)
            .cloned()
            .unwrap_or_default();
        if main_names.contains(&type_name) {
            continue;
        }
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
// p04：创建 main class 骨架（名字 + abstract 标志）
// ────────────────────────────────────────────────────────────────────

fn p04_resolve_inheritance(blob: &TpkTypeTree, state: &mut Pipeline) {
    log::info!("p04: 创建 main class 骨架");

    // 第一遍：创建所有 EmitClass
    // p00 已过滤掉 classID 100000-100011（引擎内部类型，无 release_root），
    // 这里无需再检查 release_root。
    for (class_id, info) in &blob.classes {
        // MonoBehaviour (classID=114) 不生成：用户脚本字段不在 TPK 里，
        // 运行时由主 crate 用 bundle 内嵌 TypeTree 解释器处理。
        if *class_id == 114 {
            log::debug!("p04: 跳过 MonoBehaviour (classID=114)");
            continue;
        }

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

        let class = EmitClass {
            key: ClassKey::Main(*class_id),
            name: rust_name,
            unity_name: name,
            own_fields: Vec::new(),
            is_abstract,
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

    // main class 名字集合（排除 main class Type 节点，避免被误当 subclass）
    let main_names: BTreeSet<String> = blob
        .classes
        .values()
        .flat_map(|info| info.classes.iter())
        .filter_map(|e| e.class.as_ref())
        .filter(|c| !c.flags.is_abstract())
        .filter_map(|c| blob.string_pool.get(c.name_index as usize).cloned())
        .collect();

    // 1. 对每个 main class 计算字段（扁平，含继承链；TPK 已把继承链平铺到 release_root）
    let mut main_cache: BTreeMap<i32, (Vec<EmitField>, Vec<VersionLayout>)> = BTreeMap::new();
    let main_ids: Vec<i32> = blob.classes.keys().copied().collect();
    for class_id in &main_ids {
        let class_name = state
            .classes
            .get(&ClassKey::Main(*class_id))
            .map(|c| c.name.as_str())
            .unwrap_or("Unknown");
        compute_main_fields(*class_id, class_name, blob, &mut main_cache);
    }

    // 写回 state
    for (class_id, (own, layouts)) in &main_cache {
        if let Some(class) = state.classes.get_mut(&ClassKey::Main(*class_id)) {
            class.own_fields = own.clone();
            class.version_layouts = layouts.clone();
        }
    }

    // 2. 收集每个 subclass 在所有 release_root 中的 (version, fields) 出现。
    //    与 main class 同逻辑做跨版本合并 —— 修前 subclass 取 node_pool 首次出现，
    //    无版本门控，同名 subclass 内部字段跨版本变化时会读错字节。
    let mut sub_occurrences: BTreeMap<String, Vec<(UnityVersion, Vec<FlatField>)>> =
        BTreeMap::new();
    for info in blob.classes.values() {
        for entry in &info.classes {
            if let Some(cls) = entry.class.as_ref()
                && let Some(root_idx) = cls.release_root
            {
                collect_subclass_from_node(
                    root_idx as usize,
                    entry.version,
                    blob,
                    &main_names,
                    &mut sub_occurrences,
                );
            }
        }
    }

    // 3. 对每个 subclass 跨版本合并字段
    let sub_keys: Vec<ClassKey> = state
        .classes
        .keys()
        .filter(|k| matches!(k, ClassKey::Subclass(_)))
        .cloned()
        .collect();
    for key in &sub_keys {
        let ClassKey::Subclass(unity_name) = key else {
            continue;
        };
        let class_name = state
            .classes
            .get(key)
            .map(|c| c.name.as_str())
            .unwrap_or("Unknown");

        if let Some(occurrences) = sub_occurrences.get(unity_name) {
            // 有 release_root 出现：跨版本合并
            let (own_fields, version_layouts) =
                merge_and_layout(occurrences.clone(), class_name);
            if let Some(class) = state.classes.get_mut(key) {
                class.own_fields = own_fields;
                class.version_layouts = version_layouts;
            }
        } else {
            // 回退：取 node_pool 首次出现（editor-only subclass 等，无 release 引用）
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
    class_name: &str,
    blob: &TpkTypeTree,
    cache: &mut BTreeMap<i32, (Vec<EmitField>, Vec<VersionLayout>)>,
) {
    if cache.contains_key(&class_id) {
        return;
    }

    let info = match blob.classes.get(&class_id) {
        Some(i) => i,
        None => {
            cache.insert(class_id, (Vec::new(), Vec::new()));
            return;
        }
    };

    let mut version_fields: Vec<(UnityVersion, Vec<FlatField>)> = Vec::new();
    for entry in &info.classes {
        if let Some(cls) = entry.class.as_ref()
            && let Some(root_idx) = cls.release_root
        {
            let fields = flatten_root(root_idx as usize, blob);
            // 去重：TPK 可能含同 (major, minor, build) 但不同 type 后缀的版本
            // （如 2021.1.0b1 和 2021.1.0f1）。取最后一个（更接近 final）。
            if version_fields
                .last()
                .is_some_and(|(v, _)| v.major_minor_build() == entry.version.major_minor_build())
            {
                *version_fields.last_mut().unwrap() = (entry.version, fields);
            } else {
                version_fields.push((entry.version, fields));
            }
        }
    }

    let result = merge_and_layout(version_fields, class_name);
    cache.insert(class_id, result);
}

/// 判定是否需要 version-gated parse 块，需要时返回按版本去重后的 layouts。
///
/// 线性 parse + per-field `Option<T>` 门控**只在以下三个条件都满足时才安全**：
/// 1. 所有版本变体的公共字段相对顺序一致（`common_order_consistent`）
/// 2. 合并后的字段顺序兼容每个版本的实际字节布局（`merged_order_compatible`）
/// 3. 每个字段的 presence 单调——一旦出现就不再消失（`presence_is_monotonic`）
///
/// 条件 2 是必要的因为：新字段可能插在现有字段之间（如 v5.3 在 `material` 前插入
/// `density`）。此时公共字段顺序不变（条件 1 过），但 `merge_fields_across_versions`
/// 按首次出现位置排序得到的合并顺序（`material` 先于 `density`）与新版本的实际字节
/// 顺序（`density` 先于 `material`）不兼容，线性 parse 会读到错误的字节。
///
/// 条件 3 是必要的因为：`merge_fields_across_versions` 对非全版本字段只产出
/// `Since(earliest)`，不产出 `Range`。若字段在早期版本存在、后期被移除（非单调），
/// `Since(earliest)` 会让线性 parse 在字段已不存在的版本里仍然尝试读取，导致后续
/// 字节全部错位。此时必须用 version-gated 块——每版本分支只读自己 layout 里的字段，
/// 字段消失的版本不会多读。
fn dedup_layouts(layouts: &[VersionLayout]) -> Vec<VersionLayout> {
    if layouts.len() <= 1 {
        return Vec::new(); // 0 或 1 个 = 无需分块
    }

    let orders: Vec<Vec<&str>> = layouts
        .iter()
        .map(|l| l.fields.iter().map(|f| f.rust_name.as_str()).collect())
        .collect();

    // 条件 1：公共字段相对顺序是否一致
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
        // 条件 2：合并顺序是否兼容每个版本的实际字段顺序
        let merged_order = compute_merged_field_order(&orders);
        // 条件 3：字段 presence 是否单调（一旦出现就不再消失）
        if orders
            .iter()
            .all(|ver_order| merged_order_compatible(&merged_order, ver_order))
            && presence_is_monotonic(&orders)
        {
            return Vec::new(); // 线性 parse 安全
        }
        // 合并顺序不兼容（新字段插在现有字段之间）或字段非单调消失 → 需要 version-gated
    }

    // 有真重排（或合并顺序不兼容）：合并连续相同顺序的 layouts，减少分支数
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

/// 计算合并后的字段顺序。逻辑同 `merge_fields_across_versions` 的排序：
/// 按"所有版本中首次出现的最小位置"稳定排序。
///
/// 这一步必须和 `merge_fields_across_versions` 产出相同的顺序，否则兼容性检查
/// 会验证错误的序列。
fn compute_merged_field_order<'a>(orders: &[Vec<&'a str>]) -> Vec<&'a str> {
    let mut all_names: Vec<&str> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for order in orders {
        for &name in order {
            if seen.insert(name) {
                all_names.push(name);
            }
        }
    }
    // 稳定排序：按"所有版本中的最小位置"
    all_names.sort_by_key(|&name| {
        orders
            .iter()
            .filter_map(|order| order.iter().position(|&n| n == name))
            .min()
            .unwrap_or(usize::MAX)
    });
    all_names
}

/// 检查合并顺序是否兼容某个版本的实际字段顺序。
///
/// 兼容 = 该版本的字段在合并顺序中的位置序列**严格递增**。
/// 不兼容（如新字段插在现有字段之间，导致合并顺序与版本实际顺序矛盾）→
/// 线性 parse 会读到错误的字节，必须用 version-gated 块。
fn merged_order_compatible(merged: &[&str], version_fields: &[&str]) -> bool {
    let merged_pos: BTreeMap<&str, usize> = merged
        .iter()
        .enumerate()
        .map(|(i, &name)| (name, i))
        .collect();

    let positions: Vec<usize> = version_fields
        .iter()
        .filter_map(|&name| merged_pos.get(&name).copied())
        .collect();

    // 位置序列必须严格递增
    positions.windows(2).all(|w| w[0] < w[1])
}

/// 检查字段 presence 是否单调：每个字段一旦出现就不再消失。
///
/// `orders` 按版本升序排列。对每个字段，遍历各版本的 presence，一旦出现（`seen=true`）
/// 就不应再消失。非单调（如字段在 v5.0 存在、v2017 被移除）时返回 `false`，
/// 触发 version-gated 块。
///
/// 这弥补了 `merge_fields_across_versions` 只产出 `Since(earliest)` 不产出 `Range`
/// 的限制：非单调字段在线性 parse 下会在不存在的版本里被错误读取。
fn presence_is_monotonic(orders: &[Vec<&str>]) -> bool {
    let all_fields: BTreeSet<&str> = orders
        .iter()
        .flat_map(|o| o.iter().copied())
        .collect();
    for name in &all_fields {
        let mut seen = false;
        for order in orders {
            if order.contains(name) {
                seen = true;
            } else if seen {
                return false; // 非单调：存在后消失
            }
        }
    }
    true
}

/// 展平字段（root 的直接子节点 = 字段列表）。
#[derive(Clone)]
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

/// 递归遍历节点子树，收集所有 `NodeType::Type` 节点（subclass）的出现。
///
/// 每次出现记录 `(version, 展平字段)`，version 来自父 main class 的版本。
/// 递归进入所有子节点（包括 vector/map/pair 容器），以发现嵌套 subclass。
/// 排除 `main_names` 中的 main class 名字（它们的 release_root 节点本身也是
/// `NodeType::Type`，不应被当作 subclass）。
fn collect_subclass_from_node(
    node_idx: usize,
    version: UnityVersion,
    blob: &TpkTypeTree,
    main_names: &BTreeSet<String>,
    out: &mut BTreeMap<String, Vec<(UnityVersion, Vec<FlatField>)>>,
) {
    let node = &blob.node_pool[node_idx];
    let type_name = blob
        .string_pool
        .get(node.type_name_index as usize)
        .map(String::as_str)
        .unwrap_or("");
    let nt = NodeType::classify(type_name, &node.sub_nodes);

    if nt == NodeType::Type && !main_names.contains(type_name) && !type_name.is_empty() {
        let fields = flatten_root(node_idx, blob);
        out.entry(type_name.to_string())
            .or_default()
            .push((version, fields));
    }

    // 递归子节点（所有类型，以发现嵌套 subclass）
    for &child_idx in &node.sub_nodes {
        collect_subclass_from_node(child_idx as usize, version, blob, main_names, out);
    }
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
///
/// 当同名字段在不同版本里类型不同时（如 `Mesh.m_shapes` 旧版为
/// `Vec<MeshBlendShape>`，新版为 `Vec<MeshBlendShapeVertex>`），
/// 不再取最新版本类型，而是生成 `VersionedEnum`。
fn merge_fields_across_versions(
    versions: &[(UnityVersion, Vec<FlatField>)],
    class_name: &str,
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
        // 收集该字段在所有版本里的 (version, rust_type, meta_flag)
        let present: Vec<(UnityVersion, RustType, u32)> = versions
            .iter()
            .filter_map(|(v, fields)| {
                fields
                    .iter()
                    .find(|f| &f.rust_name == rust_name)
                    .map(|f| (*v, f.rust_type.clone(), f.meta_flag))
            })
            .collect();

        let present_versions: Vec<UnityVersion> =
            present.iter().map(|(v, _, _)| *v).collect();

        let gate = if present_versions.len() == total {
            VersionGate::Always
        } else if let Some(earliest) = present_versions.iter().min() {
            VersionGate::Since(earliest.major, earliest.minor, earliest.build)
        } else {
            VersionGate::Always
        };

        // 检测该字段在所有版本里的 (类型, align_after) 是否一致。
        // 类型不变但 align_after 变了也要生成 VersionedEnum，
        // 否则老版本会按新版本的 align 读，多读/少读对齐字节。
        let first = &present[0];
        let first_align = meta_to_align(first.2);
        let all_same = present
            .iter()
            .all(|(_, t, m)| t == &first.1 && meta_to_align(*m) == first_align);

        let base_type = if all_same {
            first.1.clone()
        } else {
            build_versioned_enum(&present, rust_name, class_name)
        };

        let rust_type = if matches!(gate, VersionGate::Always) {
            base_type
        } else {
            RustType::Option(Box::new(base_type))
        };

        // 取最新版本的字段定义（unity_name / meta_flag）
        let latest_field = versions
            .iter()
            .rev()
            .flat_map(|(_, fields)| fields.iter())
            .find(|f| &f.rust_name == rust_name)
            .expect("字段应在某版本出现");

        result.push(EmitField {
            rust_name: latest_field.rust_name.clone(),
            unity_name: latest_field.unity_name.clone(),
            rust_type,
            version_gate: Some(gate),
            align_after: false,
            unity_meta_flag: latest_field.meta_flag,
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

/// 为类型或 align_after 跨版本变化的字段构建 `VersionedEnum`。
///
/// `present` 按版本升序排列。按 `(类型, align_after)` 分组：连续相同
/// (类型, align_after) 的版本归为一组。类型不变但 align_after 变了
/// 也拆成新变体——否则老版本会按新版本的 align 读，多读/少读对齐字节。
/// 第一组 → `VersionGate::Always`；后续组 → `VersionGate::Since(该组起始版本)`。
fn build_versioned_enum(
    present: &[(UnityVersion, RustType, u32)],
    field_name: &str,
    class_name: &str,
) -> RustType {
    // 按 (类型, align_after) 分组
    let mut groups: Vec<(UnityVersion, RustType, bool)> = Vec::new();
    for (ver, ty, meta) in present {
        let align = meta_to_align(*meta);
        if let Some(last) = groups.last()
            && last.1 == *ty
            && last.2 == align
        {
            continue; // 同 (类型, align)，合并
        }
        groups.push((*ver, ty.clone(), align));
    }

    let pascal = field_name.to_upper_camel_case(); // m_shapes → MShapes
    let enum_name = format!("{}{}Variant", class_name, pascal);

    let mut variants = Vec::new();
    for (i, (ver, ty, align)) in groups.iter().enumerate() {
        let vgate = if i == 0 {
            VersionGate::Always
        } else {
            VersionGate::Since(ver.major, ver.minor, ver.build)
        };
        // Subclass 在 enum 变体里按值存储，若存在递归引用（如 Clip → OffsetPtr →
        // OffsetPtrDataVariant → Clip）会导致无限大小。Box 打断循环。
        let boxed = if matches!(ty, RustType::Subclass(_)) {
            RustType::Box(Box::new(ty.clone()))
        } else {
            ty.clone()
        };
        variants.push(EnumVariant {
            gate: vgate,
            rust_type: boxed,
            align_after: *align,
        });
    }

    RustType::VersionedEnum {
        name: enum_name,
        variants,
    }
}

/// meta_flag → align_after：AlignBytes (0x4000) 或 AnyChildUsesAlignBytes (0x8000)。
fn meta_to_align(meta: u32) -> bool {
    (meta & 0x4000) != 0 || (meta & 0x8000) != 0
}

/// 从跨版本字段列表计算 merged `own_fields` + deduped `version_layouts`。
///
/// 供 main class 和 subclass 共用，确保两者用完全相同的合并逻辑：
/// 1. 排序 + 按 `major_minor_build` 去重（取最后一个，同 main class 逻辑）
/// 2. `merge_fields_across_versions` → merged own_fields（含 VersionGate / VersionedEnum）
/// 3. 每个 version 变体 → `VersionLayout`（按 release_root 真实顺序）
/// 4. `dedup_layouts` 判定线性 vs version-gated
fn merge_and_layout(
    version_fields: Vec<(UnityVersion, Vec<FlatField>)>,
    class_name: &str,
) -> (Vec<EmitField>, Vec<VersionLayout>) {
    // 排序 + 按 major_minor_build 去重（取最后一个）
    let mut sorted = version_fields;
    sorted.sort_by_key(|(v, _)| v.major_minor_build());
    let mut deduped: Vec<(UnityVersion, Vec<FlatField>)> = Vec::new();
    for (ver, fields) in sorted {
        if deduped
            .last()
            .is_some_and(|(v, _)| v.major_minor_build() == ver.major_minor_build())
        {
            *deduped.last_mut().unwrap() = (ver, fields);
        } else {
            deduped.push((ver, fields));
        }
    }

    let own_fields = merge_fields_across_versions(&deduped, class_name);

    let layouts: Vec<VersionLayout> = deduped
        .iter()
        .map(|(ver, fields)| VersionLayout {
            since: ver.major_minor_build(),
            fields: fields
                .iter()
                .map(|f| EmitField {
                    rust_name: f.rust_name.clone(),
                    unity_name: f.unity_name.clone(),
                    rust_type: f.rust_type.clone(),
                    version_gate: Some(VersionGate::Always),
                    align_after: meta_to_align(f.meta_flag),
                    unity_meta_flag: f.meta_flag,
                })
                .collect(),
        })
        .collect();
    let version_layouts = dedup_layouts(&layouts);
    (own_fields, version_layouts)
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
            field.align_after = meta_to_align(field.unity_meta_flag);
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

    #[test]
    fn merged_order_compatible_simple() {
        // 合并顺序与版本顺序完全一致
        let merged = ["a", "b", "c"];
        assert!(merged_order_compatible(&merged, &["a", "b", "c"]));
        // 版本少了字段（Option 门控），顺序仍兼容
        assert!(merged_order_compatible(&merged, &["a", "c"]));
        assert!(merged_order_compatible(&merged, &["b"]));
    }

    #[test]
    fn merged_order_incompatible_field_inserted() {
        // 核心测试：新字段插在现有字段之间
        // 版本 v5.3 实际顺序：[a, density, b]
        // merge 后合并顺序：[a, b, density]（b 首次出现在更早版本，排在 density 前面）
        let merged = ["a", "b", "density"];
        // v5.0 没有 density → 兼容
        assert!(merged_order_compatible(&merged, &["a", "b"]));
        // v5.3 有 density 在 a 和 b 之间 → 合并顺序 [a,b,density] 把 density 排到 b 后面
        // 版本顺序 [a,density,b] 在合并顺序中的位置序列 [0,2,1] → 不递增 → 不兼容
        assert!(!merged_order_compatible(&merged, &["a", "density", "b"]));
    }

    #[test]
    fn merged_order_compatible_new_field_at_end() {
        // 新字段加在末尾 → 兼容（合并在末尾读取，Option 门控）
        let merged = ["a", "b", "c"];
        assert!(merged_order_compatible(&merged, &["a", "b", "c"]));
        assert!(merged_order_compatible(&merged, &["a", "b"]));
    }

    #[test]
    fn presence_monotonic_all_present() {
        // 所有版本都有所有字段 → 单调
        assert!(presence_is_monotonic(&[
            vec!["a", "b", "c"],
            vec!["a", "b", "c"],
        ]));
    }

    #[test]
    fn presence_monotonic_field_added() {
        // 字段在后续版本新增 → 单调（Since 门控可正确处理）
        assert!(presence_is_monotonic(&[
            vec!["a", "b"],
            vec!["a", "b", "c"],
        ]));
        // 多版本渐进新增
        assert!(presence_is_monotonic(&[
            vec!["a"],
            vec!["a", "b"],
            vec!["a", "b", "c"],
        ]));
    }

    #[test]
    fn presence_non_monotonic_field_removed() {
        // 字段在后期版本被移除 → 非单调
        // v5.0: [a, b, c], v2017: [a, c]（b 被删除）
        assert!(!presence_is_monotonic(&[
            vec!["a", "b", "c"],
            vec!["a", "c"],
        ]));
    }

    #[test]
    fn presence_non_monotonic_field_removed_and_readded() {
        // 字段消失后重现 → 非单调
        assert!(!presence_is_monotonic(&[
            vec!["a", "b"],
            vec!["a"],
            vec!["a", "b"],
        ]));
    }

    #[test]
    fn presence_monotonic_single_version() {
        // 单版本 → trivially 单调
        assert!(presence_is_monotonic(&[vec!["a", "b"]]));
    }
}
