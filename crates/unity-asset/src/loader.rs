//! 声明式加载入口:`Loader`/`LoaderBuilder`/`LoadReport`/`Diagnostic`。
//!
//! 取代旧 [`crate::environment::EnvironmentBuilder`] 的关键变化:
//! - **声明式**:`path()`/`bytes()`/`scan_dir()` 只记录输入,真正工作放到 [`LoaderBuilder::build`]。
//! - **`build() -> Result<LoadReport>`**:顶层 IO/配置错误 → `Err`;单文件/单对象/fallback 等
//!   局部问题 → `LoadReport::diagnostics`(`Environment::failures` 仍是结构化细节层)。
//! - **`build_strict`**:任何 Error 级 diagnostic 都让 build 失败。
//! - 不再依赖 `log::warn!` 承载调用方必须知道的信息。
//!
//! # 示例
//!
//! ```no_run
//! # use unity_asset::{Loader, Registry};
//! let report = Loader::builder()
//!     .registry(&Registry::defaults())
//!     .path("main.ab")
//!     .path("shared.ab")
//!     .build()?; // IO 错误在此返回
//! let env = report.environment();
//! for d in report.diagnostics() {
//!     eprintln!("{d}");
//! }
//! # Ok::<(), unity_asset::Error>(())
//! ```

use std::path::{Path, PathBuf};
use std::sync::Arc;

use bytes::Bytes;

use crate::CollectionId;
use crate::decompress::Registry;
use crate::environment::{
    AssetCollection, DependencyProvider, Environment, EnvironmentObject, ParseFailure,
    ResourceFile, ResourceProvider, phase_a,
};
use crate::error::{Error, Result};
use crate::format::LoadedFile;
use crate::serialized_file::FileIdentifier;

/// 声明式加载器入口。由 [`Loader::builder`] 构造,经 [`LoaderBuilder::build`] 执行。
///
/// `path`/`bytes`/`scan_dir` 在 builder 阶段只记录输入;`build()` 时才统一跑
/// IO + 递归再派发 + Phase A。局部失败进入 [`LoadReport::diagnostics`],顶层
/// IO/配置错误以 `Err` 返回。
pub struct Loader {
    sources: Vec<Source>,
    registry: Registry,
    dependency_provider: Option<Arc<dyn DependencyProvider>>,
    resource_provider: Option<Arc<dyn ResourceProvider>>,
}

#[derive(Debug)]
enum Source {
    /// 磁盘文件路径(`fs::read` 时再读)。
    Path(PathBuf),
    /// 已读入内存的字节 + 建议名。
    Bytes { name: String, bytes: Bytes },
    /// 预解析 LoadedFile(高级用户用自定义 registry 预加载后直接灌入)。
    Loaded(LoadedFile),
    /// 递归扫描的目录。
    ScanDir(PathBuf),
}

/// [`Loader`] 的 builder。`build()` 消费 self 返回 [`LoadReport`]。
#[derive(Default)]
pub struct LoaderBuilder {
    sources: Vec<Source>,
    registry: Registry,
    dependency_provider: Option<Arc<dyn DependencyProvider>>,
    resource_provider: Option<Arc<dyn ResourceProvider>>,
}

impl LoaderBuilder {
    /// 设定解压器注册表(clone 入驻)。默认 [`Registry::defaults`]。
    ///
    /// 需要自定义 codec(如 Arknights LZ4 变体占 flag 4/5——见 ADR-0003)时在此注册。
    pub fn registry(mut self, reg: &Registry) -> Self {
        self.registry = reg.clone();
        self
    }

    /// 记录一个磁盘文件路径作为输入源(`build()` 时才 `fs::read` + 递归再派发)。
    pub fn path(mut self, path: impl AsRef<Path>) -> Self {
        self.sources.push(Source::Path(path.as_ref().to_path_buf()));
        self
    }

    /// 记录一段内存字节作为输入源。`name` 仅用于诊断/日志,不参与匹配。
    pub fn bytes(mut self, name: impl Into<String>, bytes: Bytes) -> Self {
        self.sources.push(Source::Bytes {
            name: name.into(),
            bytes,
        });
        self
    }

    /// 记录一个已解析的 [`LoadedFile`] 作为输入源(高级用户用自定义 registry 预加载后灌入)。
    pub fn loaded(mut self, loaded: LoadedFile) -> Self {
        self.sources.push(Source::Loaded(loaded));
        self
    }

    /// 记录一个递归扫描的目录作为输入源(`build()` 时遍历 + 嗅探加载)。
    pub fn scan_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.sources
            .push(Source::ScanDir(path.as_ref().to_path_buf()));
        self
    }

    /// 注册依赖提供者:外部引用在已加载 collection 未命中时回调。
    pub fn dependency_provider(mut self, provider: impl DependencyProvider + 'static) -> Self {
        self.dependency_provider = Some(Arc::new(provider));
        self
    }

    /// 注册资源提供者:`.resS` 未命中时回调。
    pub fn resource_provider(mut self, provider: impl ResourceProvider + 'static) -> Self {
        self.resource_provider = Some(Arc::new(provider));
        self
    }

    /// 执行加载。顶层 IO/配置错误 → `Err`;局部错误进入 [`LoadReport::diagnostics`]。
    pub fn build(self) -> Result<LoadReport> {
        Loader::from_builder(self).build()
    }

    /// 严格模式:任何 Error 级 diagnostic 都让 build 失败(Warning 级仍进入 report)。
    pub fn build_strict(self) -> Result<LoadReport> {
        let report = self.build()?;
        if report
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
        {
            let msg = report
                .diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .map(|d| d.message.clone())
                .collect::<Vec<_>>()
                .join("; ");
            return Err(Error::SerializedFile {
                reason: format!("build_strict: {msg}"),
            });
        }
        Ok(report)
    }
}

impl Loader {
    /// 创建一个空白 builder(默认 registry、无输入源、无 provider)。
    pub fn builder() -> LoaderBuilder {
        LoaderBuilder {
            sources: Vec::new(),
            registry: Registry::defaults(),
            dependency_provider: None,
            resource_provider: None,
        }
    }

    fn from_builder(b: LoaderBuilder) -> Self {
        Self {
            sources: b.sources,
            registry: b.registry,
            dependency_provider: b.dependency_provider,
            resource_provider: b.resource_provider,
        }
    }

    /// 真正执行加载。等价于 [`LoaderBuilder::build`]。
    pub fn build(self) -> Result<LoadReport> {
        let mut objects: Vec<EnvironmentObject> = Vec::new();
        let mut collections: Vec<AssetCollection> = Vec::new();
        let mut resources: Vec<ResourceFile> = Vec::new();
        let mut failures: Vec<ParseFailure> = Vec::new();
        let mut pending: Vec<(CollectionId, Vec<FileIdentifier>)> = Vec::new();
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let mut loaded_files: Vec<PathBuf> = Vec::new();

        for src in self.sources {
            match src {
                Source::Path(p) => {
                    let bytes = std::fs::read(&p).map_err(|e| Error::Io { source: e })?;
                    let loaded = crate::format::load_recursive(
                        Bytes::from(bytes),
                        &p.to_string_lossy(),
                        &self.registry,
                        None,
                    )?;
                    phase_a(
                        loaded,
                        &mut objects,
                        &mut collections,
                        &mut resources,
                        &mut failures,
                        &mut pending,
                    );
                    loaded_files.push(p);
                }
                Source::Bytes { name, bytes } => {
                    let loaded = crate::format::load_recursive(bytes, &name, &self.registry, None)?;
                    phase_a(
                        loaded,
                        &mut objects,
                        &mut collections,
                        &mut resources,
                        &mut failures,
                        &mut pending,
                    );
                }
                Source::Loaded(loaded) => {
                    phase_a(
                        loaded,
                        &mut objects,
                        &mut collections,
                        &mut resources,
                        &mut failures,
                        &mut pending,
                    );
                }
                Source::ScanDir(dir) => {
                    let mut any = false;
                    for entry in walk_files(&dir)? {
                        if crate::detect::looks_like_loadable_path(&entry)? {
                            let bytes =
                                std::fs::read(&entry).map_err(|e| Error::Io { source: e })?;
                            let loaded = crate::format::load_recursive(
                                Bytes::from(bytes),
                                &entry.to_string_lossy(),
                                &self.registry,
                                None,
                            )?;
                            phase_a(
                                loaded,
                                &mut objects,
                                &mut collections,
                                &mut resources,
                                &mut failures,
                                &mut pending,
                            );
                            loaded_files.push(entry);
                            any = true;
                        } else {
                            diagnostics.push(Diagnostic::warning(format!(
                                "scan_dir: 跳过非可加载文件: {}",
                                entry.display()
                            )));
                        }
                    }
                    if !any {
                        diagnostics.push(Diagnostic::warning(format!(
                            "scan_dir: {} 下未发现任何可加载文件",
                            dir.display()
                        )));
                    }
                }
            }
        }

        // scan_dir 等非结构化警告先入 diagnostics(不依赖 failures)。
        let scan_diag_count = diagnostics.len();

        let collection_count = collections.len();
        let resource_count = resources.len();
        let failure_count = failures.len();
        let mut env = Environment {
            objects,
            collections,
            resources,
            failures,
            dependency_provider: self.dependency_provider,
            resource_provider: self.resource_provider,
            containers: crate::container::ContainerIndex::default(),
            collection_sources: vec![None; collection_count],
            resource_sources: vec![None; resource_count],
            failure_sources: vec![None; failure_count],
            source_ids: std::collections::HashSet::new(),
            next_source_id: 1,
        };
        env.resolve_dependencies(&mut pending);
        env.prefetch_resources();
        env.build_container_index();

        // 把所有 Environment.failures(Phase A + Phase B 的结构化失败)投影成 Diagnostic,
        // 追加在 scan_dir 等非结构化诊断之后。
        let failure_diag: Vec<Diagnostic> =
            env.failures.iter().map(Diagnostic::from_failure).collect();
        diagnostics.splice(scan_diag_count..scan_diag_count, failure_diag);

        Ok(LoadReport {
            environment: env,
            diagnostics,
            loaded_files,
        })
    }
}

/// 一次 `build()` 的结果:已构建的 [`Environment`] + 构建期诊断。
pub struct LoadReport {
    environment: Environment,
    diagnostics: Vec<Diagnostic>,
    loaded_files: Vec<PathBuf>,
}

impl LoadReport {
    /// 已构建的环境。等价于消费 report 后的 [`LoadReport::into_environment`]。
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// 全部诊断(按生成顺序:scan_dir 跳过 → ParseFailure 投影 → provider 错误)。
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// 本次 Loader 从磁盘实际接受并解析的文件路径。
    ///
    /// `scan_dir` 中被格式嗅探跳过的文件不在此列表；内存 bytes/loaded 输入也不在。
    pub fn loaded_files(&self) -> &[PathBuf] {
        &self.loaded_files
    }

    /// 是否存在任何 Error 级诊断。
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }

    /// 消费 report,取出 [`Environment`]。
    pub fn into_environment(self) -> Environment {
        self.environment
    }
}

/// 诊断严重级别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// 单文件/单对象/provider 等可恢复错误(记录但 build 不失败)。
    Error,
    /// scan_dir 跳过非可加载文件、空目录等不影响结果的信息。
    Warning,
}

/// 一条构建期诊断。对 [`Environment::failures`](`Environment::failures`) 的扁平化 +
/// scan_dir 等非结构化信息的统一载体。
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    /// 结构化来源(若来自 [`ParseFailure`])。
    pub kind: Option<DiagnosticKind>,
}

/// 诊断的分类,对应 [`ParseFailure`] 的三个变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// SerializedFile 整体解析失败。
    File,
    /// 单个对象解析失败。
    Object,
    /// io 层子容器递归失败 / provider 错误。
    Container,
}

impl Diagnostic {
    pub fn warning(message: String) -> Self {
        Self {
            severity: Severity::Warning,
            message,
            kind: None,
        }
    }
    pub fn error(message: String, kind: DiagnosticKind) -> Self {
        Self {
            severity: Severity::Error,
            message,
            kind: Some(kind),
        }
    }
    fn from_failure(f: &ParseFailure) -> Self {
        match f {
            ParseFailure::File { name, error } => {
                Self::error(format!("file 失败 {name}: {error}"), DiagnosticKind::File)
            }
            ParseFailure::Object {
                collection,
                path_id,
                class_id,
                error,
            } => Self::error(
                format!("object 失败 col={collection} pid={path_id} cid={class_id}: {error}"),
                DiagnosticKind::Object,
            ),
            ParseFailure::Container { name, error } => Self::error(
                format!("container 失败 {name}: {error}"),
                DiagnosticKind::Container,
            ),
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = match self.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARN",
        };
        write!(f, "[{tag}] {}", self.message)
    }
}

/// 递归收集目录下全部文件路径(先序)。符号链接子目录跳过(避免循环)。
fn walk_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk_into(root, &mut out)?;
    Ok(out)
}

fn walk_into(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).map_err(|e| Error::Io { source: e })? {
        let entry = entry.map_err(|e| Error::Io { source: e })?;
        let path = entry.path();
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if ft.is_file() {
            out.push(path);
        } else if ft.is_dir() {
            // 跳过符号链接目录,避免循环。普通目录递归。
            walk_into(&path, out)?;
        }
    }
    Ok(())
}
