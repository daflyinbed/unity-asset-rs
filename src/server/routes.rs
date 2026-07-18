//! HTTP handler:加载 / 列表 / 详情 / 导出 PNG。
//!
//! 所有读操作走 `AppState::get_bundle`(读锁 → `Arc::clone` → 立即释放)。
//! 加载操作(`load_bundle`)是阻塞 IO + CPU 密集,必须 [`tokio::task::spawn_blocking`],
//! 避免 `Loader::build` 内部的 `std::fs::read` + eager parse 卡住 tokio worker。
//!
//! 每个 handler 上的 `#[utoipa::path]` 经 `OpenApiRouter::routes(routes!(...))` 收集进
//! OpenApi 文档(mod.rs)。路径参数用 `{bid}` 语法(axum 0.8 + utoipa 一致)。

use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use axum::extract::{Path as AxumPath, Query, State};
use axum::http::header;
use axum::Json;
use serde::{Deserialize, Serialize};
use unity_asset::{Environment, Loader};
use utoipa::IntoParams;

use super::dto::{is_image_class, ObjectDetail, ObjectDto, ObjectMeta};
use super::error::{ApiError, ApiErrorBody};
use super::state::AppState;

// ── 存活检查 ──

/// 存活检查,返回纯文本 `"ok"`。
#[utoipa::path(
    get,
    path = "/health",
    tag = "bundle",
    responses((status = 200, description = "服务存活", body = String, content_type = "text/plain"))
)]
pub async fn health() -> &'static str {
    "ok"
}

// ── bundle 列表 / 加载 / 详情 / 卸载 ──

/// 已加载 bundle 的摘要(出现在 [`list_bundles`] 列表与 [`get_bundle`] 详情)。
#[derive(Serialize, utoipa::ToSchema)]
pub struct BundleListItem {
    pub id: String,
    pub collections: usize,
    pub objects: usize,
    pub resources: usize,
    pub failures: usize,
}

/// 列出全部已加载 bundle。
#[utoipa::path(
    get,
    path = "/bundles",
    tag = "bundle",
    responses((status = 200, description = "已加载 bundle 列表", body = [BundleListItem]))
)]
pub async fn list_bundles(
    State(s): State<AppState>,
) -> Result<Json<Vec<BundleListItem>>, ApiError> {
    let lock = s.bundles.read();
    let mut out = Vec::with_capacity(lock.map.len());
    for (id, env) in lock.map.iter() {
        out.push(BundleListItem {
            id: id.clone(),
            collections: env.collections().len(),
            objects: env.objects().len(),
            resources: env.resources().len(),
            failures: env.failures().len(),
        });
    }
    Ok(Json(out))
}

/// POST /bundles 的 body。
#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoadBody {
    /// bundle 文件路径(相对 serve `--root` 或绝对)。
    pub path: String,
    /// bundle id(缺省用文件名 stem)。
    pub id: Option<String>,
    /// 明日方舟 LZ4 变体(劫持 flag 4/5)。
    #[serde(default)]
    pub arknights: bool,
}

/// [`load_bundle`] 的响应。
#[derive(Serialize, utoipa::ToSchema)]
pub struct LoadResponse {
    pub id: String,
    pub loaded: bool,
    pub collections: usize,
    pub objects: usize,
    pub failures: usize,
}

/// 加载 bundle 文件为 Environment(阻塞 IO + eager parse 全部对象)。
///
/// `path` 相对 serve 的 `--root` 解析,已做路径穿越防护。
#[utoipa::path(
    post,
    path = "/bundles",
    tag = "bundle",
    request_body = LoadBody,
    responses(
        (status = 201, description = "加载成功", body = LoadResponse),
        (status = 400, description = "路径不存在 / 超出根目录 / body 非法", body = ApiErrorBody),
        (status = 422, description = "build 失败", body = ApiErrorBody),
    )
)]
pub async fn load_bundle(
    State(s): State<AppState>,
    Json(body): Json<LoadBody>,
) -> Result<(axum::http::StatusCode, Json<LoadResponse>), ApiError> {
    let full = resolve_within_root(&s.root, &body.path)?;
    let registry = crate::make_registry(body.arknights);
    // 先从 full 提取默认 id(文件名 stem),再 move full 进闭包。
    let id = body
        .id
        .clone()
        .or_else(|| {
            full.file_stem()
                .and_then(|s| s.to_str().map(str::to_string))
        })
        .ok_or_else(|| ApiError::BadRequest("无法从 path 推断 id,请显式提供".into()))?;
    // 阻塞 IO + CPU 密集 → spawn_blocking。registry/full move 进闭包。
    let join = tokio::task::spawn_blocking(move || -> Result<Environment, String> {
        let report = Loader::builder()
            .registry(&registry)
            .path(&full)
            .build()
            .map_err(|e| format!("build 失败: {e}"))?;
        for d in report.diagnostics() {
            eprintln!("{d}");
        }
        Ok(report.into_environment())
    })
    .await
    .map_err(|e| format!("worker join 失败: {e}"))?;

    let env = join.map_err(ApiError::Load)?;
    let resp = LoadResponse {
        id: id.clone(),
        loaded: true,
        collections: env.collections().len(),
        objects: env.objects().len(),
        failures: env.failures().len(),
    };
    s.bundles.write().map.insert(id.clone(), Arc::new(env));
    Ok((axum::http::StatusCode::CREATED, Json(resp)))
}

/// 取单个 bundle 的摘要。
#[utoipa::path(
    get,
    path = "/bundles/{bid}",
    tag = "bundle",
    params(("bid" = String, Path, description = "bundle id")),
    responses(
        (status = 200, description = "bundle 摘要", body = BundleListItem),
        (status = 404, description = "bundle 未加载", body = ApiErrorBody),
    )
)]
pub async fn get_bundle(
    State(s): State<AppState>,
    AxumPath(bid): AxumPath<String>,
) -> Result<Json<BundleListItem>, ApiError> {
    let env = s.get_bundle(&bid).ok_or(ApiError::NotFound)?;
    Ok(Json(BundleListItem {
        id: bid,
        collections: env.collections().len(),
        objects: env.objects().len(),
        resources: env.resources().len(),
        failures: env.failures().len(),
    }))
}

/// 卸载 bundle(从内存移除 Environment)。
#[utoipa::path(
    delete,
    path = "/bundles/{bid}",
    tag = "bundle",
    params(("bid" = String, Path, description = "bundle id")),
    responses(
        (status = 200, description = "卸载成功", body = serde_json::Value),
        (status = 404, description = "bundle 未加载", body = ApiErrorBody),
    )
)]
pub async fn unload_bundle(
    State(s): State<AppState>,
    AxumPath(bid): AxumPath<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let removed = s.bundles.write().map.remove(&bid).is_some();
    if !removed {
        return Err(ApiError::NotFound);
    }
    Ok(Json(serde_json::json!({ "unloaded": true, "id": bid })))
}

// ── collection / object 列表 ──

/// AssetCollection(SerializedFile 逻辑分组)摘要。
#[derive(Serialize, utoipa::ToSchema)]
pub struct CollectionItem {
    pub index: usize,
    pub name: String,
    pub name_fixed: String,
    pub unity_version: String,
    pub endian: String,
    pub assets: usize,
}

/// 列出 bundle 内全部 collection。
#[utoipa::path(
    get,
    path = "/bundles/{bid}/collections",
    tag = "collection",
    params(("bid" = String, Path, description = "bundle id")),
    responses(
        (status = 200, description = "collection 列表", body = [CollectionItem]),
        (status = 404, description = "bundle 未加载", body = ApiErrorBody),
    )
)]
pub async fn list_collections(
    State(s): State<AppState>,
    AxumPath(bid): AxumPath<String>,
) -> Result<Json<Vec<CollectionItem>>, ApiError> {
    let env = s.get_bundle(&bid).ok_or(ApiError::NotFound)?;
    let out = env
        .collections()
        .iter()
        .enumerate()
        .map(|(i, c)| CollectionItem {
            index: i,
            name: c.name.clone(),
            name_fixed: c.name_fixed.clone(),
            unity_version: format!("{}", c.unity_version),
            endian: format!("{:?}", c.endian),
            assets: c.assets.len(),
        })
        .collect();
    Ok(Json(out))
}

/// 列出 collection 内全部对象的元数据(按 path_id 排序)。
#[utoipa::path(
    get,
    path = "/bundles/{bid}/collections/{cid}/objects",
    tag = "object",
    params(
        ("bid" = String, Path, description = "bundle id"),
        ("cid" = usize, Path, description = "collection 下标(见 list_collections 的 index)"),
    ),
    responses(
        (status = 200, description = "对象元数据列表", body = [ObjectMeta]),
        (status = 404, description = "bundle/collection 不存在", body = ApiErrorBody),
    )
)]
pub async fn list_objects(
    State(s): State<AppState>,
    AxumPath((bid, cid)): AxumPath<(String, usize)>,
) -> Result<Json<Vec<ObjectMeta>>, ApiError> {
    let env = s.get_bundle(&bid).ok_or(ApiError::NotFound)?;
    let col = env.collections().get(cid).ok_or(ApiError::NotFound)?;
    // path_id 全局不唯一,只在 collection 内唯一 —— 故必须带 collection 查。
    let mut rows: Vec<ObjectMeta> = col
        .assets
        .iter()
        .filter_map(|(&pid, &idx)| {
            env.objects().get(idx).map(|o| ObjectMeta::from_obj(pid, o))
        })
        .collect();
    rows.sort_by_key(|m| m.path_id);
    Ok(Json(rows))
}

// ── 单个对象 ──

/// [`get_object`] 的查询参数。
#[derive(Deserialize, IntoParams)]
pub struct DetailQuery {
    /// 传 `"full"` 附带原始 dump(typed → Debug 字符串,dynamic/mono → Value 递归 JSON)。
    pub detail: Option<String>,
}

/// 取单个对象的元数据 + 可选 detail。
///
/// path_id 只在 collection 内唯一,故寻址用复合 `{cid}/{pid}`。
#[utoipa::path(
    get,
    path = "/bundles/{bid}/collections/{cid}/objects/{pid}",
    tag = "object",
    params(
        ("bid" = String, Path, description = "bundle id"),
        ("cid" = usize, Path, description = "collection 下标"),
        ("pid" = i64, Path, description = "对象 path_id(collection 内唯一)"),
        DetailQuery,
    ),
    responses(
        (status = 200, description = "对象元数据(+可选 detail)", body = ObjectDto),
        (status = 404, description = "bundle/collection/object 不存在", body = ApiErrorBody),
    )
)]
pub async fn get_object(
    State(s): State<AppState>,
    AxumPath((bid, cid, pid)): AxumPath<(String, usize, i64)>,
    Query(q): Query<DetailQuery>,
) -> Result<Json<ObjectDto>, ApiError> {
    let env = s.get_bundle(&bid).ok_or(ApiError::NotFound)?;
    let col_id = env.collection_id(cid).ok_or(ApiError::NotFound)?;
    let obj = env.resolve_local(col_id, pid).ok_or(ApiError::NotFound)?;
    let detail = if q.detail.as_deref() == Some("full") {
        Some(ObjectDetail::from_parsed(&obj.object))
    } else {
        None
    };
    Ok(Json(ObjectDto {
        meta: ObjectMeta::from_obj(pid, obj),
        detail,
    }))
}

// ── PNG 导出(export feature gate)──

/// 导出 Texture2D / Sprite 为 PNG(`export` feature 下可用)。
#[cfg(feature = "export")]
#[utoipa::path(
    get,
    path = "/bundles/{bid}/collections/{cid}/objects/{pid}/png",
    tag = "object",
    params(
        ("bid" = String, Path, description = "bundle id"),
        ("cid" = usize, Path, description = "collection 下标"),
        ("pid" = i64, Path, description = "对象 path_id(须为 Texture2D/Sprite)"),
    ),
    responses(
        (status = 200, description = "PNG 图像字节", content_type = "image/png"),
        (status = 400, description = "非 Texture2D/Sprite", body = ApiErrorBody),
        (status = 404, description = "bundle/collection/object 不存在", body = ApiErrorBody),
        (status = 422, description = "解码/编码失败", body = ApiErrorBody),
    )
)]
pub async fn export_object_png(
    State(s): State<AppState>,
    AxumPath((bid, cid, pid)): AxumPath<(String, usize, i64)>,
) -> Result<axum::response::Response, ApiError> {
    use unity_asset::{DecodeOptions, DecodedImage, ParsedObject};

    let env = s.get_bundle(&bid).ok_or(ApiError::NotFound)?;
    // spawn_blocking 要 'static,不能把借用 `obj` move 进闭包。
    // 在闭包内用 clone 的 Arc<Environment> 重新 resolve。
    let env_for_blocking = env.clone();
    // collection id 校验提前做(避免 spawn 一个注定失败的任务)。
    let _ = env.collection_id(cid).ok_or(ApiError::NotFound)?;

    let decoded = tokio::task::spawn_blocking(move || -> Result<DecodedImage, String> {
        let col_id = env_for_blocking
            .collection_id(cid)
            .ok_or_else(|| format!("collection {cid} 不存在"))?;
        let obj = env_for_blocking
            .resolve_local(col_id, pid)
            .ok_or_else(|| format!("path_id {pid} 不存在"))?;
        // 快速过滤:非 Texture2D/Sprite 直接报错。
        if !matches!(&obj.object, ParsedObject::Typed(uo) if is_image_class(uo)) {
            return Err(format!("path_id={pid} 不是 Texture2D/Sprite"));
        }
        env_for_blocking
            .decode_image(obj, &DecodeOptions::default())
            .map_err(|e| format!("{e}"))
    })
    .await
    .map_err(|e| format!("worker join 失败: {e}"))?
    .map_err(ApiError::Export)?;

    let mut buf: Vec<u8> = Vec::with_capacity(decoded.rgba.len() / 2);
    decoded
        .write_png(&mut buf)
        .map_err(|e| ApiError::Export(format!("PNG 编码失败: {e}")))?;

    // 文件名已 sanitize(ASCII),from_str 安全。
    let filename = crate::sanitize_filename(&pid.to_string());
    let disposition =
        axum::http::HeaderValue::from_str(&format!("inline; filename=\"{filename}.png\""))
            .map_err(|e| ApiError::Internal(format!("非法 header: {e}")))?;

    let mut resp = axum::response::Response::new(buf.into());
    resp.headers_mut()
        .insert(header::CONTENT_TYPE, "image/png".parse().unwrap());
    resp.headers_mut()
        .insert(header::CONTENT_DISPOSITION, disposition);
    Ok(resp)
}

/// 无 export feature 时:PNG 端点不注册进 router(`routes!(export_object_png)` 在 mod.rs
/// 同样 cfg gate),此处保留同名函数以备无 feature 编译,但实际不会被调用。
#[cfg(not(feature = "export"))]
pub async fn export_object_png(
    _s: State<AppState>,
    _p: AxumPath<(String, usize, i64)>,
) -> Result<axum::response::Response, ApiError> {
    Err(ApiError::Internal(
        "export feature 未启用,PNG 导出不可用".into(),
    ))
}

// ── 路径安全 ──

/// 解析 `path` 相对 `root`,确保规范化后仍在 `root` 内(防 `../` 穿越)。
pub fn resolve_within_root(root: &Path, requested: &str) -> Result<PathBuf, ApiError> {
    let p = Path::new(requested);
    let joined = if p.is_absolute() {
        p.to_path_buf()
    } else {
        root.join(p)
    };
    // canonicalize 要求路径存在——加载前先校验存在性,顺带拿到规范化路径。
    let canon = joined
        .canonicalize()
        .map_err(|e| ApiError::BadRequest(format!("路径不存在 {}: {e}", requested)))?;
    let root_canon = root
        .canonicalize()
        .map_err(|e| ApiError::Internal(format!("根目录无效 {}: {e}", root.display())))?;
    // 校验 canon 以 root_canon 为前缀(逐 component 比对,避免符号链接绕过 starts_with)。
    if !starts_with_componentwise(&canon, &root_canon) {
        return Err(ApiError::BadRequest(format!(
            "路径 {} 超出根目录 {}",
            canon.display(),
            root_canon.display()
        )));
    }
    Ok(canon)
}

/// 逐 component 比较 `path` 是否以 `base` 开头(即 base 是 path 的前缀)。
/// `Path::starts_with` 的语义在某些平台上会因前缀组件差异误判,这里显式逐段比对更稳。
fn starts_with_componentwise(path: &Path, base: &Path) -> bool {
    let mut base_iter = base.components();
    for p_comp in path.components() {
        match base_iter.next() {
            // base 当前段匹配 path 当前段,继续比下一段。
            Some(b_comp) if components_eq(p_comp, b_comp) => continue,
            // base 已耗尽 → path 后续都是 base 的子路径,合法。
            None => return true,
            // base 当前段不匹配 → path 不在 base 下。
            Some(_) => return false,
        }
    }
    // path 耗尽:要么 path == base(长度相等,全匹配),要么 path 比 base 短(非法但返回 false 保守)。
    base_iter.next().is_none()
}

fn components_eq(a: Component<'_>, b: Component<'_>) -> bool {
    match (a, b) {
        (Component::Normal(a), Component::Normal(b)) => a == b,
        (Component::RootDir, Component::RootDir) => true,
        (Component::ParentDir, Component::ParentDir) => true,
        (Component::CurDir, Component::CurDir) => true,
        (Component::Prefix(a), Component::Prefix(b)) => a.as_os_str() == b.as_os_str(),
        _ => false,
    }
}
