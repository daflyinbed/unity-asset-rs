//! p07：代码发射。
//!
//! 把 `Pipeline` 转成多个 Rust 源文件，写到指定目录。
//! 输出布局（详见 `docs/codegen-design.md` §2.5）：
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

use std::collections::BTreeSet;
use std::path::Path;

use crate::ir::{
    EmitClass, EmitField, ParentSpec, Pipeline, RustType, VersionGate,
};
use crate::names::to_rust_type_name;

/// 把整个 Pipeline emit 到 `out_dir`。
pub fn emit_all(state: &Pipeline, out_dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir.join("classes"))?;
    std::fs::create_dir_all(out_dir.join("subclasses"))?;

    // 按需 emit 父类 enum 定义；只有当某 main class 是 Variable 时才需要。
    // 同时记录父类 enum 已写哪些 class（避免重复）。
    let main_classes: Vec<&EmitClass> = state
        .classes
        .values()
        .filter(|c| c.is_main() && !c.is_abstract)
        .collect();
    let all_emit: Vec<&EmitClass> = state.classes.values().collect();

    // 每个主 class / subclass 一个文件
    for class in &all_emit {
        let sub = class_matches_subclass(class);
        let dir = if sub { "subclasses" } else { "classes" };
        let file_name = format!("{}.rs", snake_case(&class.name));
        let path = out_dir.join(dir).join(file_name);
        let src = emit_class_file(class);
        std::fs::write(path, src)?;
    }

    // classes/mod.rs 和 subclasses/mod.rs
    let classes_mod = emit_mod_rs(
        &state
            .classes
            .values()
            .filter(|c| c.is_main())
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

fn emit_class_file(class: &EmitClass) -> String {
    let mut out = String::new();
    out.push_str("//! 自动生成。请勿手动修改。\n\n");

    // 收集 imports
    let imports = collect_imports(class);
    if !imports.is_empty() {
        out.push_str(&imports);
        out.push('\n');
    }

    // 父类 enum（如果是 Variable）
    if let ParentSpec::Variable {
        enum_name,
        branches,
    } = &class.parent
    {
        emit_parent_enum(&mut out, enum_name, branches);
        out.push('\n');
    }

    // struct 定义
    emit_struct(&mut out, class);

    // impl Parse
    emit_impl_parse(&mut out, class);

    out
}

fn collect_imports(class: &EmitClass) -> String {
    let mut imports = BTreeSet::new();
    for f in &class.own_fields {
        collect_type_imports(&f.rust_type, &mut imports);
    }
    // 父类引用
    match &class.parent {
        ParentSpec::Stable(name) => {
            imports.insert(format!("super::{}::{name}", module_name(name)));
        }
        ParentSpec::Variable { branches, .. } => {
            for (_, name) in branches {
                imports.insert(format!("super::{}::{name}", module_name(name)));
            }
        }
        ParentSpec::Root => {}
    }

    let mut s = String::new();
    for imp in imports {
        s.push_str(&format!("use {imp};\n"));
    }
    s
}

fn collect_type_imports(t: &RustType, out: &mut BTreeSet<String>) {
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
        RustType::List(t) => collect_type_imports(t, out),
        RustType::Pair(a, b) => {
            collect_type_imports(a, out);
            collect_type_imports(b, out);
        }
        RustType::Pptr { target_class: _ } => {
            // MVP: PPtr 非 generic，无需 import target class
            out.insert("crate::PPtr".into());
        }
        RustType::Subclass(name) => {
            out.insert(format!("crate::subclasses::{}::{name}", module_name(name)));
        }
        RustType::Option(inner) => {
            // gated 字段：递归到 inner
            collect_type_imports(inner, out);
        }
    }
}

fn emit_parent_enum(out: &mut String, enum_name: &str, branches: &[(VersionGate, String)]) {
    // 去重（同名父类只保留首个分支，因 enum 不能有重复 variant）
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let unique: Vec<&(VersionGate, String)> = branches
        .iter()
        .filter(|(_, name)| seen.insert(name.clone()))
        .collect();

    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub enum {enum_name} {{\n"));
    for (_, name) in &unique {
        out.push_str(&format!("    {name}({name}),\n"));
    }
    out.push_str("}\n");
}

fn emit_struct(out: &mut String, class: &EmitClass) {
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str(&format!("pub struct {} {{\n", class.name));

    // parent field
    match &class.parent {
        ParentSpec::Root => {}
        ParentSpec::Stable(name) => {
            out.push_str(&format!("    pub parent: {name},\n"));
        }
        ParentSpec::Variable { enum_name, .. } => {
            out.push_str(&format!("    pub parent: {enum_name},\n"));
        }
    }

    // own fields
    for f in &class.own_fields {
        let ty_str = f.rust_type.to_rust_src();
        out.push_str(&format!("    pub {}: {},\n", f.rust_name, ty_str));
    }
    out.push_str("}\n");
}

fn emit_impl_parse(out: &mut String, class: &EmitClass) {
    out.push_str(&format!(
        "\nimpl crate::Parse for {} {{\n",
        class.name
    ));
    out.push_str(
        "    fn parse<R: crate::EndianReader>(\n        r: &mut R,\n        ver: &crate::UnityVersion,\n    ) -> Result<Self, R::Error> {\n",
    );

    // MonoBehaviour：父类照常 + token: Vec<u8> 兜底（特殊处理）
    if class.is_mono_behaviour {
        out.push_str("        // MonoBehaviour 用户字段不在 TPK 中，token 保留原字节供运行时解释。\n");
    }

    // 解 parent
    match &class.parent {
        ParentSpec::Root => {}
        ParentSpec::Stable(name) => {
            out.push_str(&format!(
                "        let parent = {name}::parse(r, ver)?;\n"
            ));
        }
        ParentSpec::Variable {
            enum_name,
            branches,
        } => {
            // if/else chain
            for (i, (gate, name)) in branches.iter().enumerate() {
                let cond = gate_to_rust_cond(gate);
                if i == 0 {
                    out.push_str(&format!(
                        "        let parent = if {cond} {{\n            {enum_name}::{name}({name}::parse(r, ver)?)\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "        }} else if {cond} {{\n            {enum_name}::{name}({name}::parse(r, ver)?)\n"
                    ));
                }
            }
            // Fallback：理论上不会触发（branches 应覆盖所有版本），保险起见 panic。
            out.push_str("        } else {\n            panic!(\"无匹配的父类分支：version = {:?}\", ver)\n        };\n");
        }
    }

    // 解 own fields
    for f in &class.own_fields {
        emit_field_parse(out, f);
    }

    // 构造 Self
    out.push_str(&format!("\n        Ok({} {{\n", class.name));
    if !matches!(class.parent, ParentSpec::Root) {
        out.push_str("            parent,\n");
    }
    for f in &class.own_fields {
        out.push_str(&format!("            {},\n", f.rust_name));
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n");
}

fn emit_field_parse(out: &mut String, f: &EmitField) {
    let gate = f.version_gate.clone().unwrap_or(VersionGate::Always);
    let is_gated = !matches!(gate, VersionGate::Always);
    // field_read_expr 返回 Result<T, R::Error> 表达式（不含外层 ?）
    let result_expr = field_read_expr(&f.rust_type);

    if is_gated {
        let cond = gate_to_rust_cond(&gate);
        if f.align_after {
            // align 在 if 内（仅当字段被读时才对齐）
            out.push_str(&format!(
                "        let {name} = if {cond} {{\n            let v = {expr}?;\n            r.align(4)?;\n            Some(v)\n        }} else {{\n            None\n        }};\n",
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
        if f.align_after {
            out.push_str("        r.align(4)?;\n");
        }
    }
}

/// 字段读取表达式。**返回 `Result<T, R::Error>`，不含外层 `?`**。
///
/// 调用方负责加 `?`（如 `let x = {expr}?;`）。这样能在 closure 里用：
/// `(0..n).map(|_| {expr}).collect::<Result<Vec<_>, _>>()`。
fn field_read_expr(t: &RustType) -> String {
    match t {
        RustType::Bool => "r.read_bool()".into(),
        RustType::I8 => "r.read_i8()".into(),
        RustType::U8 => "r.read_u8()".into(),
        RustType::I16 => "r.read_i16()".into(),
        RustType::U16 => "r.read_u16()".into(),
        RustType::I32 => "r.read_i32()".into(),
        RustType::U32 => "r.read_u32()".into(),
        RustType::I64 => "r.read_i64()".into(),
        RustType::U64 => "r.read_u64()".into(),
        RustType::F32 => "r.read_f32()".into(),
        RustType::F64 => "r.read_f64()".into(),
        RustType::String => "r.read_aligned_string()".into(),
        RustType::Bytes => "( { let n = r.read_i32()? as usize; r.read_bytes(n) } )".into(),
        RustType::List(elem) => {
            let inner = field_read_expr(elem);
            format!(
                "( {{ let n = r.read_i32()? as usize; (0..n).map(|_| {{ {inner} }}).collect::<Result<Vec<_>, _>>() }} )"
            )
        }
        RustType::Pair(a, b) => {
            let a_inner = field_read_expr(a);
            let b_inner = field_read_expr(b);
            format!("( {{ let a = {a_inner}?; let b = {b_inner}?; Ok((a, b)) }} )")
        }
        RustType::Pptr { .. } => "crate::PPtr::parse(r, ver)".into(),
        RustType::Subclass(name) => format!("{name}::parse(r, ver)"),
        RustType::Option(inner) => field_read_expr(inner),
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
//! 由 `crates/codegen` 生成（详见 `docs/codegen-design.md`）。

pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;

/// 解析 trait。每个生成的 struct 实现它。
pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}

/// 字节序 reader 抽象。
/// 实际实现由 `crates/unity-asset` 提供（为现有 Cursor impl）。
pub trait EndianReader {
    type Error;
    fn read_bool(&mut self) -> Result<bool, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_i16(&mut self) -> Result<i16, Self::Error>;
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;
    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;
    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;
    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Self::Error>;
    fn read_aligned_string(&mut self) -> Result<String, Self::Error>;
    fn align(&mut self, n: usize) -> Result<(), Self::Error>;
}

/// Unity 版本号（生成代码里用到）。
/// 实际定义在 `crates/unity-asset-primitives`，但为保持 generated crate 自包含，
/// 这里复制定义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnityVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
}

impl UnityVersion {
    pub const fn new(major: u16, minor: u16, build: u16) -> Self {
        Self {
            major,
            minor,
            build,
        }
    }
}

/// PPtr<X> —— 跨引用指针。所有 PPtr 在磁盘上都是 m_FileID + m_PathID 两整数。
///
/// MVP 不带 generic：避免依赖 target class 存在（如 PrefabInstance 在某些 TPK 版本被改名）。
/// 用户拿不到类型区分（PPtr<GameObject> vs PPtr<Texture2D> 都变成 PPtr），但能编译。
/// 未来要做类型安全可改为 generic + PhantomData<fn() -> T>。
#[derive(Debug, Clone)]
pub struct PPtr {
    pub m_file_id: i32,
    pub m_path_id: i64,
}

impl PPtr {
    pub fn parse<R: EndianReader>(
        r: &mut R,
        _ver: &UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file_id = r.read_i32()?;
        // 简化：m_PathID 永远按 i64 读。
        // 老版本（ver < 5）实际是 i32，但生成器暂不做版本分支（D5 后改进）。
        let m_path_id = r.read_i64()?;
        Ok(PPtr {
            m_file_id,
            m_path_id,
        })
    }
}
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
    s.push_str("        r: &mut R,\n");
    s.push_str("        ver: &crate::UnityVersion,\n");
    s.push_str("    ) -> Result<Self, R::Error> {\n");
    s.push_str("        match class_id {\n");
    for c in main_classes {
        s.push_str(&format!(
            "            crate::ClassIDType::{} => Ok(Self::{}(crate::classes::{}::{}::parse(r, ver)?)),\n",
            c.name, c.name, module_name(&c.name), c.name
        ));
    }
    s.push_str("            _ => unimplemented!(\"未覆盖的 class_id: {:?}\", class_id),\n");
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
            if prev.is_lowercase() || prev.is_ascii_digit() || (prev.is_uppercase() && next_lower)
            {
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
