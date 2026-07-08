//! Pass002：节点重命名 / 拆分（移植自 AssetRipper `Pass002_RenameSubnodes.cs`）。
//!
//! TPK 里若干 type_name 在不同上下文下布局完全不同（如 `ColorRGBA` 既可能是
//! 4×float(16字节) 也可能是 UInt32(4字节)）。若不拆分，p05 会把它们当同一
//! subclass 合并，按版本切换 enum——但实际区别是**父类上下文**而非版本，
//! 导致 typed parse 读错字节（见 Material 的 ColorRGBA bug）。
//!
//! 本 pass 在 p01 之后、p02 之前对 `TpkTypeTree` 原地重命名，把同名异构类型
//! 拆成独立 subclass（`ColorRGBA` → `ColorRGBAf` / `ColorRGBA32` 等），让 p05
//! 各自独立处理。只做**结构性拆分**和**类型/类名规范化**；纯字段重命名（如
//! `m_Dst` → `m_Destination`）不影响字节布局，本 pass 不做。
//!
//! # 遍历顺序
//!
//! 与 AssetRipper 一致：自下而上（先递归子节点再处理本节点），因为部分规则
//! 需要读子节点重命名后的类型名（如 `Keyframe` 读 `m_Value` 的类型，
//! `AnimationCurve` 读内层 keyframe 的类型）。
//!
//! # 字符串池
//!
//! 直接修改 `node.type_name_index` 指向新插入的字符串。原字符串保留不动
//! （可能被别的节点引用）。节点池每棵子树独立（143 个 ColorRGBA 出现 = 143
//! 个独立节点索引），就地修改安全。

use crate::tpk::TpkTypeTree;

/// 对应 AssetRipper `Pass002.DoPass`：遍历所有 class 的 release/editor root，
/// 自下而上重命名。
pub fn run(blob: &mut TpkTypeTree) {
    log::info!("rename: Pass002 节点重命名 / 拆分");

    // 1. ClassNameReplacement：改 class 名字（影响 main class 名 + root 节点 type_name）。
    //    先收集需要改的（最终 name / base 字符串 + root 索引），再统一应用，
    //    避免对 blob 的双重借用。
    struct ClassFix {
        class_id: i32,
        entry: usize,
        final_name: String,
        final_base: Option<String>,
        release_root: Option<u16>,
        editor_root: Option<u16>,
    }
    let mut fixes: Vec<ClassFix> = Vec::new();

    for (&class_id, info) in &blob.classes {
        for (ei, entry) in info.classes.iter().enumerate() {
            let Some(cls) = entry.class.as_ref() else { continue };
            let cur_name = name_str(blob, cls.name_index).to_string();
            let new_name = class_name_replacement(&cur_name)
                .map(|s| s.to_string())
                .unwrap_or(cur_name.clone());
            let cur_base = cls
                .base_index
                .map(|b| name_str(blob, b).to_string());
            let final_base = cur_base.and_then(|b| {
                class_name_replacement(&b)
                    .map(|s| s.to_string())
                    .or(Some(b))
            });
            if new_name != cur_name
                || final_base.is_some()
                || cls
                    .release_root
                    .or(cls.editor_root)
                    .is_some_and(|r| type_str(blob, r) != new_name)
            {
                fixes.push(ClassFix {
                    class_id,
                    entry: ei,
                    final_name: new_name,
                    final_base,
                    release_root: cls.release_root,
                    editor_root: cls.editor_root,
                });
            }
        }
    }
    // 统一应用：先 intern 字符串，再改 name_index / base_index / root type_name
    for fix in &fixes {
        let name_idx = intern(blob, &fix.final_name);
        let base_idx = fix.final_base.as_ref().map(|b| intern(blob, b));
        let info = blob.classes.get_mut(&fix.class_id).unwrap();
        let cls = info.classes[fix.entry].class.as_mut().unwrap();
        cls.name_index = name_idx;
        cls.base_index = base_idx;
        if let Some(r) = fix.release_root {
            blob.node_pool[r as usize].type_name_index = name_idx;
        }
        if let Some(r) = fix.editor_root {
            blob.node_pool[r as usize].type_name_index = name_idx;
        }
    }

    // 2. 收集所有需要遍历的 root 索引（release + editor）
    let mut roots: Vec<u16> = Vec::new();
    for info in blob.classes.values() {
        for entry in &info.classes {
            if let Some(cls) = entry.class.as_ref() {
                if let Some(r) = cls.release_root {
                    roots.push(r);
                }
                if let Some(r) = cls.editor_root {
                    roots.push(r);
                }
            }
        }
    }

    // 3. 自下而上重命名。注意：节点池可能共享（TPK 实测每个 ColorRGBA 出现
    //    都是独立节点），但保险起见用 visited 集合避免重复处理 + 避免环。
    let mut visited = std::collections::HashSet::new();
    for root in roots {
        rename_subtree(blob, root, &mut visited);
    }
}

// ────────────────────────────────────────────────────────────────────
// 核心遍历
// ────────────────────────────────────────────────────────────────────

fn rename_subtree(blob: &mut TpkTypeTree, idx: u16, visited: &mut std::collections::HashSet<u16>) {
    if !visited.insert(idx) {
        return; // 已处理 / 避免环
    }
    // 先深拷贝子节点索引列表（避免 borrow 冲突），递归
    let child_idxs: Vec<u16> = blob.node_pool[idx as usize].sub_nodes.clone();
    for c in child_idxs {
        rename_subtree(blob, c, visited);
    }
    rename_one(blob, idx);
}

/// 对应 AssetRipper `DoSecondaryRenaming`：按规则重命名单个节点。
fn rename_one(blob: &mut TpkTypeTree, idx: u16) {
    let tn = type_str(blob, idx).to_owned();
    if tn.is_empty() {
        return;
    }

    // 注意：AssetRipper 的 `string → Utf8String` 重命名这里**不做**。
    // 本 codegen 已通过 NodeType::String 特判正确处理 `string`（读 size + bytes + align(4)）；
    // 若改成 Utf8String subclass，会被当作普通 struct，丢失末尾对齐 → 字节错位。

    // TypeNameReplacement 字典
    if let Some(rep) = type_name_replacement(&tn) {
        set_typename(blob, idx, rep);
        return;
    }

    // OffsetPtr → OffsetPtr_<m_Data 类型>
    if tn == "OffsetPtr" {
        if let Some(data_tn) = get_subnode_typename_by_name(blob, idx, "m_Data") {
            let new_name = format!("OffsetPtr_{}", sanitize_type_for_compound(&data_tn));
            set_typename(blob, idx, new_name);
        }
        return;
    }

    // Keyframe → Keyframe_<m_Value 类型>（float → Single）
    if tn == "Keyframe" {
        if let Some(val_tn) = get_subnode_typename_by_name(blob, idx, "m_Value") {
            let elem = if val_tn == "float" { "Single".to_string() } else { val_tn };
            set_typename(blob, idx, format!("Keyframe_{elem}"));
        }
        return;
    }

    // AnimationCurve → AnimationCurve_<keyframe 类型>
    if tn == "AnimationCurve" {
        if let Some(elem_tn) = get_curve_element_typename(blob, idx) {
            set_typename(blob, idx, format!("AnimationCurve_{elem_tn}"));
        }
        return;
    }

    // ColorRGBA → ColorRGBAf (4 子节点) / ColorRGBA32 (1 子节点)
    if tn == "ColorRGBA" {
        let sc = blob.node_pool[idx as usize].sub_nodes.len();
        let new_name = match sc {
            4 => "ColorRGBAf",
            1 => "ColorRGBA32",
            _ => {
                log::warn!("rename: ColorRGBA 子节点数 {sc} 非 1/4，跳过");
                return;
            }
        };
        set_typename(blob, idx, new_name);
        return;
    }

    // PackedBitVector → _Single(5) / _Int32(3) / _Quaternionf(2)
    if tn == "PackedBitVector" {
        let sc = blob.node_pool[idx as usize].sub_nodes.len();
        let suffix = match sc {
            5 => "PackedBitVector_Single",
            3 => "PackedBitVector_Int32",
            2 => "PackedBitVector_Quaternionf",
            _ => {
                log::warn!("rename: PackedBitVector 子节点数 {sc} 非 2/3/5，跳过");
                return;
            }
        };
        set_typename(blob, idx, suffix);
        return;
    }

    // 以下为按字段名/子结构上下文判定重命名（只挑影响 subclass 标识的）

    // TilemapRefCountedData → TilemapRefCountedData_<m_Data 类型>
    if tn == "TilemapRefCountedData" {
        if let Some(data_tn) = get_subnode_typename_by_name(blob, idx, "m_Data") {
            set_typename(blob, idx, format!("TilemapRefCountedData_{}", sanitize_type_for_compound(&data_tn)));
        }
        return;
    }

    // Blend2dDataConstant：Unity4 时 Blend1d 也叫 Blend2d（单子节点 + m_ChildThresholdArray）
    if tn == "Blend2dDataConstant" {
        let subs = &blob.node_pool[idx as usize].sub_nodes;
        if subs.len() == 1 {
            let s0 = subs[0];
            if name_str(blob, blob.node_pool[s0 as usize].name_index) == "m_ChildThresholdArray" {
                set_typename(blob, idx, "Blend1dDataConstant");
            }
        }
        return;
    }

    // VariantInfo：有 m_Shader 字段 → ShaderVariantInfo（Unity 6+）
    if tn == "VariantInfo" && has_subnode_named(blob, idx, "m_Shader") {
        set_typename(blob, idx, "ShaderVariantInfo");
        return;
    }

    // ComputeShaderPlatformVariant → ComputeShaderVariant
    if tn == "ComputeShaderPlatformVariant" {
        set_typename(blob, idx, "ComputeShaderVariant");
        return;
    }

    // JointAngleLimit2D → JointAngleLimits2D
    if tn == "JointAngleLimit2D" {
        set_typename(blob, idx, "JointAngleLimits2D");
        return;
    }

    // Gradient：有 m_Color_0_ 子节点 → GradientOld
    if tn == "Gradient" && has_subnode_named(blob, idx, "m_Color_0_") {
        set_typename(blob, idx, "GradientOld");
        return;
    }

    // HumanLayerConstant → LayerConstant（与 LayerConstant 合并）
    if tn == "HumanLayerConstant" {
        set_typename(blob, idx, "LayerConstant");
        return;
    }

    // ClipMuscleConstant → MuscleClipConstant
    if tn == "ClipMuscleConstant" {
        set_typename(blob, idx, "MuscleClipConstant");
        return;
    }

    // AnimatorEvent → AnimatorControllerParameter
    if tn == "AnimatorEvent" {
        set_typename(blob, idx, "AnimatorControllerParameter");
        return;
    }

    // Child：有 m_IsAnim 子节点 → ChildMotion
    if tn == "Child" && has_subnode_named(blob, idx, "m_IsAnim") {
        set_typename(blob, idx, "ChildMotion");
        return;
    }

    // Condition：有 m_ConditionEvent 子节点 → AnimatorCondition
    if tn == "Condition" && has_subnode_named(blob, idx, "m_ConditionEvent") {
        set_typename(blob, idx, "AnimatorCondition");
        return;
    }

    // Output：按子节点字段判定 FontOutput / AudioImporterOutputOld
    if tn == "Output" {
        if has_subnode_named(blob, idx, "m_HasEmptyFontData") {
            set_typename(blob, idx, "FontOutput");
        } else if has_subnode_named(blob, idx, "m_PreviewData") {
            set_typename(blob, idx, "AudioImporterOutputOld");
            set_name(blob, idx, "m_OutputOld");
        }
        return;
    }

    // Google：按字段判定 GoogleCardboard / GoogleDayDream
    if tn == "Google" {
        if has_subnode_named(blob, idx, "m_EnableTransitionView") {
            set_typename(blob, idx, "GoogleCardboard");
        } else if has_subnode_named(blob, idx, "m_UseSustainedPerformanceMode") {
            set_typename(blob, idx, "GoogleDayDream");
        }
        return;
    }

    // PlatformSettings：有 m_MaxTextureSize → TextureImporterPlatformSettings
    if tn == "PlatformSettings" && has_subnode_named(blob, idx, "m_MaxTextureSize") {
        set_typename(blob, idx, "TextureImporterPlatformSettings");
        return;
    }

    // BuildTargetSettings：有 m_MaxTextureSize → TextureImporterPlatformSettings，否则 MaterialBuildTargetSettings
    if tn == "BuildTargetSettings" {
        let new = if has_subnode_named(blob, idx, "m_MaxTextureSize") {
            "TextureImporterPlatformSettings"
        } else {
            "MaterialBuildTargetSettings"
        };
        set_typename(blob, idx, new);
        return;
    }

    // PlatformSettingsData：有 m_Enabled → _Plugin，否则 _Editor
    if tn == "PlatformSettingsData" {
        let new = if has_subnode_named(blob, idx, "m_Enabled") {
            "PlatformSettingsData_Plugin"
        } else {
            "PlatformSettingsData_Editor"
        };
        set_typename(blob, idx, new);
        return;
    }

    // MultiModeParameter：无 m_Value → _MeshSpawn
    if tn == "MultiModeParameter" && !has_subnode_named(blob, idx, "m_Value") {
        set_typename(blob, idx, "MultiModeParameter_MeshSpawn");
        return;
    }

    // InputImportSettings：有 m_Value → _SubstanceValue
    if tn == "InputImportSettings" && has_subnode_named(blob, idx, "m_Value") {
        set_typename(blob, idx, "InputImportSettings_SubstanceValue");
        return;
    }

    // BuildTargetShaderSettings：有 m_ShaderSettings 字段 → TierSettings（5.5 前）
    if tn == "BuildTargetShaderSettings" {
        if try_rename_subnode(blob, idx, "m_ShaderSettings", "m_Settings") {
            set_typename(blob, idx, "TierSettings");
        }
        return;
    }

    // NameToObjectMap：按 m_ObjectToName 内层 PPtr 类型拆分
    if tn == "NameToObjectMap" {
        if let Some(t) = get_name_to_object_map_element_type(blob, idx) {
            set_typename(blob, idx, t);
        }
        return;
    }

    // VFX 系列：VFXEntryExposed / VFXEntryExpressionValue / VFXField → _<元素类型>
    if tn == "VFXEntryExposed" || tn == "VFXEntryExpressionValue" {
        if let Some(elem) = get_subnode_typename_by_name(blob, idx, "m_Value") {
            set_typename(blob, idx, format!("{tn}_{}", sanitize_type_for_compound(&elem)));
        }
        return;
    }
    if tn == "VFXField"
        && let Some(elem) = get_vfx_field_element_type(blob, idx)
    {
        set_typename(blob, idx, format!("VFXField_{}", sanitize_type_for_compound(&elem)));
    }

    // Xform / ValueArray：按内层 vector 类型给字段加后缀（仅字段重命名，不影响 type）
    // 这类是字段名后缀化以区分版本，p05 合并按 unity_name，对字节布局无影响，跳过。
    // （AssetRipper 改的是子节点 name，不是本节点 type。）

    // 其余纯字段重命名（m_Dst→m_Destination、Hash128 字段排序、m_Val→m_Value 等）
    // 不影响字节布局，跳过。
}

// ────────────────────────────────────────────────────────────────────
// AnimationCurve 内层元素类型提取
// ────────────────────────────────────────────────────────────────────

/// AnimationCurve → m_Curve(vector) → Array → data(Keyframe_xxx)。
/// 返回去掉 `Keyframe_` 前缀的元素类型名。
fn get_curve_element_typename(blob: &TpkTypeTree, idx: u16) -> Option<String> {
    let curve_idx = get_subnode_by_type_and_name(blob, idx, "vector", "m_Curve")?;
    // curve 的 sub_nodes[0] = Array wrapper
    let array_idx = blob.node_pool[curve_idx as usize].sub_nodes.first().copied()?;
    // Array 的 sub_nodes: [size, data]
    let data_idx = blob.node_pool[array_idx as usize].sub_nodes.get(1).copied()?;
    let data_tn = type_str(blob, data_idx);
    data_tn.strip_prefix("Keyframe_").map(|s| s.to_string())
}

// ────────────────────────────────────────────────────────────────────
// NameToObjectMap 元素类型提取
// ────────────────────────────────────────────────────────────────────

/// NameToObjectMap → m_ObjectToName(map) → Array → pair(data) → first(PPtr<...>)。
/// 返回 `NameToObjectMap_X`（X 为 PPtr 目标，已去除 `PPtr<` / `>` 并 sanitize）。
fn get_name_to_object_map_element_type(blob: &TpkTypeTree, idx: u16) -> Option<String> {
    let map_idx = get_subnode_by_type_and_name(blob, idx, "map", "m_ObjectToName")?;
    let array_idx = blob.node_pool[map_idx as usize].sub_nodes.first().copied()?;
    let pair_idx = blob.node_pool[array_idx as usize].sub_nodes.get(1).copied()?;
    let first_idx = blob.node_pool[pair_idx as usize].sub_nodes.first().copied()?;
    let first_tn = type_str(blob, first_idx).to_string();
    // 支持 PPtr<X> 和 PPtr_X 两种形式
    let target = if let Some(inner) = first_tn.strip_prefix("PPtr<").and_then(|s| s.strip_suffix('>')) {
        inner
    } else if let Some(inner) = first_tn.strip_prefix("PPtr_") {
        inner.trim_end_matches('_')
    } else {
        return None;
    };
    Some(format!("NameToObjectMap_{}", sanitize_type_for_compound(target)))
}

/// VFXField → m_Array(vector) → Array → data 的类型。
fn get_vfx_field_element_type(blob: &TpkTypeTree, idx: u16) -> Option<String> {
    let array_node_idx = get_subnode_by_type_and_name(blob, idx, "vector", "m_Array")?;
    let arr_wrapper = blob.node_pool[array_node_idx as usize].sub_nodes.first().copied()?;
    let data_idx = blob.node_pool[arr_wrapper as usize].sub_nodes.get(1).copied()?;
    Some(type_str(blob, data_idx).to_string())
}

// ────────────────────────────────────────────────────────────────────
// 命名表（对应 AssetRipper ClassNameReplacement / TypeNameReplacement）
// ────────────────────────────────────────────────────────────────────

fn class_name_replacement(name: &str) -> Option<&'static str> {
    match name {
        "AvatarBodyMask" => Some("AvatarMask"),
        "LightmapSnapshot" => Some("LightingDataAsset"),
        "PhysicMaterial" => Some("PhysicsMaterial"),
        "RenderManager" => Some("GraphicsSettings"),
        "State" => Some("AnimatorState"),
        "StateMachine" => Some("AnimatorStateMachine"),
        "Transition" => Some("AnimatorStateTransition"),
        _ => None,
    }
}

fn type_name_replacement(name: &str) -> Option<&'static str> {
    match name {
        "AlbedoSwatchInfo" => Some("AlbedoSwitchInfo"),
        "PlatformShaderSettings" => Some("TierGraphicsSettingsEditor"),
        "BoneInfluence" => Some("BoneWeights4"),
        "IntPoint" => Some("Vector2Long"),
        "Int2_storage" => Some("Vector2Int"),
        "Int3_storage" => Some("Vector3Int"),
        "Float3" => Some("Vector3Float"),
        "Float4" => Some("Vector4Float"),
        "Fixed_bitset" => Some("FixedBitset"),
        "GradientNEW" => Some("Gradient"),
        "ProgramParameters" => Some("SerializedProgramParameters"),
        "BufferBinding" => Some("BufferBindingParameter"),
        "ConstantBufferParameter" => Some("ConstantBuffer"),
        _ => None,
    }
}

// ────────────────────────────────────────────────────────────────────
// TpkTypeTree 访问辅助（只读查询 + 写入）
// ────────────────────────────────────────────────────────────────────

fn name_str(blob: &TpkTypeTree, idx: u16) -> &str {
    blob.string_pool
        .get(idx as usize)
        .map(String::as_str)
        .unwrap_or("")
}

fn type_str(blob: &TpkTypeTree, node_idx: u16) -> &str {
    let tn_idx = blob.node_pool[node_idx as usize].type_name_index;
    name_str(blob, tn_idx)
}

/// 把 type_name 指向新字符串（intern 到 string_pool 末尾，去重）。
fn set_typename(blob: &mut TpkTypeTree, node_idx: u16, new_name: impl AsRef<str>) {
    let new_idx = intern(blob, new_name.as_ref());
    blob.node_pool[node_idx as usize].type_name_index = new_idx;
}

fn set_name(blob: &mut TpkTypeTree, node_idx: u16, new_name: &str) {
    let new_idx = intern(blob, new_name);
    blob.node_pool[node_idx as usize].name_index = new_idx;
}

/// 字符串 intern：已存在返回索引，否则追加。返回 u16 索引（超 65535 报错）。
fn intern(blob: &mut TpkTypeTree, s: &str) -> u16 {
    if let Some(i) = blob.string_pool.iter().position(|x| x == s) {
        return i as u16;
    }
    let i = blob.string_pool.len();
    if i > u16::MAX as usize {
        panic!("rename: string_pool 超过 u16 范围");
    }
    blob.string_pool.push(s.to_string());
    i as u16
}

/// 按字段名找子节点，返回其 type_name。
fn get_subnode_typename_by_name(blob: &TpkTypeTree, idx: u16, field_name: &str) -> Option<String> {
    let node = &blob.node_pool[idx as usize];
    for &c in &node.sub_nodes {
        let cn = name_str(blob, blob.node_pool[c as usize].name_index);
        if cn == field_name {
            return Some(type_str(blob, c).to_string());
        }
    }
    None
}

/// 按 (type_name, field_name) 找子节点索引。
fn get_subnode_by_type_and_name(
    blob: &TpkTypeTree,
    idx: u16,
    type_name: &str,
    field_name: &str,
) -> Option<u16> {
    let node = &blob.node_pool[idx as usize];
    for &c in &node.sub_nodes {
        let cn = name_str(blob, blob.node_pool[c as usize].name_index);
        let ct = type_str(blob, c);
        if cn == field_name && ct == type_name {
            return Some(c);
        }
    }
    None
}

fn has_subnode_named(blob: &TpkTypeTree, idx: u16, field_name: &str) -> bool {
    let node = &blob.node_pool[idx as usize];
    node.sub_nodes
        .iter()
        .any(|&c| name_str(blob, blob.node_pool[c as usize].name_index) == field_name)
}

/// 重命名子节点字段名，找到返回 true。
fn try_rename_subnode(blob: &mut TpkTypeTree, idx: u16, cur: &str, new: &str) -> bool {
    let subs = blob.node_pool[idx as usize].sub_nodes.clone();
    for c in subs {
        if name_str(blob, blob.node_pool[c as usize].name_index) == cur {
            set_name(blob, c, new);
            return true;
        }
    }
    false
}

/// 把用于拼接新 type_name 的元素类型做 sanitize，对应 AssetRipper
/// `ValidNameGenerator.ReplaceBadCharacters`：把 `< > [ ] 空白 & ( ) : . -` 替换为 `_`。
///
/// 用于 OffsetPtr/TilemapRefCountedData/VFXField/NameToObjectMap 等拼名规则——
/// 内层元素可能是 `PPtr<GameObject>`，直接拼成 `OffsetPtr_PPtr<GameObject>` 会产生
/// 非法 Rust 标识符（文件名/模块名）。本函数把 `PPtr<GameObject>` → `PPtr_GameObject_`。
fn sanitize_type_for_compound(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' | '>' | '[' | ']' | ' ' | '\t' | '\n' | '\r' | '&' | '(' | ')' | ':' | '.' | '-' => '_',
            _ => c,
        })
        .collect()
}
