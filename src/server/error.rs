//! 统一 API 错误。转 axum `IntoResponse`,所有 handler 返回 `Result<T, ApiError>`。

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;

/// 错误响应体(所有非 2xx 响应统一是 `{ "error": <msg> }`)。
///
/// 仅用于 OpenApi 文档标注(`#[utoipa::path] responses(... body = ApiErrorBody)`);
/// 实际序列化由 [`ApiError::into_response`] 用 `json!` 完成结构相同。
#[derive(Serialize, ToSchema)]
pub struct ApiErrorBody {
    pub error: String,
}

/// HTTP 错误。统一序列化成 `{"error": <msg>}`,handler 里 `?` 即可。
#[derive(Debug)]
pub enum ApiError {
    /// 资源不存在(bundle/collection/object id 找不到)。
    NotFound,
    /// 请求参数非法(JSON 解析失败、path 穿越等)。
    BadRequest(String),
    /// 加载/构建 Environment 失败。
    Load(String),
    /// 导出(PNG)失败。
    Export(String),
    /// 其他内部错误(spawn_blocking join 失败等)。
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, msg): (StatusCode, String) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            ApiError::Load(m) => (StatusCode::UNPROCESSABLE_ENTITY, m),
            ApiError::Export(m) => (StatusCode::UNPROCESSABLE_ENTITY, m),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (code, Json(json!({ "error": msg }))).into_response()
    }
}

impl From<axum::Error> for ApiError {
    fn from(e: axum::Error) -> Self {
        ApiError::BadRequest(format!("body 解析失败: {e}"))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError::Internal(format!("JSON 序列化失败: {e}"))
    }
}

/// handler 里 spawn_blocking 闭包常返回 `Result<_, String>`,统一映射成 Load。
impl From<String> for ApiError {
    fn from(s: String) -> Self {
        ApiError::Load(s)
    }
}
