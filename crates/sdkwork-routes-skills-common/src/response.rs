use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sdkwork_utils_rust::{SdkWorkApiResponse, SdkWorkResourceData};
use sdkwork_web_core::{
    problem_response, WebFrameworkError, WebFrameworkErrorKind, WebRequestContext,
};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiProblem>;

pub fn ok_json<T>(data: T) -> ApiResult<T> {
    Ok(data)
}

pub fn item_data<T>(item: T) -> SdkWorkResourceData<T> {
    SdkWorkResourceData { item }
}

fn success_response<T: Serialize>(
    ctx: &WebRequestContext,
    status: StatusCode,
    data: T,
) -> Result<Response, ApiProblem> {
    let trace_id = ctx.resolved_trace_id();
    let envelope = SdkWorkApiResponse::success(data, trace_id.clone());
    let mut response = (status, Json(envelope)).into_response();
    attach_trace_header(&mut response, &trace_id);
    Ok(response)
}

fn attach_trace_header(response: &mut Response, trace_id: &str) {
    if let Ok(value) = HeaderValue::from_str(trace_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-sdkwork-trace-id"), value);
    }
}

pub fn finish_api_json<T: Serialize>(ctx: &WebRequestContext, result: ApiResult<T>) -> Response {
    match result {
        Ok(data) => success_response(ctx, StatusCode::OK, data)
            .unwrap_or_else(|problem| problem.into_response_for(ctx)),
        Err(problem) => problem.into_response_for(ctx),
    }
}

pub fn finish_created_api_json<T: Serialize>(
    ctx: &WebRequestContext,
    result: ApiResult<T>,
) -> Response {
    match result {
        Ok(data) => success_response(ctx, StatusCode::CREATED, data)
            .unwrap_or_else(|problem| problem.into_response_for(ctx)),
        Err(problem) => problem.into_response_for(ctx),
    }
}

pub fn finish_no_content(ctx: &WebRequestContext, result: ApiResult<()>) -> Response {
    match result {
        Ok(()) => {
            let mut response = StatusCode::NO_CONTENT.into_response();
            attach_trace_header(&mut response, &ctx.resolved_trace_id());
            response
        }
        Err(problem) => problem.into_response_for(ctx),
    }
}

#[derive(Debug)]
pub struct ApiProblem {
    message: String,
    status: StatusCode,
}

impl ApiProblem {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::FORBIDDEN,
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn framework_error(&self) -> WebFrameworkError {
        let kind = match self.status {
            StatusCode::BAD_REQUEST => WebFrameworkErrorKind::BadRequest,
            StatusCode::FORBIDDEN => WebFrameworkErrorKind::Forbidden,
            StatusCode::NOT_FOUND => WebFrameworkErrorKind::NotFound,
            StatusCode::CONFLICT => WebFrameworkErrorKind::Conflict,
            StatusCode::INTERNAL_SERVER_ERROR => WebFrameworkErrorKind::InternalServerError,
            _ => WebFrameworkErrorKind::InternalServerError,
        };
        WebFrameworkError {
            kind,
            message: self.message.clone(),
            retry_after_seconds: None,
        }
    }

    pub fn into_response_for(self, ctx: &WebRequestContext) -> Response {
        problem_response(&self.framework_error(), ctx.problem_correlation())
    }
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use sdkwork_web_core::{ServerRequestId, WebApiSurface, WebAuthMode, WebTransportFacts};

    use super::*;

    fn test_context(method: &str) -> WebRequestContext {
        WebRequestContext {
            request_id: ServerRequestId("skills-response-test".to_owned()),
            api_surface: WebApiSurface::BackendApi,
            auth_mode: WebAuthMode::DualToken,
            principal: None,
            transport: WebTransportFacts {
                path: "/backend/v3/api/skill_packages".to_owned(),
                method: method.to_owned(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                oauth_bearer_present: false,
                agent_token_present: false,
            },
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: Some("skills-trace-id".to_owned()),
            idempotency_key: None,
        }
    }

    #[tokio::test]
    async fn create_response_uses_201_and_sdkwork_envelope() {
        let context = test_context("POST");
        let response = finish_created_api_json(
            &context,
            ok_json(serde_json::json!({ "item": { "id": "101" } })),
        );

        assert_eq!(StatusCode::CREATED, response.status());
        assert_eq!(
            Some("skills-trace-id"),
            response
                .headers()
                .get("x-sdkwork-trace-id")
                .and_then(|value| value.to_str().ok())
        );
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("create response body");
        let payload: serde_json::Value =
            serde_json::from_slice(&body).expect("create response JSON");
        assert_eq!(Some(0), payload["code"].as_i64());
        assert_eq!("101", payload["data"]["item"]["id"].as_str().unwrap());
        assert_eq!("skills-trace-id", payload["traceId"].as_str().unwrap());
    }

    #[tokio::test]
    async fn delete_response_uses_204_and_empty_body() {
        let context = test_context("DELETE");
        let response = finish_no_content(&context, ok_json(()));

        assert_eq!(StatusCode::NO_CONTENT, response.status());
        assert_eq!(
            Some("skills-trace-id"),
            response
                .headers()
                .get("x-sdkwork-trace-id")
                .and_then(|value| value.to_str().ok())
        );
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("delete response body");
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn delete_error_remains_problem_json() {
        let context = test_context("DELETE");
        let response = finish_no_content(&context, Err(ApiProblem::not_found("package missing")));

        assert_eq!(StatusCode::NOT_FOUND, response.status());
        assert_eq!(
            Some("application/problem+json"),
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
        );
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("problem response body");
        let payload: serde_json::Value =
            serde_json::from_slice(&body).expect("problem response JSON");
        assert_eq!(404, payload["status"].as_u64().unwrap());
        assert_eq!(40401, payload["code"].as_i64().unwrap());
        assert_eq!("skills-trace-id", payload["traceId"].as_str().unwrap());
    }
}
