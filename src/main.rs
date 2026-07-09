//! inspect CLI：浏览 Unity AssetBundle（.ab / .unity3d）。
//!
//! 用法：
//!   unity-asset-rs <path>                 # 列出 bundle 头 + 内部文件 + 对象表（= list）
//!   unity-asset-rs <path> list            # 同上
//!   unity-asset-rs <path> object <path_id># dump 单个对象的字段
//!
//! 依赖 `Environment` 聚合层（eager parse 全部对象 + collection 分组）。
//! debug 模式由 `Cargo.toml` 的 `profile.dev opt-level=1` 保证不爆栈。

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use generated::classes::texture2_d::Texture2DMNameVariant;

use unity_asset_rs::{
    load, ClassIDType, Environment, ParsedObject, UnityObject, Value,
};

#[derive(Parser)]
#[command(
    name = "unity-asset-rs",
    about = "inspect Unity AssetBundle (.ab) 文件"
)]
struct Cli {
    /// AssetBundle 路径。
    path: PathBuf,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// 列出 bundle 头信息 + 内部文件 + 对象表（无子命令时也是默认）。
    List,
    /// dump 单个对象的字段（typed 走 Debug，dynamic 走 TypeTree Value 树）。
    Object {
        /// 目标对象的 path_id（先 `list` 拿到）。path_id 常为负数，故允许 `-` 开头。
        #[arg(allow_hyphen_values = true)]
        path_id: i64,
    },
}

/// 在 `EnvironmentBuilder::bundle` move bundle 之前捕获的头信息。
struct BundleHeader {
    format_version: u32,
    unity_revision: String,
    /// (name, bytes.len, flags)。
    files: Vec<(String, usize, u32)>,
}

fn main() -> ExitCode {
    env_logger::init();
    let cli = Cli::parse();

    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("错误: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> Result<(), String> {
    let (header, env) = load_env(&cli.path)?;
    match cli.cmd.as_ref().unwrap_or(&Cmd::List) {
        Cmd::List => cmd_list(&header, &env),
        Cmd::Object { path_id } => cmd_object(&env, *path_id),
    }
}

/// 加载 bundle 并构建 Environment。返回头信息（在 move 前捕获）+ Environment。
///
/// 注意：`EnvironmentBuilder::bundle` 会 move bundle，必须在此之前提取
/// `format_version` / `unity_revision` / `files` 等字段。
fn load_env(path: &PathBuf) -> Result<(BundleHeader, Environment), String> {
    let bundle = load(path).map_err(|e| format!("加载 bundle 失败: {e}"))?;
    let header = BundleHeader {
        format_version: bundle.format_version,
        unity_revision: bundle.unity_revision.clone(),
        files: bundle
            .files
            .iter()
            .map(|f| (f.name.clone(), f.bytes.len(), f.flags))
            .collect(),
    };
    let env = Environment::builder().bundle(bundle).build();
    Ok((header, env))
}

// ── 子命令 ──

fn cmd_list(header: &BundleHeader, env: &Environment) -> Result<(), String> {
    println!(
        "bundle: format v{}, engine {}, {} internal files",
        header.format_version,
        header.unity_revision,
        header.files.len()
    );
    println!("\n内部文件:");
    for (name, size, flags) in &header.files {
        println!("  {name:<40} {size:>10} bytes  flags={flags}");
    }

    println!("\n对象:");
    for (col_idx, col) in env.collections().iter().enumerate() {
        println!(
            "\n== collection[{col_idx}] {} (unity={}, endian={:?}, {} assets) ==",
            col.name, col.unity_version, col.endian, col.assets.len()
        );
        // 收集本 collection 的对象并按 path_id 排序，便于人眼定位。
        let mut rows: Vec<(i64, &unity_asset_rs::EnvironmentObject)> = col
            .assets
            .iter()
            .filter_map(|(&pid, &obj_idx)| {
                env.objects().get(obj_idx).map(|o| (pid, o))
            })
            .collect();
        rows.sort_by_key(|(pid, _)| *pid);
        for (pid, obj) in &rows {
            let class = class_name(obj.class_id);
            let name = obj_display_name(&obj.object);
            println!("  {pid:>20}  {class:<22}  {name}");
        }
    }

    let failures = env.failures();
    if !failures.is_empty() {
        println!("\n解析失败（{} 条）:", failures.len());
        for f in failures {
            match f {
                unity_asset_rs::ParseFailure::File { name, error } => {
                    println!("  [file] {name}: {error}");
                }
                unity_asset_rs::ParseFailure::Object {
                    collection,
                    path_id,
                    class_id,
                    error,
                } => {
                    println!(
                        "  [object] col={collection} pid={path_id} cid={class_id} ({}): {error}",
                        class_name(*class_id)
                    );
                }
            }
        }
    }

    Ok(())
}

fn cmd_object(env: &Environment, path_id: i64) -> Result<(), String> {
    let obj = env
        .objects()
        .iter()
        .find(|o| o.path_id == path_id)
        .ok_or_else(|| format!("找不到 path_id={path_id} 的对象"))?;

    let class = class_name(obj.class_id);
    let name = obj_display_name(&obj.object);
    println!(
        "path_id={} [{}] {:?} (col={})",
        obj.path_id, class, name, obj.collection
    );

    match &obj.object {
        ParsedObject::Typed(uo) => {
            // typed 统一走 Debug（不逐类格式化）。
            println!("\n{uo:#?}");
        }
        ParsedObject::Dynamic(v) => {
            println!("\n(dynamic / TypeTree):");
            dump_value(v, 1);
        }
        ParsedObject::MonoBehaviour(mb) => {
            println!("\n(mono behaviour):");
            println!("  m_GameObject: {:?}", mb.m_game_object);
            println!("  m_Enabled: {}", mb.m_enabled);
            println!("  m_Script: {:?}", mb.m_script);
            println!("  m_Name: {:?}", mb.m_name);
            if let Some(eci) = &mb.m_editor_class_identifier {
                println!("  m_EditorClassIdentifier: {eci:?}");
            }
            println!("  structure:");
            dump_value(&mb.structure, 2);
        }
    }
    Ok(())
}

// ── 辅助函数 ──

/// class_id -> 人类可读名（`ClassIDType` 无 Display，用 Debug；unknown id 原样显示）。
///
/// 注：codegen 跳过了 classID=114（MonoBehaviour），`ClassIDType::from_i32` 不认识它，
/// 这里单独映射成 `MonoBehaviour`。
fn class_name(class_id: i32) -> String {
    if class_id == 114 {
        return "MonoBehaviour".to_string();
    }
    ClassIDType::from_i32(class_id)
        .map(|c| format!("{c:?}"))
        .unwrap_or_else(|| format!("ClassID({class_id})"))
}

/// 从对象取展示名。
///
/// - typed：覆盖常见类的 `m_name` 字段（Texture2D 走版本变体枚举，其余 String）。
///   未匹配的类型返回空串（不强求全覆盖）。
/// - dynamic：查 `Value::Struct` 里的 `"m_Name"` 键（typetree/common_string.rs 的
///   interned 约定，PascalCase）。
/// - monobehaviour：直接取 typed shell 的 `m_name`。
fn obj_display_name(obj: &ParsedObject) -> String {
    match obj {
        ParsedObject::Typed(uo) => typed_name(uo).unwrap_or_default(),
        ParsedObject::Dynamic(v) => dynamic_name(v),
        ParsedObject::MonoBehaviour(mb) => mb.m_name.clone(),
    }
}

/// 常见 typed 类的 m_name 提取；未覆盖返回 None（该列留空）。
///
/// 仅覆盖「有 m_name 字段」的常见类——其余类型不强求（list 表里该列留空）。
fn typed_name(uo: &UnityObject) -> Option<String> {
    let s: String = match uo {
        // m_name: Texture2DMNameVariant（版本变体枚举，统一取内层 String）
        UnityObject::Texture2D(t) => texture_name(&t.m_name)?,
        // 其余常见类 m_name 直接是 String
        UnityObject::GameObject(o) => o.m_name.clone(),
        UnityObject::Mesh(o) => o.m_name.clone(),
        UnityObject::Material(o) => o.m_name.clone(),
        UnityObject::Shader(o) => o.m_name.clone(),
        UnityObject::AudioClip(o) => o.m_name.clone(),
        UnityObject::TextAsset(o) => o.m_name.clone(),
        UnityObject::Sprite(o) => o.m_name.clone(),
        UnityObject::Font(o) => o.m_name.clone(),
        UnityObject::AnimationClip(o) => o.m_name.clone(),
        _ => return None,
    };
    Some(s)
}

/// 从 `Texture2DMNameVariant` 取内层 String（三个变体内层都是 String）。
fn texture_name(v: &Texture2DMNameVariant) -> Option<String> {
    match v {
        Texture2DMNameVariant::Default(s)
        | Texture2DMNameVariant::V4_2(s)
        | Texture2DMNameVariant::V5_0(s) => Some(s.clone()),
    }
}

/// dynamic Value 树里的 "m_Name" 字符串提取。
fn dynamic_name(v: &Value) -> String {
    if let Value::Struct(fields) = v {
        for (k, val) in fields {
            if k == "m_Name" && let Value::String(s) = val {
                return s.clone();
            }
        }
    }
    String::new()
}

/// 递归打印 dynamic Value 树（带缩进）。bytes/string 截断到 80 字符。
fn dump_value(v: &Value, indent: usize) {
    let pad = "  ".repeat(indent);
    match v {
        Value::Struct(fields) => {
            for (k, val) in fields {
                match val {
                    Value::Struct(inner) => {
                        println!("{pad}{k}: struct ({} fields)", inner.len());
                    }
                    Value::Array(items) => {
                        println!("{pad}{k}: array[{}]", items.len());
                    }
                    Value::Bytes(b) => {
                        println!("{pad}{k}: bytes[{}]", b.len());
                    }
                    Value::String(s) => {
                        println!("{pad}{k}: {:?}", truncate_str(s, 80));
                    }
                    other => {
                        println!("{pad}{k}: {other:?}");
                    }
                }
            }
        }
        other => {
            println!("{pad}{other:?}");
        }
    }
}

/// 截断字符串到 max_chars（超出加省略号）。
fn truncate_str(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        format!("{}…", s.chars().take(max_chars).collect::<String>())
    } else {
        s.to_string()
    }
}
