//! inspect CLI：浏览 Unity AssetBundle（.ab / .unity3d）。
//!
//! 用法：
//!   unity-asset-rs [path]                 # 列出对象表 + 解析状态 + 汇总（= list）
//!   unity-asset-rs [path] list            # 同上
//!   unity-asset-rs <path> object <path_id># dump 单个对象的字段
//!
//! 不传 path 时默认遍历 `example/`；传目录时递归扫描全部 bundle。
//! list 末尾附解析汇总（typed / dynamic / monobehaviour / fail），有失败时退出码 1。
//!
//! 依赖 `Environment` 聚合层（eager parse 全部对象 + collection 分组）。
//! debug 模式由 `Cargo.toml` 的 `profile.dev opt-level=1` 保证不爆栈。

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use generated::classes::texture2_d::Texture2DMNameVariant;

mod server;

use unity_asset::{
    load_with, ClassId, CompressionType, ContainerEntry, ContainerKind, Decompressor, Environment,
    Error, LoadedFile, Loader, ObjectId, ParsedObject, ParseFailure, Registry, UnityVersion, Value,
};
use generated::{ClassIDType, UnityObject};

#[derive(Parser)]
#[command(
    name = "unity-asset-rs",
    about = "inspect Unity AssetBundle (.ab) 文件"
)]
struct Cli {
    /// AssetBundle 路径。省略时默认 `example/`；可为文件或目录。
    path: Option<PathBuf>,

    /// 明日方舟（Arknights）自定义 LZ4 变体（劫持 flag 4/5）。
    #[arg(long, alias = "ak")]
    arknights: bool,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// 列出对象表（含解析状态）+ 汇总统计（无子命令时也是默认）。
    List,
    /// dump 单个对象的字段（typed 走 Debug，dynamic 走 TypeTree Value 树）。
    Object {
        /// 目标对象的 path_id（先 `list` 拿到）。path_id 常为负数，故允许 `-` 开头。
        #[arg(allow_hyphen_values = true)]
        path_id: i64,
    },
    /// 导出 Texture2D / Sprite 为 PNG（支持 ASTC LDR / ETC1/ETC2/EAC / BCn/DXT / 未压缩格式）。
    Export {
        /// 输出目录（默认当前目录）。
        #[arg(long, short = 'o', default_value = ".")]
        out: PathBuf,
        /// 只导出指定 path_id（可重复）；省略则导出全部 Texture2D + Sprite。
        #[arg(long, allow_hyphen_values = true)]
        only: Vec<i64>,
        /// 不导出 Sprite（仅导 Texture2D）。
        #[arg(long)]
        no_sprites: bool,
        /// 只导出 Sprite（跳过 Texture2D）。
        #[arg(long)]
        only_sprites: bool,
    },
    /// 列出 container 索引(AssetBundle.m_Container + ResourceManager.m_Container 聚合)。
    ///
    /// 默认正向视图(path → asset);`--reverse` 反向(object → path)。
    /// multidict(同 path 多条)全部保留。
    Containers {
        /// 反向视图:按对象归类,显示 obj -> path(线性扫描;含 preload 派生)。
        #[arg(long)]
        reverse: bool,
        /// 按宿主类型过滤:`AssetBundle` / `ResourceManager`。
        #[arg(long)]
        kind: Option<String>,
    },
    /// 启动 HTTP 服务，把 list / object / export 能力通过 REST 接口暴露。
    ///
    /// bundle 在运行时按请求加载（POST /bundles），支持多 bundle 并存。
    /// 端点总览见 `src/server/mod.rs`。
    Serve {
        /// 监听地址，默认 127.0.0.1:8080。
        #[arg(long, short = 'a', default_value = "127.0.0.1:8080")]
        addr: std::net::SocketAddr,
        /// bundle 文件根目录（POST /bundles 的 `path` 相对它解析），默认当前目录。
        #[arg(long, short = 'd', default_value = ".")]
        root: PathBuf,
    },
    /// 输出 serve 使用的 OpenAPI JSON，供前端生成类型。
    Openapi {
        /// 写入文件；省略时输出到 stdout。
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
    },
}

/// 在 `EnvironmentBuilder::loaded` move 之前捕获的头信息（仅单文件时有值）。
struct BundleHeader {
    format_version: Option<u32>,
    unity_revision: Option<String>,
    /// (name, bytes.len)。
    files: Vec<(String, usize)>,
}

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    let cli = Cli::parse();

    match run(&cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("错误: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: &Cli) -> Result<(), String> {
    match cli.cmd.as_ref().unwrap_or(&Cmd::List) {
        // serve 走自己的加载路径（运行时按请求加载），不预加载 path。
        Cmd::Serve { addr, root } => crate::server::serve(*addr, root.clone()).await,
        Cmd::Openapi { output } => {
            let json = crate::server::openapi_json()?;
            if let Some(path) = output {
                std::fs::write(path, json)
                    .map_err(|e| format!("写 OpenAPI JSON {} 失败: {e}", path.display()))?;
            } else {
                println!("{json}");
            }
            Ok(())
        }
        // 其余子命令:启动时一次性加载 path（短任务，直接在当前 worker 跑）。
        Cmd::List | Cmd::Object { .. } | Cmd::Export { .. } | Cmd::Containers { .. } => {
            let path = cli.path.as_deref().unwrap_or_else(|| Path::new("example"));
            let (header, env) = build_env(path, cli.arknights)?;
            match cli.cmd.as_ref().unwrap() {
                Cmd::List => cmd_list(header.as_ref(), &env),
                Cmd::Object { path_id } => cmd_object(&env, *path_id),
                Cmd::Export {
                    out,
                    only,
                    no_sprites,
                    only_sprites,
                } => cmd_export(&env, out, only, *no_sprites, *only_sprites),
                Cmd::Containers { reverse, kind } => {
                    cmd_containers(&env, *reverse, kind.as_deref())
                }
                Cmd::Serve { .. } => unreachable!(),
                Cmd::Openapi { .. } => unreachable!(),
            }
        }
    }
}

/// 构建 registry（arknights=true 时注册 lz4inv 到 flag 4/5）。
pub(crate) fn make_registry(arknights: bool) -> Registry {
    if arknights {
        let mut r = Registry::defaults();
        let ak: Arc<dyn Decompressor> = Arc::new(AkLz4Decompressor);
        r.register(CompressionType(4), ak.clone());
        r.register(CompressionType(5), ak);
        r
    } else {
        Registry::defaults()
    }
}

/// 加载资产文件（单个或目录）并构建 Environment。
///
/// 单文件时返回 `Some(BundleHeader)`（format_version / unity_revision / 文件列表），
/// 目录扫描时返回 `None`（多 bundle 时 header 无意义）。
fn build_env(path: &Path, arknights: bool) -> Result<(Option<BundleHeader>, Environment), String> {
    let registry = make_registry(arknights);
    if path.is_dir() {
        let report = Loader::builder()
            .registry(&registry)
            .scan_dir(path)
            .build()
            .map_err(|e| format!("扫描目录失败: {e}"))?;
        // 扫描目录场景下 header 无意义;诊断打印到 stderr。
        for d in report.diagnostics() {
            eprintln!("{d}");
        }
        Ok((None, report.into_environment()))
    } else {
        let loaded = load_with(path, &registry).map_err(|e| format!("加载资产文件失败: {e}"))?;
        let header = capture_header(&loaded);
        let report = Loader::builder()
            .registry(&registry)
            .loaded(loaded)
            .build()
            .map_err(|e| format!("构建 Environment 失败: {e}"))?;
        Ok((Some(header), report.into_environment()))
    }
}

/// 从 `LoadedFile` 树提取展示用头信息（顶层 Container 的 unity_revision / format_version
/// + 全部叶子的 name/size）。非 Container 顶层（如松散 SerializedFile）则字段为 None。
fn capture_header(loaded: &LoadedFile) -> BundleHeader {
    let (format_version, unity_revision) = match loaded {
        LoadedFile::Container {
            unity_revision,
            format_version,
            ..
        } => (*format_version, unity_revision.clone()),
        LoadedFile::Leaf { unity_revision, .. } => (None, unity_revision.clone()),
        LoadedFile::Failed { unity_revision, .. } => (None, unity_revision.clone()),
    };
    let files = loaded
        .leaves()
        .into_iter()
        .map(|(name, bytes, _, _)| (name.to_string(), bytes.len()))
        .collect();
    BundleHeader {
        format_version,
        unity_revision,
        files,
    }
}

// ── 子命令 ──

/// 列出对象表（每行含 path_id / class / 解析方式 / 展示名）+ 末尾汇总。
/// 有解析失败时返回 Err（main 据此返回退出码 1）。
fn cmd_list(header: Option<&BundleHeader>, env: &Environment) -> Result<(), String> {
    // 单文件时显示 bundle header（format / revision / 内部文件列表）。
    if let Some(h) = header {
        let fv = h
            .format_version
            .map(|v| format!("v{v}"))
            .unwrap_or_else(|| "?".to_string());
        let ur = h
            .unity_revision
            .clone()
            .unwrap_or_else(|| "?".to_string());
        println!(
            "bundle: format {fv}, engine {ur}, {} internal files",
            h.files.len()
        );
        println!("\n内部文件:");
        for (name, size) in &h.files {
            println!("  {name:<40} {size:>10} bytes");
        }
    }

    println!("\n对象:");
    let mut typed = 0usize;
    let mut mono = 0usize;
    let mut dynamic = 0usize;

    for (col_idx, col) in env.collections().iter().enumerate() {
        println!(
            "\n== collection[{col_idx}] {} (unity={}, endian={:?}, {} assets) ==",
            col.name, col.unity_version, col.endian, col.assets.len()
        );
        // 收集本 collection 的对象并按 path_id 排序，便于人眼定位。
        let mut rows: Vec<(i64, &unity_asset::EnvironmentObject)> = col
            .assets
            .iter()
            .filter_map(|(&pid, &obj_idx)| env.objects().get(obj_idx).map(|o| (pid, o)))
            .collect();
        rows.sort_by_key(|(pid, _)| *pid);
        for (pid, obj) in &rows {
            let class = class_name(obj.class_id);
            let name = obj_display_name(&obj.object);
            let (tag, label) = parse_status(&obj.object);
            match label {
                "typed" => typed += 1,
                "mono" => mono += 1,
                "dynamic" => dynamic += 1,
                _ => {}
            }
            println!("  {pid:>20}  {class:<22}  {tag:<12}  {name}");
        }
    }

    let failures = env.failures();
    let fail_count = failures.len();
    if !failures.is_empty() {
        println!("\n解析失败（{fail_count} 条）:");
        for f in failures {
            match f {
                ParseFailure::File { name, error } => {
                    println!("  [file] {name}: {error}");
                }
                ParseFailure::Object {
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
                ParseFailure::Container { name, error } => {
                    println!("  [container] {name}: {error}");
                }
            }
        }
    }

    println!("\n========== 汇总 ==========");
    println!("typed 成功:     {typed}");
    println!("monobehaviour:  {mono}");
    println!("dynamic:        {dynamic}");
    println!("失败:           {fail_count}");

    if fail_count > 0 {
        Err(format!("{fail_count} 个对象/文件解析失败"))
    } else {
        Ok(())
    }
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

/// 列出 container 索引(AssetBundle.m_Container + ResourceManager.m_Container 聚合)。
///
/// 默认正向视图(path → asset);`--reverse` 切到反向视图(object → path)。
/// `--kind` 按宿主类型过滤(非法值报错退出)。multidict(同 path 多条)全部保留。
fn cmd_containers(
    env: &Environment,
    reverse: bool,
    kind: Option<&str>,
) -> Result<(), String> {
    // kind 字符串 → ContainerKind;非法值显式报错(避免静默返回空列表误导用户)。
    let kind_filter: Option<ContainerKind> = match kind {
        None => None,
        Some("AssetBundle") => Some(ContainerKind::AssetBundle),
        Some("ResourceManager") => Some(ContainerKind::ResourceManager),
        Some(other) => {
            return Err(format!(
                "非法 --kind {other:?},应为 \"AssetBundle\" 或 \"ResourceManager\""
            ));
        }
    };

    let entries: Vec<&ContainerEntry> = env
        .containers()
        .entries()
        .iter()
        .filter(|e| kind_filter.is_none_or(|k| e.kind == k))
        .collect();

    if entries.is_empty() {
        println!("(无 container 条目)");
        return Ok(());
    }

    if !reverse {
        // 正向视图:path → asset
        println!(
            "{:<12}  {:<50}  {:<24}  preload",
            "kind", "path", "asset"
        );
        println!("{}", "-".repeat(100));
        for e in &entries {
            let kind_str = kind_label(e.kind);
            let asset_str = match e.asset {
                Some(id) => format_ref(env, id),
                None => "(unresolved)".to_string(),
            };
            let preload_count = e
                .preload
                .as_ref()
                .map(|s| {
                    let total = s.len();
                    let resolved = s.iter().filter(|p| p.is_some()).count();
                    format!("{resolved}/{total}")
                })
                .unwrap_or_else(|| "-".to_string());
            println!(
                "{:<12}  {:<50}  {:<24}  {}",
                kind_str, e.path, asset_str, preload_count
            );
        }
        println!("\n共 {} 条", entries.len());
    } else {
        // 反向视图:object → path。
        // 收集每个对象的所有 path(multidict),含 preload 派生条目。
        // ObjectId 未 impl Ord,故用 HashMap 累积后转 Vec 按 (collection, path_id) 排序。
        // 单条 (path, kind, via_preload):via_preload=false 表示对象是 main asset。
        type RevRow = (String, ContainerKind, bool);
        let mut by_obj: std::collections::HashMap<ObjectId, Vec<RevRow>> =
            std::collections::HashMap::new();
        for e in &entries {
            if let Some(id) = e.asset {
                by_obj
                    .entry(id)
                    .or_default()
                    .push((e.path.clone(), e.kind, false));
            }
            if let Some(slice) = &e.preload {
                for id in slice.iter().flatten() {
                    by_obj
                        .entry(*id)
                        .or_default()
                        .push((e.path.clone(), e.kind, true));
                }
            }
        }
        // 按 (collection 下标, path_id) 排序输出,便于人眼定位。
        let mut flat: Vec<(ObjectId, Vec<RevRow>)> = by_obj.into_iter().collect();
        flat.sort_by_key(|(id, _)| (id.collection.to_index(), id.path_id));
        println!(
            "{:<24}  {:<12}  {:<8}  path",
            "object", "kind", "via"
        );
        println!("{}", "-".repeat(100));
        for (id, paths) in &flat {
            let obj_str = format_ref(env, *id);
            for (path, kind, via_preload) in paths {
                let kind_str = kind_label(*kind);
                let via = if *via_preload { "preload" } else { "main" };
                println!("{:<24}  {:<12}  {:<8}  {}", obj_str, kind_str, via, path);
            }
        }
        let total_refs: usize = flat.iter().map(|(_, v)| v.len()).sum();
        println!("\n共 {} 个对象, {} 条引用", flat.len(), total_refs);
    }
    Ok(())
}

/// container kind → 展示标签。
fn kind_label(k: ContainerKind) -> &'static str {
    match k {
        ContainerKind::AssetBundle => "AssetBundle",
        ContainerKind::ResourceManager => "ResourceMgr",
    }
}

/// ObjectId → 紧凑展示字符串(`<col>:<path_id> [ClassName]`)。
/// 解析失败的对象只显示 id(理论上不会发生,因 id 来自已 resolve 的引用)。
fn format_ref(env: &Environment, id: ObjectId) -> String {
    let class = env
        .resolve_ref(id)
        .and_then(|r| r.get())
        .map(|o| class_name(o.class_id))
        .unwrap_or_else(|| "?".to_string());
    format!("{}:{} [{}]", id.collection.to_index(), id.path_id, class)
}

/// 导出 Texture2D 为 PNG。
///
/// 遍历全部 Texture2D（或 `only` 指定的 path_id），对 ASTC LDR / ETC1/ETC2/EAC /
/// BCn/DXT / 未压缩格式解码 + 写 PNG；不支持的格式记 skip。随后遍历全部 Sprite
/// （受 `--no-sprites` / `--only-sprites` 开关控制），从其 atlas 上裁出 textureRect +
/// 可选 packing rotation / downscale，写 PNG。末尾汇总，有失败时返回 Err（退出码 1）。
#[cfg(feature = "export")]
fn cmd_export(
    env: &Environment,
    out: &Path,
    only: &[i64],
    no_sprites: bool,
    only_sprites: bool,
) -> Result<(), String> {
    use generated::classes::sprite::Sprite;
    use generated::classes::texture2_d::Texture2D;
    use unity_asset::sprite_export::export_sprite;
    use unity_asset::texture_export::TextureFormat;

    std::fs::create_dir_all(out).map_err(|e| format!("创建输出目录失败: {e}"))?;

    let mut tex_exported = 0usize;
    let mut tex_skipped = 0usize;
    let mut tex_failed = 0usize;

    if !only_sprites {
        for obj in env.objects_of_class(ClassIDType::Texture2D) {
            if !only.is_empty() && !only.contains(&obj.path_id) {
                continue;
            }
            let ParsedObject::Typed(UnityObject::Texture2D(t)) = &obj.object else {
                // 非 typed（dynamic fallback）的 Texture2D —— 跳过。
                tex_skipped += 1;
                continue;
            };
            let t: &Texture2D = t;
            let name =
                texture_name(&t.m_name).unwrap_or_else(|| format!("tex_{}", obj.path_id));

            let fmt = match TextureFormat::from_i32(t.m_format) {
                Some(f) if f.is_supported() => f,
                Some(f) => {
                    println!("⊘ skip  {name}  format={}({f:?}, 不支持)", t.m_format);
                    tex_skipped += 1;
                    continue;
                }
                None => {
                    println!("⊘ skip  {name}  format={}(未知)", t.m_format);
                    tex_skipped += 1;
                    continue;
                }
            };

            let result = export_one(
                env,
                t,
                &name,
                env.collections()[obj.collection.to_index()].unity_version,
                out,
            );
            match result {
                Ok(file) => {
                    println!(
                        "✓ ok    {name}  {}x{} {} -> {}",
                        t.m_width,
                        t.m_height,
                        fmt.label(),
                        file.display()
                    );
                    tex_exported += 1;
                }
                Err(e) => {
                    println!("✗ fail  {name}  {e}");
                    tex_failed += 1;
                }
            }
        }
    }

    println!(
        "纹理汇总: 导出 {tex_exported}, 跳过 {tex_skipped}, 失败 {tex_failed}",
    );

    let mut spr_exported = 0usize;
    let mut spr_skipped = 0usize;
    let mut spr_failed = 0usize;

    if !no_sprites {
        for obj in env.objects_of_class(ClassIDType::Sprite) {
            if !only.is_empty() && !only.contains(&obj.path_id) {
                continue;
            }
            let ParsedObject::Typed(UnityObject::Sprite(s)) = &obj.object else {
                // 非 typed（dynamic fallback）的 Sprite —— 跳过。
                spr_skipped += 1;
                continue;
            };
            let s: &Sprite = s;
            let name = if s.m_name.is_empty() {
                format!("sprite_{}", obj.path_id)
            } else {
                s.m_name.clone()
            };
            let ver = env.collections()[obj.collection.to_index()].unity_version;
            match export_sprite(env, obj, s, &name, ver, out) {
                Ok(file) => {
                    println!("✓ ok    {name}  sprite -> {}", file.display());
                    spr_exported += 1;
                }
                Err(e) => {
                    println!("✗ fail  {name}  {e}");
                    spr_failed += 1;
                }
            }
        }
    }

    println!(
        "Sprite 汇总: 导出 {spr_exported}, 跳过 {spr_skipped}, 失败 {spr_failed}",
    );

    let total_failed = tex_failed + spr_failed;
    if total_failed > 0 {
        Err(format!("有 {total_failed} 个导出失败"))
    } else {
        Ok(())
    }
}

/// 解码并写出单个 Texture2D 到 `<out>/<name>.png`,返回实际文件路径。
#[cfg(feature = "export")]
fn export_one(
    env: &Environment,
    t: &generated::classes::texture2_d::Texture2D,
    name: &str,
    ver: UnityVersion,
    out: &Path,
) -> Result<PathBuf, unity_asset::texture_export::TextureExportError> {
    // 统一调 export::decode_texture(消除与 sprite_export 的 5 路分派重复)。
    let img = unity_asset::export::decode_texture(t, env, ver)?;
    let file = out.join(sanitize_filename(name)).with_extension("png");
    img.write_png_path(&file)?;
    Ok(file)
}

/// 无 feature 时的占位（避免未使用 import 警告 / 给用户清晰提示）。
#[cfg(not(feature = "export"))]
fn cmd_export(
    _env: &Environment,
    _out: &Path,
    _only: &[i64],
    _no_sprites: bool,
    _only_sprites: bool,
) -> Result<(), String> {
    Err("export 子命令需要 `export` feature（默认开启；当前构建关闭了它）".to_string())
}

/// 把 m_name 里的路径非法字符（`[`, `]`, `/`, `\`, `:` 等）替换为 `_`。
pub(crate) fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '[' | ']' => '_',
            _ => c,
        })
        .collect()
}

// ── 明日方舟 LZ4 变体适配器 ──

/// 明日方舟自定义 LZ4 变体解压器（nibble 对调 + BE offset）。
///
/// 放在 CLI 层而非库里——遵守 AGENTS.md「不要在库里硬编码游戏特定 codec」。
pub(crate) struct AkLz4Decompressor;

impl Decompressor for AkLz4Decompressor {
    fn decompress(&self, input: &[u8], expected: usize) -> Result<Vec<u8>, Error> {
        let mut out = vec![0u8; expected];
        lz4inv::decompress(input, &mut out)
            .map_err(|e| Error::Decompress { reason: e.to_string() })?;
        Ok(out)
    }
}

// ── 辅助函数 ──

/// 解析方式 → (展示标签, 分类短名)。
pub(crate) fn parse_status(obj: &ParsedObject) -> (&'static str, &'static str) {
    match obj {
        ParsedObject::Typed(_) => ("✓ typed", "typed"),
        ParsedObject::MonoBehaviour(_) => ("✓ mono", "mono"),
        ParsedObject::Dynamic(_) => ("✓ dynamic", "dynamic"),
    }
}

/// class_id -> 人类可读名（`ClassIDType` 无 Display，用 Debug；unknown id 原样显示）。
///
/// 注：codegen 跳过了 classID=114（MonoBehaviour），`ClassIDType::from_i32` 不认识它，
/// 这里单独映射成 `MonoBehaviour`。
pub(crate) fn class_name(class_id: ClassId) -> String {
    if class_id == ClassId::MONO_BEHAVIOUR {
        return "MonoBehaviour".to_string();
    }
    class_id
        .known()
        .map(|c| format!("{c:?}"))
        .unwrap_or_else(|| format!("ClassID({})", class_id.get()))
}

/// 从对象取展示名。
///
/// - typed：覆盖常见类的 `m_name` 字段（Texture2D 走版本变体枚举，其余 String）。
///   未匹配的类型返回空串（不强求全覆盖）。
/// - dynamic：查 `Value::Struct` 里的 `"m_Name"` 键（typetree/common_string.rs 的
///   interned 约定，PascalCase）。
/// - monobehaviour：直接取 typed shell 的 `m_name`。
pub(crate) fn obj_display_name(obj: &ParsedObject) -> String {
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
