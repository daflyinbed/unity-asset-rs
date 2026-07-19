//! 单一活动 Environment 的 source-aware REST API。
//!
//! Rust 不知道 project；Electron 为每条持久化 source 提供稳定的 client id。

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use axum::Json;
use axum::extract::{Path as AxumPath, Query, State};
use axum::http::{StatusCode, header};
use serde::{Deserialize, Serialize};
use unity_asset::{Environment, Loader};
use utoipa::{IntoParams, ToSchema};

use super::dto::{ContainerEntryDto, ObjectDetail, ObjectDto, ObjectMeta, is_image_class};
use super::error::{ApiError, ApiErrorBody};
use super::routes::{CollectionItem, ContainerQuery, DetailQuery, resolve_within_root};
use super::state::{ActiveEnvironment, AppState, SourceRecord};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SourceKind {
    File,
    Directory,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct SourceInput {
    /// Electron/SQLite 分配的稳定 source id；Rust 只把它当作不透明字符串。
    pub id: String,
    pub path: String,
    pub kind: SourceKind,
    #[serde(default)]
    pub arknights: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct ReplaceEnvironmentBody {
    pub sources: Vec<SourceInput>,
}

#[derive(Clone, Serialize, ToSchema)]
pub struct SourceItem {
    pub id: String,
    pub path: String,
    pub kind: SourceKind,
    pub arknights: bool,
    /// 该 source 实际加载的文件。目录 source 会展开为递归扫描后命中的文件列表。
    pub files: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct EnvironmentSummary {
    pub revision: u64,
    pub sources: Vec<SourceItem>,
    pub collections: usize,
    pub objects: usize,
    pub resources: usize,
    pub failures: usize,
}

#[derive(Serialize, ToSchema)]
pub struct CollectionListResponse {
    pub revision: u64,
    pub items: Vec<CollectionItem>,
}

#[derive(Serialize, ToSchema)]
pub struct ObjectListResponse {
    pub revision: u64,
    pub items: Vec<ObjectMeta>,
}

#[derive(Serialize, ToSchema)]
pub struct ContainerListResponse {
    pub revision: u64,
    pub items: Vec<ContainerEntryDto>,
}

#[derive(Serialize, ToSchema)]
pub struct ObjectResponse {
    pub revision: u64,
    pub object: ObjectDto,
}

#[derive(Deserialize, IntoParams)]
pub struct RevisionQuery {
    /// 若提供且与当前 revision 不同，返回 409，避免把旧句柄用于新 Environment。
    pub revision: Option<u64>,
}

fn summary(active: &ActiveEnvironment) -> EnvironmentSummary {
    let mut sources = active
        .sources
        .iter()
        .map(|(id, source)| SourceItem {
            id: id.clone(),
            path: source.path.to_string_lossy().into_owned(),
            kind: if source.is_directory {
                SourceKind::Directory
            } else {
                SourceKind::File
            },
            arknights: source.arknights,
            files: source
                .files
                .iter()
                .map(|path| path.to_string_lossy().into_owned())
                .collect(),
        })
        .collect::<Vec<_>>();
    sources.sort_by(|a, b| a.id.cmp(&b.id));
    EnvironmentSummary {
        revision: active.revision,
        sources,
        collections: active.environment.collections().len(),
        objects: active.environment.objects().len(),
        resources: active.environment.resources().len(),
        failures: active.environment.failures().len(),
    }
}

fn check_revision(current: u64, requested: Option<u64>) -> Result<(), ApiError> {
    if let Some(requested) = requested
        && requested != current
    {
        return Err(ApiError::Conflict(format!(
            "stale environment revision {requested}; current revision is {current}"
        )));
    }
    Ok(())
}

async fn build_source(
    state: &AppState,
    source: &SourceInput,
) -> Result<(Environment, PathBuf, Vec<PathBuf>), ApiError> {
    if source.id.trim().is_empty() {
        return Err(ApiError::BadRequest("source id 不能为空".into()));
    }
    let full = resolve_within_root(&state.root, &source.path)?;
    if (source.kind == SourceKind::Directory) != full.is_dir() {
        return Err(ApiError::BadRequest(format!(
            "source kind 与路径类型不一致: {}",
            full.display()
        )));
    }
    let registry = crate::make_registry(source.arknights);
    let build_path = full.clone();
    let kind = source.kind;
    let (built, loaded_files) =
        tokio::task::spawn_blocking(move || -> Result<(Environment, Vec<PathBuf>), String> {
            let builder = Loader::builder().registry(&registry);
            let report = match kind {
                SourceKind::File => builder.path(&build_path).build(),
                SourceKind::Directory => builder.scan_dir(&build_path).build(),
            }
            .map_err(|error| format!("build 失败: {error}"))?;
            for diagnostic in report.diagnostics() {
                eprintln!("{diagnostic}");
            }
            let loaded_files = report.loaded_files().to_vec();
            Ok((report.into_environment(), loaded_files))
        })
        .await
        .map_err(|error| ApiError::Internal(format!("worker join 失败: {error}")))?
        .map_err(ApiError::Load)?;
    Ok((built, full, loaded_files))
}

#[utoipa::path(
    get, path = "/environment", tag = "environment",
    responses((status = 200, body = EnvironmentSummary))
)]
pub async fn get_environment(State(state): State<AppState>) -> Json<EnvironmentSummary> {
    Json(summary(&state.active.read()))
}

#[utoipa::path(
    put, path = "/environment", tag = "environment", request_body = ReplaceEnvironmentBody,
    responses(
        (status = 200, description = "原子替换活动 Environment", body = EnvironmentSummary),
        (status = 400, body = ApiErrorBody), (status = 422, body = ApiErrorBody)
    )
)]
pub async fn replace_environment(
    State(state): State<AppState>,
    Json(body): Json<ReplaceEnvironmentBody>,
) -> Result<Json<EnvironmentSummary>, ApiError> {
    let _mutation = state.mutation.lock().await;
    let mut seen = HashSet::new();
    if body
        .sources
        .iter()
        .any(|source| !seen.insert(source.id.clone()))
    {
        return Err(ApiError::BadRequest("source id 不能重复".into()));
    }
    let mut staged = Vec::with_capacity(body.sources.len());
    for source in body.sources {
        let (environment, canonical_path, files) = build_source(&state, &source).await?;
        staged.push((source, canonical_path, files, environment));
    }

    let mut environment = Environment::empty();
    let mut sources = HashMap::new();
    for (input, canonical_path, files, parsed) in staged {
        let runtime_id = environment.insert_source(parsed);
        sources.insert(
            input.id,
            SourceRecord {
                runtime_id,
                path: canonical_path,
                is_directory: input.kind == SourceKind::Directory,
                arknights: input.arknights,
                files,
            },
        );
    }
    let mut active = state.active.write();
    let revision = active.revision.saturating_add(1);
    *active = ActiveEnvironment {
        environment,
        sources,
        revision,
    };
    Ok(Json(summary(&active)))
}

#[utoipa::path(
    delete, path = "/environment", tag = "environment",
    responses((status = 200, body = EnvironmentSummary))
)]
pub async fn clear_environment(State(state): State<AppState>) -> Json<EnvironmentSummary> {
    let _mutation = state.mutation.lock().await;
    let mut active = state.active.write();
    let revision = active.revision.saturating_add(1);
    *active = ActiveEnvironment {
        revision,
        ..ActiveEnvironment::default()
    };
    Json(summary(&active))
}

#[utoipa::path(
    post, path = "/environment/sources", tag = "environment", request_body = SourceInput,
    responses(
        (status = 201, body = EnvironmentSummary),
        (status = 400, body = ApiErrorBody), (status = 409, body = ApiErrorBody),
        (status = 422, body = ApiErrorBody)
    )
)]
pub async fn add_source(
    State(state): State<AppState>,
    Json(input): Json<SourceInput>,
) -> Result<(StatusCode, Json<EnvironmentSummary>), ApiError> {
    let _mutation = state.mutation.lock().await;
    if state.active.read().sources.contains_key(&input.id) {
        return Err(ApiError::Conflict(format!("source {} 已存在", input.id)));
    }
    let (parsed, canonical_path, files) = build_source(&state, &input).await?;
    let mut active = state.active.write();
    let runtime_id = active.environment.insert_source(parsed);
    active.sources.insert(
        input.id,
        SourceRecord {
            runtime_id,
            path: canonical_path,
            is_directory: input.kind == SourceKind::Directory,
            arknights: input.arknights,
            files,
        },
    );
    active.revision = active.revision.saturating_add(1);
    Ok((StatusCode::CREATED, Json(summary(&active))))
}

#[utoipa::path(
    delete, path = "/environment/sources/{sid}", tag = "environment",
    params(("sid" = String, Path)),
    responses((status = 200, body = EnvironmentSummary), (status = 404, body = ApiErrorBody))
)]
pub async fn remove_source(
    State(state): State<AppState>,
    AxumPath(source_id): AxumPath<String>,
) -> Result<Json<EnvironmentSummary>, ApiError> {
    let _mutation = state.mutation.lock().await;
    let mut active = state.active.write();
    let source = active
        .sources
        .remove(&source_id)
        .ok_or(ApiError::NotFound)?;
    active.environment.remove_source(source.runtime_id);
    active.revision = active.revision.saturating_add(1);
    Ok(Json(summary(&active)))
}

#[utoipa::path(
    post, path = "/environment/sources/{sid}/refresh", tag = "environment",
    params(("sid" = String, Path)),
    responses((status = 200, body = EnvironmentSummary), (status = 404, body = ApiErrorBody), (status = 422, body = ApiErrorBody))
)]
pub async fn refresh_source(
    State(state): State<AppState>,
    AxumPath(source_id): AxumPath<String>,
) -> Result<Json<EnvironmentSummary>, ApiError> {
    let _mutation = state.mutation.lock().await;
    let input = {
        let active = state.active.read();
        let source = active.sources.get(&source_id).ok_or(ApiError::NotFound)?;
        SourceInput {
            id: source_id.clone(),
            path: source.path.to_string_lossy().into_owned(),
            kind: if source.is_directory {
                SourceKind::Directory
            } else {
                SourceKind::File
            },
            arknights: source.arknights,
        }
    };
    let (replacement, canonical_path, files) = build_source(&state, &input).await?;
    let mut active = state.active.write();
    let runtime_id = active
        .sources
        .get(&source_id)
        .ok_or(ApiError::NotFound)?
        .runtime_id;
    active.environment.replace_source(runtime_id, replacement);
    if let Some(source) = active.sources.get_mut(&source_id) {
        source.path = canonical_path;
        source.files = files;
    }
    active.revision = active.revision.saturating_add(1);
    Ok(Json(summary(&active)))
}

#[utoipa::path(
    get, path = "/environment/collections", tag = "collection", operation_id = "list_environment_collections", params(RevisionQuery),
    responses((status = 200, body = CollectionListResponse), (status = 409, body = ApiErrorBody))
)]
pub async fn list_collections(
    State(state): State<AppState>,
    Query(query): Query<RevisionQuery>,
) -> Result<Json<CollectionListResponse>, ApiError> {
    let active = state.active.read();
    check_revision(active.revision, query.revision)?;
    let items = active
        .environment
        .collections()
        .iter()
        .enumerate()
        .map(|(index, collection)| CollectionItem {
            index,
            name: collection.name.clone(),
            name_fixed: collection.name_fixed.clone(),
            unity_version: collection.unity_version.to_string(),
            endian: format!("{:?}", collection.endian),
            assets: collection.assets.len(),
        })
        .collect();
    Ok(Json(CollectionListResponse {
        revision: active.revision,
        items,
    }))
}

#[utoipa::path(
    get, path = "/environment/collections/{cid}/objects", tag = "object", operation_id = "list_environment_objects",
    params(("cid" = usize, Path), RevisionQuery),
    responses((status = 200, body = ObjectListResponse), (status = 404, body = ApiErrorBody), (status = 409, body = ApiErrorBody))
)]
pub async fn list_objects(
    State(state): State<AppState>,
    AxumPath(collection_id): AxumPath<usize>,
    Query(query): Query<RevisionQuery>,
) -> Result<Json<ObjectListResponse>, ApiError> {
    let active = state.active.read();
    check_revision(active.revision, query.revision)?;
    let collection = active
        .environment
        .collections()
        .get(collection_id)
        .ok_or(ApiError::NotFound)?;
    let mut assets = collection.assets.iter().collect::<Vec<_>>();
    assets.sort_by_key(|&(&path_id, _)| path_id);
    let items = assets
        .into_iter()
        .filter_map(|(&path_id, &index)| {
            active
                .environment
                .objects()
                .get(index)
                .map(|object| ObjectMeta::from_obj(path_id, object))
        })
        .collect::<Vec<_>>();
    Ok(Json(ObjectListResponse {
        revision: active.revision,
        items,
    }))
}

#[utoipa::path(
    get, path = "/environment/containers", tag = "container", operation_id = "list_environment_containers", params(ContainerQuery),
    responses((status = 200, body = ContainerListResponse), (status = 400, body = ApiErrorBody), (status = 409, body = ApiErrorBody))
)]
pub async fn list_containers(
    State(state): State<AppState>,
    Query(query): Query<ContainerQuery>,
) -> Result<Json<ContainerListResponse>, ApiError> {
    let active = state.active.read();
    check_revision(active.revision, query.revision)?;
    let kind = match query.kind.as_deref() {
        None => None,
        Some("AssetBundle") => Some(unity_asset::ContainerKind::AssetBundle),
        Some("ResourceManager") => Some(unity_asset::ContainerKind::ResourceManager),
        Some(_) => {
            return Err(ApiError::BadRequest(
                "kind 仅支持 AssetBundle / ResourceManager".into(),
            ));
        }
    };
    let collections_len = active.environment.collections().len();
    let items = active
        .environment
        .containers()
        .entries()
        .iter()
        .filter(|entry| query.path.as_deref().is_none_or(|path| entry.path == path))
        .filter(|entry| kind.is_none_or(|kind| entry.kind == kind))
        .map(|entry| ContainerEntryDto::from_entry(entry, collections_len))
        .collect();
    Ok(Json(ContainerListResponse {
        revision: active.revision,
        items,
    }))
}

#[utoipa::path(
    get, path = "/environment/collections/{cid}/objects/{pid}", tag = "object", operation_id = "get_environment_object",
    params(("cid" = usize, Path), ("pid" = String, Path, description = "path_id 十进制字符串"), DetailQuery),
    responses((status = 200, body = ObjectResponse), (status = 400, body = ApiErrorBody), (status = 404, body = ApiErrorBody), (status = 409, body = ApiErrorBody))
)]
pub async fn get_object(
    State(state): State<AppState>,
    AxumPath((collection_id, path_id)): AxumPath<(usize, String)>,
    Query(query): Query<DetailQuery>,
) -> Result<Json<ObjectResponse>, ApiError> {
    let path_id = super::routes::parse_path_id(&path_id)?;
    let active = state.active.read();
    check_revision(active.revision, query.revision)?;
    let collection = active
        .environment
        .collection_id(collection_id)
        .ok_or(ApiError::NotFound)?;
    let object = active
        .environment
        .resolve_local(collection, path_id)
        .ok_or(ApiError::NotFound)?;
    let detail = (query.detail.as_deref() == Some("full"))
        .then(|| ObjectDetail::from_parsed(&object.object));
    Ok(Json(ObjectResponse {
        revision: active.revision,
        object: ObjectDto {
            meta: ObjectMeta::from_obj(path_id, object),
            detail,
        },
    }))
}

#[cfg(feature = "export")]
#[utoipa::path(
    get, path = "/environment/collections/{cid}/objects/{pid}/png", tag = "object", operation_id = "export_environment_object_png",
    params(("cid" = usize, Path), ("pid" = String, Path, description = "path_id 十进制字符串"), RevisionQuery),
    responses((status = 200, content_type = "image/png"), (status = 400, body = ApiErrorBody), (status = 404, body = ApiErrorBody), (status = 422, body = ApiErrorBody))
)]
pub async fn export_object_png(
    State(state): State<AppState>,
    AxumPath((collection_id, path_id)): AxumPath<(usize, String)>,
    Query(revision): Query<RevisionQuery>,
) -> Result<axum::response::Response, ApiError> {
    use unity_asset::{DecodeOptions, DecodedImage, ParsedObject};
    let path_id = super::routes::parse_path_id(&path_id)?;
    {
        let active = state.active.read();
        check_revision(active.revision, revision.revision)?;
        active
            .environment
            .collection_id(collection_id)
            .ok_or(ApiError::NotFound)?;
    }
    let active = state.active.clone();
    let decoded = tokio::task::spawn_blocking(move || -> Result<DecodedImage, String> {
        let active = active.read();
        let collection = active
            .environment
            .collection_id(collection_id)
            .ok_or_else(|| "collection 不存在".to_string())?;
        let object = active
            .environment
            .resolve_local(collection, path_id)
            .ok_or_else(|| "object 不存在".to_string())?;
        if !matches!(&object.object, ParsedObject::Typed(value) if is_image_class(value)) {
            return Err("对象不是 Texture2D/Sprite".into());
        }
        active
            .environment
            .decode_image(object, &DecodeOptions::default())
            .map_err(|error| error.to_string())
    })
    .await
    .map_err(|error| ApiError::Internal(format!("worker join 失败: {error}")))?
    .map_err(ApiError::Export)?;
    let mut bytes = Vec::with_capacity(decoded.rgba.len() / 2);
    decoded
        .write_png(&mut bytes)
        .map_err(|error| ApiError::Export(format!("PNG 编码失败: {error}")))?;
    let mut response = axum::response::Response::new(bytes.into());
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        "image/png".parse().expect("valid MIME"),
    );
    Ok(response)
}
