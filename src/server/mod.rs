//! HTTP serve 子命令入口:axum router 装配 + OpenAPI 文档生成。
//!
//! 两套并存的 REST API(路径参数用 `{}` 语法,axum 0.8 + utoipa 一致):
//!
//! **A. legacy bundle API**(`/bundles*`,按 bundle id 寻址,多 bundle 并存):
//!
//! | Method | Path                                            | 作用                      |
//! |--------|-------------------------------------------------|---------------------------|
//! | GET    | `/health`                                       | 存活检查                  |
//! | GET    | `/bundles`                                      | 列出已加载 bundle         |
//! | POST   | `/bundles`                                      | body `{path,id?,arknights?}` 加载 |
//! | DELETE | `/bundles`                                      | 清空全部已加载 bundle     |
//! | GET    | `/bundles/{bid}`                                | bundle 详情               |
//! | DELETE | `/bundles/{bid}`                                | 卸载 bundle               |
//! | GET    | `/bundles/{bid}/collections`                    | 列 collection             |
//! | GET    | `/bundles/{bid}/collections/{cid}/objects`      | 列对象(元数据)          |
//! | GET    | `/bundles/{bid}/collections/{cid}/objects/{pid}` | 对象详情(?detail=full)   |
//! | GET    | `/bundles/{bid}/collections/{cid}/objects/{pid}/png` | 导出 PNG(Texture2D/Sprite) |
//! | GET    | `/bundles/{bid}/containers`                    | 列 container 条目(?path=/?kind=) |
//!
//! **B. source-aware environment API**(`/environment*`,单一活动 Environment,
//! 桌面端切项目用,不重启服务):
//!
//! | Method | Path                                            | 作用                      |
//! |--------|-------------------------------------------------|---------------------------|
//! | GET    | `/environment`                                  | 活动 Environment 摘要     |
//! | PUT    | `/environment`                                  | 原子替换全部 source       |
//! | DELETE | `/environment`                                  | 清空活动 Environment      |
//! | POST   | `/environment/sources`                          | 增量加 source             |
//! | DELETE | `/environment/sources/{sid}`                    | 删 source                 |
//! | POST   | `/environment/sources/{sid}/refresh`            | 重解析单个 source         |
//! | GET    | `/environment/collections`                      | 列 collection(?revision=) |
//! | GET    | `/environment/collections/{cid}/objects`        | 列对象(?revision=)        |
//! | GET    | `/environment/containers`                       | 列 container(?revision=&path=&kind=) |
//! | GET    | `/environment/collections/{cid}/objects/{pid}`  | 对象详情(?revision=&detail=full) |
//! | GET    | `/environment/collections/{cid}/objects/{pid}/png` | 导出 PNG(export feature) |
//!
//! environment API 带 `revision` 防陈旧:`?revision=` 与当前值不符返回 409 Conflict
//! (`stale environment revision`)。详见 `docs/electron-app.md`。
//!
//! | Method | Path                                            | 作用                      |
//! |--------|-------------------------------------------------|---------------------------|
//! | GET    | `/`                                             | Scalar UI(OpenApi 文档页) |
//! | GET    | `/openapi.json`                                 | OpenApi JSON 文档         |
//!
//! 寻址用复合路由 `/collections/{cid}/objects/{pid}`:path_id 全局不唯一,
//! 只在 collection 内唯一(`Environment::resolve_local` 要 collection+path_id)。
//!
//! 库 `unity-asset` 零 serde、完全同步,所有 DTO 在本模块手写,阻塞操作走 spawn_blocking。
//! OpenAPI 文档由 `#[utoipa::path]`(handler 上)+ `#[derive(ToSchema)]`(DTO 上)
//! 自动生成,经 `OpenApiRouter`(utoipa-axum)收集,Scalar UI 挂在 `/`。

use std::net::SocketAddr;
use std::path::PathBuf;

use axum::Router;
use axum::routing::get;
use parking_lot::RwLock;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

pub mod dto;
pub mod environment_routes;
pub mod error;
pub mod routes;
pub mod state;

use state::{AppState, Bundles};

/// OpenAPI 文档定义。tag 分组对齐资源(bundle/collection/object);
/// `components(schemas(...))` 列出无法经 `#[utoipa::path]` 自动收集的 schema
/// (此处 `ApiErrorBody` 用于错误响应 body 标注)。
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "bundle", description = "Bundle 加载/卸载/查询"),
        (name = "environment", description = "单一活动 Environment 与增量 source 管理"),
        (
            name = "collection",
            description = "AssetCollection(SerializedFile 逻辑分组)列表"
        ),
        (
            name = "object",
            description = "对象元数据/详情(?detail=full)/PNG 导出"
        ),
        (
            name = "container",
            description = "AssetBundle/ResourceManager 的 container(path -> 对象命名索引)"
        ),
    ),
    components(schemas(error::ApiErrorBody))
)]
pub struct ApiDoc;

/// 返回与 `/openapi.json` 完全相同的 OpenAPI 文档，供前端离线生成类型。
pub fn openapi_json() -> Result<String, String> {
    let (_, openapi) = api_router().split_for_parts();
    serde_json::to_string_pretty(&openapi).map_err(|e| format!("OpenAPI JSON 序列化失败: {e}"))
}

/// 启动 HTTP 服务,阻塞直到 server 退出。
///
/// `root` 为 POST /bundles 的 `path` 相对基准(已校验存在,不存在则报错)。
pub async fn serve(addr: SocketAddr, root: PathBuf) -> Result<(), String> {
    // root 启动时校验存在性(canonicalize),不存在直接报错退出。
    let root_canon = root
        .canonicalize()
        .map_err(|e| format!("根目录无效 {}: {e}", root.display()))?;

    let state = make_state(root_canon);
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("绑定 {addr} 失败: {e}"))?;
    log::info!(
        "serve: http://{addr}  (Scalar UI: http://{addr}/  OpenApi JSON: http://{addr}/openapi.json)"
    );
    axum::serve(listener, app)
        .await
        .map_err(|e| format!("server 退出: {e}"))?;
    Ok(())
}

/// 构造默认状态(空 bundles + 指定 root)。
fn make_state(root_canon: PathBuf) -> AppState {
    AppState {
        bundles: std::sync::Arc::new(RwLock::new(Bundles::default())),
        root: std::sync::Arc::new(root_canon),
        active: std::sync::Arc::new(RwLock::new(state::ActiveEnvironment::default())),
        mutation: std::sync::Arc::new(tokio::sync::Mutex::new(())),
    }
}

/// 装配 router:OpenApiRouter 收集带 `#[utoipa::path]` 的 handler → `(Router, OpenApi)`,
/// 再把 Scalar UI(`/`)和 OpenApi JSON(`/openapi.json`)合入主 router。
///
/// 抽出 `pub(crate)` 以便单元测试用 `tower::ServiceExt::oneshot` 直接打。
pub(crate) fn build_app(state: AppState) -> Router {
    let (api_routes, openapi) = api_router().split_for_parts();

    Router::new()
        .merge(api_routes)
        .merge(Scalar::with_url("/", openapi.clone()))
        .route("/openapi.json", get(|| async move { axum::Json(openapi) }))
        .with_state(state)
}

/// REST 路由与 OpenAPI 收集的唯一装配点，运行时文档和离线 codegen 共用。
fn api_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(routes::health))
        .routes(routes!(environment_routes::get_environment))
        .routes(routes!(environment_routes::replace_environment))
        .routes(routes!(environment_routes::clear_environment))
        .routes(routes!(environment_routes::add_source))
        .routes(routes!(environment_routes::remove_source))
        .routes(routes!(environment_routes::refresh_source))
        .routes(routes!(environment_routes::list_collections))
        .routes(routes!(environment_routes::list_objects))
        .routes(routes!(environment_routes::get_object))
        .routes(routes!(environment_routes::list_containers))
        .routes(routes!(environment_routes::export_object_png))
        .routes(routes!(routes::list_bundles))
        .routes(routes!(routes::load_bundle))
        .routes(routes!(routes::clear_bundles))
        .routes(routes!(routes::get_bundle))
        .routes(routes!(routes::unload_bundle))
        .routes(routes!(routes::list_collections))
        .routes(routes!(routes::list_objects))
        .routes(routes!(routes::get_object))
        .routes(routes!(routes::export_object_png))
        .routes(routes!(routes::list_containers))
}

#[cfg(test)]
mod tests {
    //! server 模块单元测试:用 `tower::ServiceExt::oneshot` 直接对 router 发请求,
    //! 无需起真实 TCP listener。fixture 走 `CARGO_MANIFEST_DIR`(根 crate = 仓库根)。

    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use serde_json::{Value, json};
    use tower::ServiceExt;

    /// example/ 目录(仓库根 + "example")。
    const EXAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/example");

    fn app() -> Router {
        let state = make_state(PathBuf::from(EXAMPLE_DIR));
        build_app(state)
    }

    async fn body_json(resp: axum::response::Response) -> Value {
        let bytes = to_bytes(resp.into_body(), 1024 * 1024 * 16).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_ok() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = to_bytes(resp.into_body(), 64).await.unwrap();
        assert_eq!(&bytes[..], b"ok");
    }

    #[tokio::test]
    async fn empty_bundles_list() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/bundles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let v = body_json(resp).await;
        assert!(v.as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn active_environment_adds_and_removes_sources_with_revisions() {
        let app = app();
        let replace = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/environment")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "sources": [{
                                "id": "first",
                                "path": "char_002_amiya.ab",
                                "kind": "file"
                            }]
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(replace.status(), StatusCode::OK);
        let first = body_json(replace).await;
        assert_eq!(first["sources"].as_array().unwrap().len(), 1);
        assert!(first["objects"].as_u64().unwrap() > 0);
        let first_revision = first["revision"].as_u64().unwrap();

        let add = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/environment/sources")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "id": "second",
                            "path": "Combine.unity3d",
                            "kind": "file"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(add.status(), StatusCode::CREATED);
        let second = body_json(add).await;
        assert_eq!(second["sources"].as_array().unwrap().len(), 2);
        assert!(second["revision"].as_u64().unwrap() > first_revision);

        let stale = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/environment/collections?revision={first_revision}"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(stale.status(), StatusCode::CONFLICT);

        let remove = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/environment/sources/first")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(remove.status(), StatusCode::OK);
        let remaining = body_json(remove).await;
        assert_eq!(remaining["sources"].as_array().unwrap().len(), 1);
        assert_eq!(remaining["sources"][0]["id"], "second");
    }

    #[tokio::test]
    async fn clear_all_bundles() {
        let app = app();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "char_002_amiya.ab",
                            "id": "to-clear"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/bundles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = body_json(response).await;
        assert_eq!(body["cleared"], 1);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/bundles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(body_json(response).await, json!([]));
    }

    #[tokio::test]
    async fn load_and_list_amiya() {
        let app = app();
        // POST /bundles 加载 char_002_amiya.ab。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "char_002_amiya.ab",
                            "id": "amiya"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let load = body_json(resp).await;
        assert_eq!(load["id"], "amiya");
        assert_eq!(load["loaded"], true);
        assert!(load["objects"].as_u64().unwrap() > 0);

        // GET /bundles 现在应有 1 条。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let list = body_json(resp).await;
        assert_eq!(list.as_array().unwrap().len(), 1);
        assert_eq!(list[0]["id"], "amiya");

        // GET /bundles/amiya/collections。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya/collections")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let cols = body_json(resp).await;
        assert!(!cols.as_array().unwrap().is_empty());
        assert_eq!(cols[0]["index"], 0);

        // GET collection 0 的对象列表,应含 MonoBehaviour。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya/collections/0/objects")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let objs = body_json(resp).await;
        let arr = objs.as_array().unwrap();
        assert!(!arr.is_empty());
        let has_mono = arr
            .iter()
            .any(|o| o["class_name"].as_str().unwrap() == "MonoBehaviour");
        assert!(has_mono, "应有 MonoBehaviour 对象");
    }

    #[tokio::test]
    async fn object_detail_full() {
        let app = app();
        // 先加载。
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "char_002_amiya.ab",
                            "id": "amiya2"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 取第一个对象 path_id。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya2/collections/0/objects")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let objs = body_json(resp).await;
        let first_pid = objs[0]["path_id"].as_str().unwrap();

        // 不带 detail:无 detail 字段。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/bundles/amiya2/collections/0/objects/{first_pid}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let obj = body_json(resp).await;
        assert_eq!(obj["path_id"].as_str(), Some(first_pid));
        assert!(obj.get("detail").is_none() || obj["detail"].is_null());

        // 带 detail=full:有 detail。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/bundles/amiya2/collections/0/objects/{first_pid}?detail=full"
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let obj = body_json(resp).await;
        assert!(obj["detail"].is_object());
        let kind = obj["detail"]["kind"].as_str().unwrap();
        assert!(matches!(kind, "typed" | "dynamic" | "mono"));
    }

    #[cfg(feature = "export")]
    #[tokio::test]
    async fn export_png_texture() {
        let app = app();
        // 加载有 Texture2D 的 bundle。
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "DownloadCommonUI.unity3d",
                            "id": "ui"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 找一个 Texture2D 的 path_id。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/ui/collections/0/objects")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let objs = body_json(resp).await;
        let tex_pid = objs
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["class_name"].as_str() == Some("Texture2D"))
            .map(|o| o["path_id"].as_str().unwrap())
            .expect("DownloadCommonUI 应有 Texture2D");

        // 导出 PNG。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/bundles/ui/collections/0/objects/{tex_pid}/png"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.headers().get("content-type").unwrap(), "image/png");
        let bytes = to_bytes(resp.into_body(), 1024 * 1024 * 16).await.unwrap();
        // PNG magic header。
        assert_eq!(
            &bytes[..8],
            &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
        );
        assert!(bytes.len() > 100);
    }

    #[tokio::test]
    async fn load_nonexistent_returns_400() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "does_not_exist.ab",
                            "id": "x"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let v = body_json(resp).await;
        assert!(v["error"].as_str().unwrap().contains("路径不存在"));
    }

    #[tokio::test]
    async fn path_traversal_blocked() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "../../../etc/passwd",
                            "id": "evil"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn not_found_routes() {
        let app = app();
        // 未加载的 bundle。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        // DELETE 已卸载的。
        let resp = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/bundles/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn containers_list_and_filter() {
        let app = app();
        // 加载 char_002_amiya.ab —— 必有 AssetBundle object,故 container 非空。
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/bundles")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "path": "char_002_amiya.ab",
                            "id": "amiya"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // GET /bundles/amiya/containers —— 应非空,且全部 kind=AssetBundle。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya/containers")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let arr = body_json(resp).await;
        let entries = arr.as_array().unwrap();
        assert!(!entries.is_empty(), "amiya bundle 应有 container 条目");
        assert!(
            entries
                .iter()
                .all(|e| e["kind"].as_str() == Some("AssetBundle")),
            "此 bundle 只应有 AssetBundle 类型的 container"
        );
        // host 字段必有,collection 在合法范围。
        for e in entries {
            let host_col = e["host"]["collection"].as_u64().unwrap();
            assert!(host_col < 100, "host.collection 应在合法范围");
        }
        let total = entries.len();

        // 过滤 kind=AssetBundle —— 条数不变(此 bundle 全是 AssetBundle)。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya/containers?kind=AssetBundle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let arr = body_json(resp).await;
        assert_eq!(arr.as_array().unwrap().len(), total);

        // 过滤 kind=ResourceManager —— 此 bundle 应为 0 条。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya/containers?kind=ResourceManager")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let arr = body_json(resp).await;
        assert!(arr.as_array().unwrap().is_empty());

        // 非法 kind 值 —— 400。
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/bundles/amiya/containers?kind=Foo")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        // path 过滤 —— 用第一条 entry 的 path 反查,应至少命中 1 条(multidict 可多条)。
        // 注:fixture 的 path 含 `/`(如 `assets/...`),`/` 在 query string 里合法
        // (RFC 3986),axum 能正确解析,无需转义。
        let first_path = entries[0]["path"].as_str().unwrap();
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/bundles/amiya/containers?path={first_path}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let arr = body_json(resp).await;
        let filtered = arr.as_array().unwrap();
        assert!(!filtered.is_empty(), "?path= 过滤应至少命中 1 条");
        assert!(
            filtered
                .iter()
                .all(|e| e["path"].as_str() == Some(first_path)),
            "?path= 过滤后所有条目 path 应一致"
        );
    }

    #[tokio::test]
    async fn containers_on_unloaded_bundle_is_404() {
        let app = app();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/bundles/nope/containers")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn openapi_json_accessible() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let v = body_json(resp).await;
        // OpenApi 3.0 文档结构:paths + components + info + openapi 版本字段。
        assert!(v.get("paths").is_some(), "缺 paths 字段");
        assert!(v.get("components").is_some(), "缺 components 字段");
        assert!(v.get("info").is_some(), "缺 info 字段");
        // 应包含已注册的路径。
        let paths = v["paths"].as_object().unwrap();
        assert!(
            paths.contains_key("/bundles"),
            "paths 应含 /bundles, 实际: {:?}",
            paths.keys().collect::<Vec<_>>()
        );
        assert!(paths.contains_key("/health"));
        assert!(paths.contains_key("/bundles/{bid}/collections/{cid}/objects/{pid}"));
    }

    #[tokio::test]
    async fn scalar_ui_accessible() {
        let resp = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let ct = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(
            ct.contains("text/html"),
            "Scalar 应返回 HTML, 实际 content-type: {ct}"
        );
    }
}
