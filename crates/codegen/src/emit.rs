//! p07：代码发射。
//!
//! 把 `Pipeline` 转成多个 Rust 源文件，写到指定目录。
//! 输出布局（详见 `docs/codegen.md` §9）：
//!
//! ```text
//! <out_dir>/
//! ├── lib.rs                 # 导出 + Parse trait + EndianReader trait + PPtr
//! ├── unity_object.rs        # enum UnityObject 分派
//! ├── class_id.rs            # enum ClassIDType
//! ├── classes/
//! │   ├── mod.rs
//! │   └── {snake_name}.rs    # 每个主 class
//! └── subclasses/
//!     ├── mod.rs
//!     └── {snake_name}.rs    # 每个 subclass
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::ir::{EmitClass, EmitField, EnumVariant, Pipeline, RustType, VersionGate};
use crate::names::to_rust_type_name;

/// 把整个 Pipeline emit 到 `out_dir`。
pub fn emit_all(state: &Pipeline, out_dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir.join("classes"))?;
    std::fs::create_dir_all(out_dir.join("subclasses"))?;

    // 只生成非抽象类（抽象父类 root 为空，字段已平铺到子类）
    let emit_classes: Vec<&EmitClass> = state.classes.values().filter(|c| !c.is_abstract).collect();

    // 主 class 的 Rust 名字集合。某些 main class（如 Vector3f、Collider、Scene）也被
    // 其他类按内嵌 struct（RustType::Subclass）引用 —— 这些引用必须指向 classes/ 而非
    // subclasses/，因为同名 subclass 已不再生成（见 p02 的 main_names 过滤）。
    let main_names: BTreeSet<String> = state
        .classes
        .values()
        .filter(|c| c.is_main() && !c.is_abstract)
        .map(|c| c.name.clone())
        .collect();

    // 按需 emit 父类 enum 定义；只有当某 main class 是 Variable 时才需要。
    // 同时记录父类 enum 已写哪些 class（避免重复）。
    let main_classes: Vec<&EmitClass> = state
        .classes
        .values()
        .filter(|c| c.is_main() && !c.is_abstract)
        .collect();

    // 每个主 class / subclass 一个文件
    for class in &emit_classes {
        let sub = class_matches_subclass(class);
        let dir = if sub { "subclasses" } else { "classes" };
        let file_name = format!("{}.rs", snake_case(&class.name));
        let path = out_dir.join(dir).join(file_name);
        let src = emit_class_file(class, &main_names);
        std::fs::write(path, src)?;
    }

    // classes/mod.rs 和 subclasses/mod.rs
    let classes_mod = emit_mod_rs(
        &state
            .classes
            .values()
            .filter(|c| c.is_main() && !c.is_abstract)
            .collect::<Vec<_>>(),
    );
    let subclasses_mod = emit_mod_rs(
        &state
            .classes
            .values()
            .filter(|c| !c.is_main())
            .collect::<Vec<_>>(),
    );
    std::fs::write(out_dir.join("classes").join("mod.rs"), classes_mod)?;
    std::fs::write(out_dir.join("subclasses").join("mod.rs"), subclasses_mod)?;

    // lib.rs：根 + Parse/EndianReader trait + PPtr
    let lib_rs = emit_lib_rs();
    std::fs::write(out_dir.join("lib.rs"), lib_rs)?;

    // unity_object.rs：enum UnityObject + parse 分派
    let unity_object_rs = emit_unity_object(&main_classes);
    std::fs::write(out_dir.join("unity_object.rs"), unity_object_rs)?;

    // class_id.rs：enum ClassIDType
    let class_id_rs = emit_class_id(&main_classes);
    std::fs::write(out_dir.join("class_id.rs"), class_id_rs)?;

    Ok(())
}

fn class_matches_subclass(class: &EmitClass) -> bool {
    !class.is_main()
}

// ────────────────────────────────────────────────────────────────────
// 单个 class 文件
// ────────────────────────────────────────────────────────────────────

fn emit_class_file(class: &EmitClass, main_names: &BTreeSet<String>) -> String {
    let mut out = String::new();
    out.push_str("//! 自动生成。请勿手动修改。\n\n");

    // 收集 imports
    let imports = collect_imports(class, main_names);
    if !imports.is_empty() {
        out.push_str(&imports);
        out.push('\n');
    }

    // 版本枚举定义 + impl Parse（在 struct 之前，因为 struct 引用枚举类型）
    emit_versioned_enums(&mut out, class);

    // 值依赖特判枚举（目前仅 pre-5.0 AudioClip.m_AudioData）。
    // 与 VersionedEnum 不同：这类枚举的读取依赖宿主此前读入的兄弟字段值，
    // 不能独立 `impl Parse`，由宿主 parse 直接 emit 构造。
    if class.name == "AudioClip" {
        emit_audio_data_enum(&mut out);
    }

    // struct 定义
    emit_struct(&mut out, class);

    // impl Parse
    emit_impl_parse(&mut out, class);

    out
}

fn collect_imports(class: &EmitClass, main_names: &BTreeSet<String>) -> String {
    let mut imports = BTreeSet::new();
    for f in &class.own_fields {
        collect_type_imports(&f.rust_type, &mut imports, main_names);
    }
    // version_layouts 的类型不再需要单独扫描：VersionedEnum 已把所有变体类型
    // 包含在 own_fields 里，collect_type_imports 会递归扫描变体。

    let mut s = String::new();
    for imp in imports {
        s.push_str(&format!("use {imp};\n"));
    }
    s
}

fn collect_type_imports(t: &RustType, out: &mut BTreeSet<String>, main_names: &BTreeSet<String>) {
    match t {
        RustType::Bool
        | RustType::I8
        | RustType::U8
        | RustType::I16
        | RustType::U16
        | RustType::I32
        | RustType::U32
        | RustType::I64
        | RustType::U64
        | RustType::F32
        | RustType::F64
        | RustType::String
        | RustType::Bytes => {}
        RustType::List(t) => collect_type_imports(t, out, main_names),
        RustType::Pair(a, b) => {
            collect_type_imports(a, out, main_names);
            collect_type_imports(b, out, main_names);
        }
        RustType::Pptr { target_class: _ } => {
            // MVP: PPtr 非 generic，无需 import target class
            out.insert("crate::PPtr".into());
        }
        RustType::Subclass(name) => {
            // 若名字是 main class，引用 classes/；否则 subclasses/。
            // （Vector3f / Collider / Scene 等 main class 也被按内嵌 struct 引用。）
            let dir = if main_names.contains(name) {
                "classes"
            } else {
                "subclasses"
            };
            out.insert(format!("crate::{dir}::{}::{name}", module_name(name)));
        }
        RustType::Option(inner) => {
            // gated 字段：递归到 inner
            collect_type_imports(inner, out, main_names);
        }
        RustType::VersionedEnum { name: _, variants } => {
            // 版本枚举：扫描每个变体的实际类型（参见 §12.2）
            for v in variants {
                collect_type_imports(&v.rust_type, out, main_names);
            }
        }
        RustType::Box(inner) => {
            collect_type_imports(inner, out, main_names);
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 版本枚举 emit（参见 docs/codegen.md §2.3 / §5.3）
// ────────────────────────────────────────────────────────────────────

/// 扫描 own_fields 中的 VersionedEnum 类型，emit 枚举定义 + impl Parse。
fn emit_versioned_enums(out: &mut String, class: &EmitClass) {
    for f in &class.own_fields {
        match &f.rust_type {
            RustType::VersionedEnum { name, variants } => {
                emit_versioned_enum(out, name, variants);
            }
            RustType::Option(inner) => {
                if let RustType::VersionedEnum { name, variants } = inner.as_ref() {
                    emit_versioned_enum(out, name, variants);
                }
            }
            _ => {}
        }
    }
}

/// emit 单个版本枚举定义 + 其 impl Parse。
fn emit_versioned_enum(out: &mut String, name: &str, variants: &[EnumVariant]) {
    // 枚举定义
    out.push_str(&format!("\n/// 版本枚举：{} 的类型跨版本变化。\n", name));
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub enum {} {{\n", name));
    for v in variants {
        let vname = variant_name(&v.gate);
        let ty_str = v.rust_type.to_rust_src();
        out.push_str(&format!("    {}({}),\n", vname, ty_str));
    }
    out.push_str("}\n");

    // impl Parse：按版本降序判断（新版本优先），Always 变体走 else。
    out.push_str(&format!("\nimpl crate::Parse for {} {{\n", name));
    out.push_str(
        "    fn parse<R: crate::EndianReader>(\n        rdr: &mut R,\n        ver: &crate::UnityVersion,\n    ) -> Result<Self, R::Error> {\n",
    );

    for (i, v) in variants.iter().rev().enumerate() {
        let vname = variant_name(&v.gate);
        let is_always = matches!(v.gate, VersionGate::Always);

        if is_always {
            if i == 0 {
                out.push_str("        {\n");
            } else {
                out.push_str("        } else {\n");
            }
        } else {
            let cond = gate_to_rust_cond(&v.gate);
            if i == 0 {
                out.push_str(&format!("        if {cond} {{\n"));
            } else {
                out.push_str(&format!("        }} else if {cond} {{\n"));
            }
        }

        let expr = field_read_expr(&v.rust_type);
        out.push_str(&format!("            let v = {expr}?;\n"));
        if v.align_after {
            out.push_str("            rdr.align(4)?;\n");
        }
        out.push_str(&format!("            Ok(Self::{vname}(v))\n"));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n");
}

/// emit `AudioClipMAudioData` 枚举：pre-5.0 AudioClip.m_AudioData 的值依赖读取结果。
///
/// 对齐 AssetRipper `Pass505_FixOldAudioClip` + `AudioClipHelper.ReadOldByteArray`：
/// - `m_Stream != 2`（DecompressInMemory / LoadFromMemory）：内联 `i32 count + bytes + align`
///   → `Inline(Vec<u8>)`
/// - `m_Stream == 2`（Streaming）：磁盘是 `u32 size; u32 offset;` 指向
///   `<collection.name>.resS`，字节在 Environment 层按 offset/size 取
///   → `Streamed { size, offset }`
///
/// 不生成 `impl Parse`——读取依赖宿主已读到的 `m_stream` 值，由宿主 parse 直接构造
/// （见 `set_field_none` 的 `m_audio_data` 特判）。
fn emit_audio_data_enum(out: &mut String) {
    out.push_str(
        "\n/// pre-5.0 AudioClip.m_AudioData 的值依赖读取结果。\n\
/// 对齐 AssetRipper `Pass505_FixOldAudioClip` + `AudioClipHelper.ReadOldByteArray`：\n\
/// `m_Stream==2`（Streaming）时磁盘是 `u32 size; u32 offset;` 指向\n\
/// `<collection.name>.resS`，由 Environment 按 offset/size 回填字节。\n",
    );
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub enum AudioClipMAudioData {\n");
    out.push_str("    /// 内联（pre-5.0 且 m_Stream != 2）：i32 count + bytes + align。\n");
    out.push_str("    Inline(Vec<u8>),\n");
    out.push_str("    /// 流式（pre-5.0 且 m_Stream == 2）：u32 size; u32 offset;\n");
    out.push_str("    /// 指向 `<collection.name>.resS`，字节由 Environment 取。\n");
    out.push_str("    Streamed { size: u32, offset: u32 },\n");
    out.push_str("}\n");
}
/// build 非 0 时附加 build 号，避免同 major.minor 不同 build 的变体重名。
/// 参见 §12.6：避免用 Legacy/Modern，改用版本号。
fn variant_name(gate: &VersionGate) -> String {
    match gate {
        VersionGate::Always => "Default".into(),
        VersionGate::Since(maj, min, 0) => format!("V{maj}_{min}"),
        VersionGate::Since(maj, min, build) => format!("V{maj}_{min}_{build}"),
        VersionGate::Range {
            from: (maj, min, 0),
            ..
        } => format!("V{maj}_{min}"),
        VersionGate::Range {
            from: (maj, min, build),
            ..
        } => format!("V{maj}_{min}_{build}"),
    }
}

/// 判断该类型是否自己处理 align（VersionedEnum 的 Parse 内部对齐）。
/// 宿主 class 不应对这类字段额外 align。
fn type_handles_own_align(t: &RustType) -> bool {
    match t {
        RustType::VersionedEnum { .. } => true,
        RustType::Option(inner) => type_handles_own_align(inner),
        RustType::Box(inner) => type_handles_own_align(inner),
        _ => false,
    }
}

fn emit_struct(out: &mut String, class: &EmitClass) {
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub struct {} {{\n", class.name));

    // own fields
    for f in &class.own_fields {
        // 值依赖特判：pre-5.0 AudioClip.m_AudioData 用专用 enum（见 set_field_none）。
        let ty_str = if class.name == "AudioClip" && f.rust_name == "m_audio_data" {
            "Option<AudioClipMAudioData>".into()
        } else {
            f.rust_type.to_rust_src()
        };
        out.push_str(&format!("    pub {}: {},\n", f.rust_name, ty_str));
    }
    out.push_str("}\n");
}

fn emit_impl_parse(out: &mut String, class: &EmitClass) {
    out.push_str(&format!("\nimpl crate::Parse for {} {{\n", class.name));
    out.push_str(
        "    fn parse<R: crate::EndianReader>(\n        rdr: &mut R,\n        ver: &crate::UnityVersion,\n    ) -> Result<Self, R::Error> {\n",
    );

    // 解 own fields
    if class.version_layouts.is_empty() {
        // 所有版本顺序一致：线性读
        for f in &class.own_fields {
            emit_field_parse(out, f);
        }
    } else {
        // 版本间顺序不同：version-gated 块
        emit_versioned_field_reads(out, class);
    }

    // 构造 Self
    out.push_str(&format!("\n        Ok({} {{\n", class.name));
    if class.version_layouts.is_empty() {
        for f in &class.own_fields {
            out.push_str(&format!("            {},\n", f.rust_name));
        }
    } else {
        for f in &class.own_fields {
            if matches!(f.version_gate, Some(VersionGate::Always)) {
                out.push_str(&format!(
                    "            {}: {}.unwrap(),\n",
                    f.rust_name, f.rust_name
                ));
            } else {
                out.push_str(&format!("            {},\n", f.rust_name));
            }
        }
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n");
}

/// 生成 version-gated parse 块：按 TPK release_root 的真实顺序读字段。
///
/// **锚点分段优化**：先求所有版本字段序列的公共子序列（LCS）作为「锚点」。
/// 锚点在所有版本里相对顺序一致 → 线性读一次即可。锚点之间的「段」才可能
/// 因版本而异 → 只对这些段做 version-gate，且**每段只在自己的变化版本点分支**，
/// 而非把全部分支套到所有字段上。这样大幅减少生成代码体积。
fn emit_versioned_field_reads(out: &mut String, class: &EmitClass) {
    let own_map: BTreeMap<&str, &EmitField> = class
        .own_fields
        .iter()
        .map(|f| (f.rust_name.as_str(), f))
        .collect();

    // Per-version field maps：`align_after` 等 field 属性可能跨版本不同。
    // version-gated 分支必须用对应版本 layout 的 EmitField，不能用 merged own_fields
    // （否则会把最新版本的 align_after 错误地套到老版本分支上，导致多读/少读 align 字节）。
    let version_field_maps: BTreeMap<(u16, u16, u16), BTreeMap<&str, &EmitField>> = class
        .version_layouts
        .iter()
        .map(|l| {
            let map = l.fields.iter().map(|f| (f.rust_name.as_str(), f)).collect();
            (l.since, map)
        })
        .collect();

    // 所有版本的字段名顺序
    let orders: Vec<Vec<String>> = class
        .version_layouts
        .iter()
        .map(|l| l.fields.iter().map(|f| f.rust_name.clone()).collect())
        .collect();

    // 锚点 = 全部版本字段序列的公共子序列
    let anchors = compute_anchors(&orders);

    // 声明所有 own_fields 为 None（锚点也走 None+Some，保持构造逻辑一致）
    for f in &class.own_fields {
        out.push_str(&format!("        let mut {} = None;\n", f.rust_name));
    }

    if anchors.is_empty() {
        // 退化：无公共锚点，全量 version-gated（每版本读全部字段）
        emit_full_versioned_branches(out, class, &own_map, &version_field_maps);
        return;
    }

    // 按锚点把每个版本切分成 anchors.len()+1 个段
    // 段 i = 锚点[i-1] 与锚点[i] 之间的非锚点字段（段 0 = 前缀，段 n = 后缀）
    let per_version_segs: Vec<Vec<Vec<String>>> = class
        .version_layouts
        .iter()
        .map(|l| {
            split_by_anchors(
                &l.fields
                    .iter()
                    .map(|f| f.rust_name.clone())
                    .collect::<Vec<_>>(),
                &anchors,
            )
        })
        .collect();

    let n_slots = anchors.len() + 1;
    for slot in 0..n_slots {
        // 此 slot 在各版本的字段列表（按版本升序，带 since）
        let slot_entries: Vec<((u16, u16, u16), Vec<String>)> = class
            .version_layouts
            .iter()
            .zip(&per_version_segs)
            .map(|(l, segs)| (l.since, segs[slot].clone()))
            .collect();

        emit_slot_segment(
            out,
            &class.name,
            &slot_entries,
            &own_map,
            &version_field_maps,
        );

        // 锚点（slot 之后、下一个 slot 之前）线性读
        // 锚点在所有版本中都有且相对顺序一致，用 own_map 的 align_after 即可。
        if slot < anchors.len() {
            let fname = &anchors[slot];
            set_field_none(out, &class.name, fname, &own_map, None);
        }
    }
}

/// (起始版本, 该版本此段字段名列表)
type SlotEntry = ((u16, u16, u16), Vec<String>);
/// 去重后的段变体：(起始版本, 字段名列表引用)
type SlotVariant<'a> = ((u16, u16, u16), &'a Vec<String>);

/// 段发射：此 slot 的字段列表在各版本间若全部相同 → 无条件线性读；
/// 否则 → version-gated（仅在该段变化的版本点分支）。
///
/// **field 属性（align_after 等）用 per-version layout 的 EmitField**，不用 merged
/// own_fields：同一个字段名在不同版本可能有不同的 align_after（如 CircleCollider2D
/// 的 m_used_by_effector 在 v5.6 无 align，在 v2023.1 有 align）。
fn emit_slot_segment(
    out: &mut String,
    class_name: &str,
    slot_entries: &[SlotEntry],
    own_map: &BTreeMap<&str, &EmitField>,
    version_field_maps: &BTreeMap<(u16, u16, u16), BTreeMap<&str, &EmitField>>,
) {
    // 升序去重连续相同字段列表 → 得到若干「段变体」(since, fields)
    let mut distinct: Vec<SlotVariant> = Vec::new();
    for (since, fields) in slot_entries {
        if distinct.last().is_some_and(|(_, f)| f == &fields) {
            continue;
        }
        distinct.push((*since, fields));
    }

    if distinct.len() <= 1 {
        // 稳定段：无条件读。用第一个版本的 field map 取 align。
        if let Some((since, fields)) = distinct.first() {
            let vmap = version_field_maps.get(since);
            for fname in fields.iter() {
                set_field_none(out, class_name, fname, own_map, vmap);
            }
        }
        return;
    }

    // 变化段：从最新版本开始 if/else（distinct 升序，倒序发射）
    let n = distinct.len();
    for (i, (since, fields)) in distinct.iter().rev().enumerate() {
        let (maj, min, build) = since;
        if i == 0 {
            out.push_str(&format!(
                "        if *ver >= crate::UnityVersion::new({maj}, {min}, {build}) {{\n"
            ));
        } else if i < n - 1 {
            out.push_str(&format!(
                "        }} else if *ver >= crate::UnityVersion::new({maj}, {min}, {build}) {{\n"
            ));
        } else {
            out.push_str("        } else {\n");
        }
        let vmap = version_field_maps.get(since);
        for fname in fields.iter() {
            set_field_none(out, class_name, fname, own_map, vmap);
        }
    }
    out.push_str("        }\n");
}

/// 退化路径：无公共锚点时，每版本分支读全部字段。
fn emit_full_versioned_branches(
    out: &mut String,
    class: &EmitClass,
    own_map: &BTreeMap<&str, &EmitField>,
    version_field_maps: &BTreeMap<(u16, u16, u16), BTreeMap<&str, &EmitField>>,
) {
    let n = class.version_layouts.len();
    for (i, layout) in class.version_layouts.iter().rev().enumerate() {
        let (maj, min, build) = layout.since;
        if i == 0 {
            out.push_str(&format!(
                "        if *ver >= crate::UnityVersion::new({maj}, {min}, {build}) {{\n"
            ));
        } else if i < n - 1 {
            out.push_str(&format!(
                "        }} else if *ver >= crate::UnityVersion::new({maj}, {min}, {build}) {{\n"
            ));
        } else {
            out.push_str("        } else {\n");
        }
        let vmap = version_field_maps.get(&layout.since);
        for lf in &layout.fields {
            set_field_none(out, &class.name, &lf.rust_name, own_map, vmap);
        }
    }
    out.push_str("        }\n");
}

/// 读取字段并写入对应的 None 变量：`name = Some(read_expr?);` + 对齐。
///
/// **type 来自 own_map**（与 struct 字段类型一致，避免版本类型不匹配）。
/// **align_after 优先取 version_map**（per-version 的对齐可能不同），找不到时回退 own_map。
///
/// `class_name` 用于单字段值依赖特判（目前仅 `AudioClip.m_audio_data`）。
fn set_field_none(
    out: &mut String,
    class_name: &str,
    fname: &str,
    own_map: &BTreeMap<&str, &EmitField>,
    version_map: Option<&BTreeMap<&str, &EmitField>>,
) {
    let Some(ef) = own_map.get(fname) else { return };

    // ── 值依赖特判：pre-5.0 AudioClip.m_AudioData ──
    // 对齐 AssetRipper `Pass505_FixOldAudioClip` + `AudioClipHelper.ReadOldByteArray`：
    // m_Stream==2（Streaming）时磁盘是 `u32 size; u32 offset;` 指向 <collection>.resS，
    // 而非内联 `i32 count + bytes`。此分支依赖宿主此前已读入的局部 `m_stream`（Option<i32>）。
    // 仅 pre-5.0 分支有 m_audio_data，故只在那个 else 分支命中。
    if class_name == "AudioClip" && fname == "m_audio_data" {
        out.push_str("            m_audio_data = Some(if m_stream == Some(2) {\n");
        out.push_str(
            "                // Streaming：u32 size; u32 offset; 指向 <collection>.resS\n",
        );
        out.push_str("                let size = rdr.read_u32()?;\n");
        out.push_str("                let offset = rdr.read_u32()?;\n");
        out.push_str("                crate::classes::audio_clip::AudioClipMAudioData::Streamed { size, offset }\n");
        out.push_str("            } else {\n");
        out.push_str("                let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;\n");
        out.push_str(
            "                crate::classes::audio_clip::AudioClipMAudioData::Inline(v)\n",
        );
        out.push_str("            });\n");
        out.push_str("            rdr.align(4)?;\n");
        return;
    }

    let expr = field_read_expr(&ef.rust_type);
    out.push_str(&format!("            {fname} = Some({expr}?);\n"));
    // VersionedEnum 自己处理 align，宿主 class 不再额外对齐（§12.5）。
    if type_handles_own_align(&ef.rust_type) {
        return;
    }
    let align = version_map
        .and_then(|m| m.get(fname))
        .map(|f| f.align_after)
        .unwrap_or(ef.align_after);
    if align {
        out.push_str("            rdr.align(4)?;\n");
    }
}

/// 多序列公共子序列（LCS）：所有版本字段序列都保持相对顺序的字段，即「锚点」。
fn compute_anchors(orders: &[Vec<String>]) -> Vec<String> {
    if orders.is_empty() {
        return Vec::new();
    }
    let mut acc = orders[0].clone();
    for v in &orders[1..] {
        acc = lcs_two(&acc, v);
        if acc.is_empty() {
            return Vec::new();
        }
    }
    acc
}

/// 两条字段名序列的最长公共子序列（标准 DP）。
fn lcs_two(a: &[String], b: &[String]) -> Vec<String> {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    let (mut i, mut j, mut out) = (n, m, Vec::new());
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            out.push(a[i - 1].clone());
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    out.reverse();
    out
}

/// 按锚点把字段列表切成 `anchors.len()+1` 段。
/// 返回段数组：段[0]=前缀(锚点0前)，段[k]=锚点k-1与k之间，... 段[n]=后缀。
fn split_by_anchors(fields: &[String], anchors: &[String]) -> Vec<Vec<String>> {
    let mut slots = vec![Vec::new(); anchors.len() + 1];
    let mut cur_slot = 0usize; // 当前在第几个段
    let mut next_anchor = 0usize; // 下一个待匹配的锚点下标
    for f in fields {
        if next_anchor < anchors.len() && f == &anchors[next_anchor] {
            // 命中锚点：之后的字段进入下一段
            cur_slot = next_anchor + 1;
            next_anchor += 1;
        } else {
            slots[cur_slot].push(f.clone());
        }
    }
    slots
}

fn emit_field_parse(out: &mut String, f: &EmitField) {
    let gate = f.version_gate.clone().unwrap_or(VersionGate::Always);
    let is_gated = !matches!(gate, VersionGate::Always);
    // field_read_expr 返回 Result<T, R::Error> 表达式（不含外层 ?）
    let result_expr = field_read_expr(&f.rust_type);
    // VersionedEnum 自己处理 align（§12.5）
    let need_align = f.align_after && !type_handles_own_align(&f.rust_type);

    if is_gated {
        let cond = gate_to_rust_cond(&gate);
        if need_align {
            // align 在 if 内（仅当字段被读时才对齐）
            out.push_str(&format!(
                "        let {name} = if {cond} {{\n            let v = {expr}?;\n            rdr.align(4)?;\n            Some(v)\n        }} else {{\n            None\n        }};\n",
                name = f.rust_name,
                expr = result_expr,
            ));
        } else {
            out.push_str(&format!(
                "        let {name} = if {cond} {{\n            Some({expr}?)\n        }} else {{\n            None\n        }};\n",
                name = f.rust_name,
                expr = result_expr,
            ));
        }
    } else {
        out.push_str(&format!(
            "        let {} = {}?;\n",
            f.rust_name, result_expr
        ));
        if need_align {
            out.push_str("        rdr.align(4)?;\n");
        }
    }
}

/// 字段读取表达式。**返回 `Result<T, R::Error>`，不含外层 `?`**。
///
/// 调用方负责加 `?`（如 `let x = {expr}?;`）。这样能在 closure 里用：
/// `(0..n).map(|_| {expr}).collect::<Result<Vec<_>, _>>()`。
fn field_read_expr(t: &RustType) -> String {
    match t {
        RustType::Bool => "rdr.read_bool()".into(),
        RustType::I8 => "rdr.read_i8()".into(),
        RustType::U8 => "rdr.read_u8()".into(),
        RustType::I16 => "rdr.read_i16()".into(),
        RustType::U16 => "rdr.read_u16()".into(),
        RustType::I32 => "rdr.read_i32()".into(),
        RustType::U32 => "rdr.read_u32()".into(),
        RustType::I64 => "rdr.read_i64()".into(),
        RustType::U64 => "rdr.read_u64()".into(),
        RustType::F32 => "rdr.read_f32()".into(),
        RustType::F64 => "rdr.read_f64()".into(),
        RustType::String => "rdr.read_aligned_string()".into(),
        RustType::Bytes => "( { let n = rdr.read_i32()? as usize; rdr.read_bytes(n) } )".into(),
        RustType::List(elem) => {
            let inner = field_read_expr(elem);
            format!(
                "( {{ let n = rdr.read_i32()? as usize; (0..n).map(|_| {{ {inner} }}).collect::<Result<Vec<_>, _>>() }} )"
            )
        }
        RustType::Pair(a, b) => {
            let a_inner = field_read_expr(a);
            let b_inner = field_read_expr(b);
            format!("( {{ let a = {a_inner}?; let b = {b_inner}?; Ok((a, b)) }} )")
        }
        RustType::Pptr { .. } => "crate::PPtr::parse(rdr, ver)".into(),
        RustType::Subclass(name) => format!("{name}::parse(rdr, ver)"),
        RustType::Option(inner) => field_read_expr(inner),
        RustType::VersionedEnum { name, .. } => format!("{name}::parse(rdr, ver)"),
        RustType::Box(inner) => {
            let inner_expr = field_read_expr(inner);
            format!("( {{ Ok(Box::new({inner_expr}?)) }} )")
        }
    }
}

fn gate_to_rust_cond(gate: &VersionGate) -> String {
    match gate {
        VersionGate::Always => "true".into(),
        VersionGate::Since(maj, min, build) => {
            format!("*ver >= crate::UnityVersion::new({maj}, {min}, {build})")
        }
        VersionGate::Range {
            from: (m1, n1, b1),
            to: (m2, n2, b2),
        } => {
            format!(
                "*ver >= crate::UnityVersion::new({m1}, {n1}, {b1}) && *ver < crate::UnityVersion::new({m2}, {n2}, {b2})"
            )
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// 模块文件（mod.rs）
// ────────────────────────────────────────────────────────────────────

fn emit_mod_rs(classes: &[&EmitClass]) -> String {
    let mut s = String::new();
    s.push_str("//! 自动生成。\n\n");
    for c in classes {
        s.push_str(&format!("pub mod {};\n", module_name(&c.name)));
    }
    s
}

// ────────────────────────────────────────────────────────────────────
// lib.rs / unity_object.rs / class_id.rs
// ────────────────────────────────────────────────────────────────────

fn emit_lib_rs() -> String {
    r#"//! 自动生成的 Unity 类型与 parser。请勿手动修改。
//!
//! 由 `crates/codegen` 生成（详见 `docs/codegen.md`）。
//! 共享原语（UnityVersion / EndianReader / Parse / PPtr）来自 `unity-core` crate。
//!
//! 本 crate 全部为机器生成代码，下列 lint 是 codegen 的已知副产物
//! （多余括号 / 未用版本参数 / 版本门控默认值被覆盖 / 含数字的类型名），
//! 统一在 crate 根静音，避免淹没调用方的真实告警。
#![allow(unused_parens, unused_imports, unused_variables, unused_assignments, non_camel_case_types)]

pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;

// 从 unity-core re-export，使生成代码中的 `crate::UnityVersion`、`crate::Parse`、
// `crate::EndianReader`、`crate::PPtr` 等引用仍然解析正确。
pub use unity_core::{Endian, EndianReader, Parse, PPtr, UnityVersion};
"#
    .to_string()
}

fn emit_unity_object(main_classes: &[&EmitClass]) -> String {
    let mut s = String::new();
    s.push_str("//! 自动生成的 UnityObject 分派。\n\n");
    s.push_str("use crate::Parse;\n\n");
    s.push_str("#[derive(Debug, Clone)]\n");
    s.push_str("pub enum UnityObject {\n");
    for c in main_classes {
        s.push_str(&format!(
            "    {}(crate::classes::{}::{}),\n",
            c.name,
            module_name(&c.name),
            c.name
        ));
    }
    s.push_str("}\n\n");

    s.push_str("impl UnityObject {\n");
    s.push_str("    pub fn parse<R: crate::EndianReader>(\n");
    s.push_str("        class_id: crate::ClassIDType,\n");
    s.push_str("        rdr: &mut R,\n");
    s.push_str("        ver: &crate::UnityVersion,\n");
    s.push_str("    ) -> Result<Self, R::Error> {\n");
    s.push_str("        match class_id {\n");
    for c in main_classes {
        s.push_str(&format!(
            "            crate::ClassIDType::{} => Ok(Self::{}(crate::classes::{}::{}::parse(rdr, ver)?)),\n",
            c.name, c.name, module_name(&c.name), c.name
        ));
    }
    s.push_str("            _ => return Err(rdr.make_error(&format!(\"unsupported class_id: {:?}\", class_id))),\n");
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n");
    s
}

fn emit_class_id(main_classes: &[&EmitClass]) -> String {
    let mut s = String::new();
    s.push_str("//! 自动生成的 ClassIDType 枚举。\n\n");
    s.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    s.push_str("#[repr(i32)]\n");
    s.push_str("pub enum ClassIDType {\n");
    for c in main_classes {
        if let Some(id) = c.class_id() {
            s.push_str(&format!("    {} = {id},\n", c.name));
        }
    }
    s.push_str("    Unknown = -1,\n");
    s.push_str("}\n\n");

    // from_i32：只返回有 dispatch arm 的非抽象 main class。
    s.push_str("impl ClassIDType {\n");
    s.push_str("    pub fn from_i32(id: i32) -> Option<Self> {\n");
    s.push_str("        match id {\n");
    for c in main_classes {
        if let Some(id) = c.class_id() {
            s.push_str(&format!("            {id} => Some(Self::{}),\n", c.name));
        }
    }
    s.push_str("            _ => None,\n");
    s.push_str("        }\n");
    s.push_str("    }\n");
    s.push_str("}\n");
    s
}

// ────────────────────────────────────────────────────────────────────
// 辅助：snake_case 模块名
// ────────────────────────────────────────────────────────────────────

fn snake_case(s: &str) -> String {
    // 复用 names 模块的逻辑（type_name → module_name）
    // Texture2D → texture_2d
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && c.is_uppercase() {
            let prev = chars[i - 1];
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
            if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next_lower) {
                out.push('_');
            }
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

fn module_name(type_name: &str) -> String {
    // type_name 可能是 "Texture2D" 或 "PPtr<Component>"（罕见路径）；
    // 取裸名再 snake_case 化。
    let bare = to_rust_type_name(type_name);
    snake_case(&bare)
}
