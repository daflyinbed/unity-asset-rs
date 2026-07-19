//! Pass002：节点重命名 / 拆分（移植自 AssetRipper `Pass002_RenameSubnodes.cs`）。
//!
//! TPK 里若干 type_name 在不同上下文下布局完全不同（如 `ColorRGBA` 既可能是
//! 4×float(16字节) 也可能是 UInt32(4字节)）。若不拆分，p05 会把它们当同一
//! subclass 合并，按版本切换 enum——但实际区别是**父类上下文**而非版本，
//! 导致 typed parse 读错字节（见 Material 的 ColorRGBA bug）。
//!
//! 本 pass 在 p01 之后、p02 之前对 `TpkTypeTree` 原地重命名，把同名异构类型
//! 拆成独立 subclass（`ColorRGBA` → `ColorRGBAf` / `ColorRGBA32` 等），让 p05
//! 各自独立处理。同时做**结构性拆分**、**类型/类名规范化**，以及**纯字段重命名**
//! （如 `m_Dst` → `m_Destination`、`m_TextureFormat` → `m_Format`）——后者虽不
//! 影响字节布局，但让跨版本的同字段不同名（如 `m_PrefabParentObject` /
//! `m_CorrespondingSourceObject`）被 p05 正确合并为单一字段，而非拆成两个
//! `Option<T>`。对齐 AssetRipper `Pass002_DoSecondaryRenaming` 的全部规则。
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
            let Some(cls) = entry.class.as_ref() else {
                continue;
            };
            let cur_name = name_str(blob, cls.name_index).to_string();
            let new_name = class_name_replacement(&cur_name)
                .map(|s| s.to_string())
                .unwrap_or(cur_name.clone());
            let cur_base = cls.base_index.map(|b| name_str(blob, b).to_string());
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

    // 2. 自下而上重命名整个节点池。
    //    p02 会扫整个 node_pool 抽 subclass（含 editor-only / 孤儿子类，见 §12.3），
    //    故 rename 也必须覆盖每个节点，否则孤儿子类的字段重命名会漏（如被
    //    MergeMovedGroups 移除的 class 的 release root 仍会被 p02 当 subclass 抽取）。
    //    visited 集合避免重复处理 + 避免环（TPK 节点池实测无环，但保险起见）。
    let mut visited = std::collections::HashSet::new();
    for idx in 0..blob.node_pool.len() as u16 {
        rename_subtree(blob, idx, &mut visited);
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

    // string 节点：按子节点形态判定。对应 AssetRipper `Pass002::ChangeStringToUtf8String`。
    //
    // 绝大多数 string 节点的唯一子节点是 Array（正常 string：size + bytes + align），
    // NodeType::String 正确处理，不改名。
    //
    // 例外（ExposedReferenceTable / ExtensionPropertyValue 等的 PropertyName 字段）：
    //   - 子节点是 SInt32/int（4 字节 CRC 哈希）→ 旧 PropertyName 布局，重命名为
    //     `PropertyName32`（subclass 读一个 i32 + align）。
    //   - 子节点是 string/Utf8String（嵌套真 string）→ 新 PropertyName 布局，重命名
    //     为 `PropertyNameNested`（subclass 读一个 aligned_string）。
    //
    // 若不改名，NodeType::classify 会把这两种非 Array 子节点的 string 一律当
    // NodeType::String → read_aligned_string()，读错字节（磁盘是 int 而非 len+bytes）。
    if tn == "string" {
        if blob.node_pool[idx as usize].sub_nodes.len() == 1 {
            let child_idx = blob.node_pool[idx as usize].sub_nodes[0];
            let child_tn = type_str(blob, child_idx);
            if child_tn == "SInt32" || child_tn == "int" {
                set_typename(blob, idx, "PropertyName32");
                return;
            }
            if child_tn == "string" || child_tn == "Utf8String" {
                set_typename(blob, idx, "PropertyNameNested");
                return;
            }
        }
        // 子节点是 Array 或其他 → 正常 string，不改名
        return;
    }

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
            let elem = if val_tn == "float" {
                "Single".to_string()
            } else {
                val_tn
            };
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
            set_typename(
                blob,
                idx,
                format!(
                    "TilemapRefCountedData_{}",
                    sanitize_type_for_compound(&data_tn)
                ),
            );
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
            set_typename(
                blob,
                idx,
                format!("{tn}_{}", sanitize_type_for_compound(&elem)),
            );
        }
        return;
    }
    if tn == "VFXField"
        && let Some(elem) = get_vfx_field_element_type(blob, idx)
    {
        set_typename(
            blob,
            idx,
            format!("VFXField_{}", sanitize_type_for_compound(&elem)),
        );
    }

    // Xform：m_T/m_S 按内层类型加后缀（4 → Vector4Float，3 → Vector3Float）
    if tn == "Xform" {
        if let Some(t_tn) = get_subnode_typename_by_name(blob, idx, "m_T") {
            let new = match t_tn.as_str() {
                "Vector4Float" => "m_T4",
                "Vector3Float" => "m_T3",
                _ => &t_tn[..],
            };
            try_rename_subnode(blob, idx, "m_T", new);
        }
        if let Some(s_tn) = get_subnode_typename_by_name(blob, idx, "m_S") {
            let new = match s_tn.as_str() {
                "Vector4Float" => "m_S4",
                "Vector3Float" => "m_S3",
                _ => &s_tn[..],
            };
            try_rename_subnode(blob, idx, "m_S", new);
        }
    }

    // ValueArray：m_PositionValues/m_ScaleValues 按内层 vector data 类型加后缀
    if tn == "ValueArray" {
        if let Some(pos_tn) = get_vector_data_typename(blob, idx, "m_PositionValues") {
            let suffix = match pos_tn.as_str() {
                "Vector4Float" => "4",
                "Vector3Float" => "3",
                _ => "",
            };
            if !suffix.is_empty() {
                try_rename_subnode(
                    blob,
                    idx,
                    "m_PositionValues",
                    &format!("m_PositionValues{suffix}"),
                );
            }
        }
        if let Some(scl_tn) = get_vector_data_typename(blob, idx, "m_ScaleValues") {
            let suffix = match scl_tn.as_str() {
                "Vector4Float" => "4",
                "Vector3Float" => "3",
                _ => "",
            };
            if !suffix.is_empty() {
                try_rename_subnode(
                    blob,
                    idx,
                    "m_ScaleValues",
                    &format!("m_ScaleValues{suffix}"),
                );
            }
        }
    }

    // ──────────────────────────────────────────────────────────────
    // 纯字段重命名（改子节点 name，不影响字节布局）。
    // 对应 AssetRipper Pass002 DoSecondaryRenaming 里的 TryRenameSubNode 规则。
    // 按父 type_name 分发。
    // ──────────────────────────────────────────────────────────────
    match tn.as_str() {
        "GraphicsSettings" => {
            try_rename_subnode(blob, idx, "m_BuildTargetShaderSettings", "m_TierSettings");
            try_rename_subnode(blob, idx, "m_AlbedoSwatchInfos", "m_AlbedoSwitchInfos");
        }
        "LightmapSettings" => {
            try_rename_subnode(blob, idx, "m_LightmapSnapshot", "m_LightingDataAsset");
        }
        "TextureImporter" => {
            try_rename_subnode(blob, idx, "m_BuildTargetSettings", "m_PlatformSettings");
        }
        "Mesh" => {
            // m_Shapes(vector) → m_ShapesList（避免与 m_ShapesList subclass 混淆）
            if has_subnode_by_type_and_name(blob, idx, "vector", "m_Shapes") {
                try_rename_subnode(blob, idx, "m_Shapes", "m_ShapesList");
            }
        }
        "GameObject" => {
            // 内层 pair(5.5 前) → ComponentPair，m_First→m_ClassID、m_Second→m_Component
            if let Some(arr_idx) = get_subnode_by_type_and_name(blob, idx, "vector", "m_Component")
            {
                let pair_idx = blob.node_pool[arr_idx as usize].sub_nodes.first().copied();
                if let Some(pi) = pair_idx {
                    let data_idx = blob.node_pool[pi as usize].sub_nodes.get(1).copied();
                    if let Some(di) = data_idx {
                        let d_tn = type_str(blob, di);
                        if d_tn == "pair" {
                            set_typename(blob, di, "ComponentPair");
                            try_rename_subnode(blob, di, "m_First", "m_ClassID");
                            try_rename_subnode(blob, di, "m_Second", "m_Component");
                        }
                    }
                }
            }
            try_rename_subnode(blob, idx, "m_Component", "m_Components");
        }
        "Shader" => {
            try_rename_subnode(blob, idx, "m_SubProgramBlob", "m_CompressedBlob");
        }
        "BuildSettings" => {
            try_rename_subnode(blob, idx, "m_Levels", "m_Scenes");
        }
        "ParticleSystem" => {
            // m_StartDelay 按内层类型改：float → m_StartDelay_Single，否则 m_StartDelay_MinMaxCurve
            if let Some(t) = get_subnode_typename_by_name(blob, idx, "m_StartDelay") {
                let new = if t == "float" {
                    "m_StartDelay_Single"
                } else {
                    "m_StartDelay_MinMaxCurve"
                };
                try_rename_subnode(blob, idx, "m_StartDelay", new);
            }
        }
        "BlendTree" => {
            try_rename_subnode(blob, idx, "m_BlendEvent", "m_BlendParameter");
            try_rename_subnode(blob, idx, "m_BlendEventY", "m_BlendParameterY");
        }
        "AnimatorController" => {
            try_rename_subnode(blob, idx, "m_AnimatorEvents", "m_AnimatorParameters");
            try_rename_subnode(blob, idx, "m_Layers", "m_AnimatorLayers");
        }
        "AnimatorControllerLayer" => {
            try_rename_subnode(blob, idx, "m_HumanMask", "m_Mask");
        }
        "AnimationClip" => {
            try_rename_subnode(blob, idx, "m_AnimationClipSettings", "m_MuscleClipInfo");
        }
        "GenericBinding" => {
            try_rename_subnode(blob, idx, "m_TypeID", "m_ClassID");
        }
        "MeshRenderer" => {
            try_rename_subnode(blob, idx, "m_StitchSeams", "m_StitchLightmapSeams");
        }
        "VisualEffectResource" => {
            if has_subnode_by_type_and_name(blob, idx, "VisualEffectSettings", "m_Infos") {
                try_rename_subnode(blob, idx, "m_Infos", "m_Settings");
            }
        }
        "Prefab" => {
            try_rename_subnode(blob, idx, "m_IsPrefabParent", "m_IsPrefabAsset");
            try_rename_subnode(blob, idx, "m_ParentPrefab", "m_SourcePrefab");
        }
        "PrefabInstance" => {
            // m_SourcePrefab 的 type 树错误，应为 PPtr_PrefabInstance 而非 PPtr_Prefab
            if let Some(sp_idx) = get_subnode_by_name(blob, idx, "m_SourcePrefab") {
                set_typename(blob, sp_idx, "PPtr_PrefabInstance");
            }
        }
        "VertexData" => {
            try_rename_subnode(blob, idx, "m_DataSize", "m_Data");
        }
        "ControllerConstant" => {
            try_rename_subnode(blob, idx, "m_HumanLayerArray", "m_LayerArray");
        }
        "StateMachineConstant" => {
            try_rename_subnode(blob, idx, "m_MotionSetCount", "m_SynchronizedLayerCount");
        }
        "Heightmap" => {
            try_rename_subnode(
                blob,
                idx,
                "m_EnableSurfaceMaskTextureCompression",
                "m_EnableHolesTextureCompression",
            );
            try_rename_subnode(blob, idx, "m_SurfaceMask", "m_Holes");
            try_rename_subnode(blob, idx, "m_SurfaceMaskLOD", "m_HolesLOD");
        }
        "ShapeModule" => {
            // m_Arc / m_Radius：非 float 类型时加 Parameter 后缀
            if let Some(t) = get_subnode_typename_by_name(blob, idx, "m_Arc") {
                if t != "float" {
                    try_rename_subnode(blob, idx, "m_Arc", "m_ArcParameter");
                }
            }
            if let Some(t) = get_subnode_typename_by_name(blob, idx, "m_Radius") {
                if t != "float" {
                    try_rename_subnode(blob, idx, "m_Radius", "m_RadiusParameter");
                }
            }
        }
        "Texture3D" => {
            try_rename_subnode(blob, idx, "m_DataSize", "m_CompleteImageSize");
            try_rename_subnode(blob, idx, "m_ImageCount", "m_Depth");
        }
        "MuscleClipConstant" => {
            // m_AverageSpeed 按内层类型加后缀（4 → Vector4Float，3 → Vector3Float）
            if let Some(t) = get_subnode_typename_by_name(blob, idx, "m_AverageSpeed") {
                let new = match t.as_str() {
                    "Vector4Float" => "m_AverageSpeed4",
                    "Vector3Float" => "m_AverageSpeed3",
                    _ => &t[..],
                };
                try_rename_subnode(blob, idx, "m_AverageSpeed", new);
            }
        }
        "LayerConstant" => {
            try_rename_subnode(
                blob,
                idx,
                "m_StateMachineMotionSetIndex",
                "m_StateMachineSynchronizedLayerIndex",
            );
        }
        _ => {}
    }

    // LayerConstant 特殊字段名（无 m_ 前缀的带类型名前缀）
    if tn == "LayerConstant" || tn == "HumanLayerConstant" {
        try_rename_subnode(
            blob,
            idx,
            "m__int__m_LayerBlendingMode",
            "m_LayerBlendingMode",
        );
        // m__SInt32__m_* → 去掉 m__SInt32__ 前缀（AudioClip 3.5–5）
        rename_sint32_prefixed_children(blob, idx);
    }

    // ──────────────────────────────────────────────────────────────
    // 通用字段重命名（对所有节点的子节点遍历，不限定父 type）。
    // ──────────────────────────────────────────────────────────────
    let child_idxs = blob.node_pool[idx as usize].sub_nodes.clone();
    for &c in &child_idxs {
        let cname = name_str(blob, blob.node_pool[c as usize].name_index).to_owned();
        let ctn = type_str(blob, c).to_owned();
        let new_name: Option<&str> = if cname == "m_Image_data" {
            Some("m_ImageData")
        } else if cname == "m_TextureFormat" {
            Some("m_Format")
        } else if cname == "m_TextureDimension" {
            Some("m_Dimension")
        } else if cname == "m_ObjectHideFlags" {
            Some("m_HideFlags")
        } else if cname == "m_Val" {
            Some("m_Value")
        } else {
            None
        };
        if let Some(new) = new_name {
            set_name(blob, c, new);
            continue;
        }
        // m_Dst* / m_Dest* / m_Src* 前缀归一（第 5/6 字符大写时）
        if let Some(rest) = cname.strip_prefix("m_Dst") {
            if rest.chars().next().is_some_and(char::is_uppercase) {
                set_name(blob, c, &format!("m_Destination{rest}"));
                continue;
            }
        }
        if let Some(rest) = cname.strip_prefix("m_Dest") {
            if rest.chars().next().is_some_and(char::is_uppercase) {
                set_name(blob, c, &format!("m_Destination{rest}"));
                continue;
            }
        }
        if let Some(rest) = cname.strip_prefix("m_Src") {
            if rest.chars().next().is_some_and(char::is_uppercase) {
                set_name(blob, c, &format!("m_Source{rest}"));
                continue;
            }
        }
        // PPtr<X> 子节点字段重命名：m_PrefabParentObject → m_CorrespondingSourceObject
        // （3.5–2018.2 旧名归一到新名，使跨版本同字段被 p05 正确合并）。
        // 注意：AssetRipper 还改 m_FileID→m_FileID_、m_PathID→m_PathID_，但那些是 PPtr
        // 内部字段，本 codegen 的 PPtr 是扁平 struct（PPtr::parse 直接读两整数），
        // 内部字段不单独生成，故无需改。
        if ctn.starts_with("PPtr<") && cname == "m_PrefabParentObject" {
            set_name(blob, c, "m_CorrespondingSourceObject");
        }
    }

    // Hash128 字段排序归一（m_Bytes_N_ → m_Bytes__N 或 m_Bytes_N，使顺序连续）
    if tn == "Hash128" {
        for i in 0..10 {
            try_rename_subnode(
                blob,
                idx,
                &format!("m_Bytes_{i}_"),
                &format!("m_Bytes__{i}"),
            );
        }
        for i in 10..16 {
            try_rename_subnode(blob, idx, &format!("m_Bytes_{i}_"), &format!("m_Bytes_{i}"));
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// AnimationCurve 内层元素类型提取
// ────────────────────────────────────────────────────────────────────

/// AnimationCurve → m_Curve(vector) → Array → data(Keyframe_xxx)。
/// 返回去掉 `Keyframe_` 前缀的元素类型名。
fn get_curve_element_typename(blob: &TpkTypeTree, idx: u16) -> Option<String> {
    let curve_idx = get_subnode_by_type_and_name(blob, idx, "vector", "m_Curve")?;
    // curve 的 sub_nodes[0] = Array wrapper
    let array_idx = blob.node_pool[curve_idx as usize]
        .sub_nodes
        .first()
        .copied()?;
    // Array 的 sub_nodes: [size, data]
    let data_idx = blob.node_pool[array_idx as usize]
        .sub_nodes
        .get(1)
        .copied()?;
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
    let array_idx = blob.node_pool[map_idx as usize]
        .sub_nodes
        .first()
        .copied()?;
    let pair_idx = blob.node_pool[array_idx as usize]
        .sub_nodes
        .get(1)
        .copied()?;
    let first_idx = blob.node_pool[pair_idx as usize]
        .sub_nodes
        .first()
        .copied()?;
    let first_tn = type_str(blob, first_idx).to_string();
    // 支持 PPtr<X> 和 PPtr_X 两种形式
    let target = if let Some(inner) = first_tn
        .strip_prefix("PPtr<")
        .and_then(|s| s.strip_suffix('>'))
    {
        inner
    } else if let Some(inner) = first_tn.strip_prefix("PPtr_") {
        inner.trim_end_matches('_')
    } else {
        return None;
    };
    Some(format!(
        "NameToObjectMap_{}",
        sanitize_type_for_compound(target)
    ))
}

/// VFXField → m_Array(vector) → Array → data 的类型。
fn get_vfx_field_element_type(blob: &TpkTypeTree, idx: u16) -> Option<String> {
    let array_node_idx = get_subnode_by_type_and_name(blob, idx, "vector", "m_Array")?;
    let arr_wrapper = blob.node_pool[array_node_idx as usize]
        .sub_nodes
        .first()
        .copied()?;
    let data_idx = blob.node_pool[arr_wrapper as usize]
        .sub_nodes
        .get(1)
        .copied()?;
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

/// 按 (type_name, field_name) 判断子节点是否存在。
fn has_subnode_by_type_and_name(
    blob: &TpkTypeTree,
    idx: u16,
    type_name: &str,
    field_name: &str,
) -> bool {
    get_subnode_by_type_and_name(blob, idx, type_name, field_name).is_some()
}

/// 按字段名找子节点索引（不限类型）。
fn get_subnode_by_name(blob: &TpkTypeTree, idx: u16, field_name: &str) -> Option<u16> {
    let node = &blob.node_pool[idx as usize];
    for &c in &node.sub_nodes {
        let cn = name_str(blob, blob.node_pool[c as usize].name_index);
        if cn == field_name {
            return Some(c);
        }
    }
    None
}

/// ValueArray 等的 vector 字段内层 data 类型：vector → Array → data。
fn get_vector_data_typename(blob: &TpkTypeTree, idx: u16, field_name: &str) -> Option<String> {
    let vec_idx = get_subnode_by_type_and_name(blob, idx, "vector", field_name)?;
    let array_idx = blob.node_pool[vec_idx as usize]
        .sub_nodes
        .first()
        .copied()?;
    let data_idx = blob.node_pool[array_idx as usize]
        .sub_nodes
        .get(1)
        .copied()?;
    Some(type_str(blob, data_idx).to_string())
}

/// 把 LayerConstant/AudioClip 的 `m__SInt32__m_X` 子节点改名去掉类型前缀。
/// 对应 AssetRipper `m__SInt32__m_` → `m_`（AudioClip 3.5–5 独有）。
fn rename_sint32_prefixed_children(blob: &mut TpkTypeTree, idx: u16) {
    let subs = blob.node_pool[idx as usize].sub_nodes.clone();
    for c in subs {
        let cn = name_str(blob, blob.node_pool[c as usize].name_index).to_string();
        if let Some(rest) = cn.strip_prefix("m__SInt32__m_") {
            set_name(blob, c, &format!("m_{rest}"));
        }
    }
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
            '<' | '>' | '[' | ']' | ' ' | '\t' | '\n' | '\r' | '&' | '(' | ')' | ':' | '.'
            | '-' => '_',
            _ => c,
        })
        .collect()
}
