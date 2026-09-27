#[cfg(target_arch = "wasm32")]
mod analytics;
#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod components;
#[cfg(target_arch = "wasm32")]
mod requests;
#[cfg(target_arch = "wasm32")]
mod shared;

#[cfg(target_arch = "wasm32")]
use tower::ServiceExt;
#[cfg(target_arch = "wasm32")]
use worker::*;

#[cfg(target_arch = "wasm32")]
const READ_RATE_LIMITER: &str = "READ_RATE_LIMITER";
#[cfg(target_arch = "wasm32")]
const MUTATION_RATE_LIMITER: &str = "MUTATION_RATE_LIMITER";
#[cfg(target_arch = "wasm32")]
const RATE_LIMIT_RETRY_AFTER_SECS: &str = "60";

#[cfg(target_arch = "wasm32")]
async fn enforce_rate_limit(req: &HttpRequest, env: &Env) -> Option<axum::response::Response> {
    use axum::{
        http::{HeaderValue, Method, StatusCode, header},
        response::IntoResponse,
    };

    let is_mutation = !matches!(
        req.method(),
        &Method::GET | &Method::HEAD | &Method::OPTIONS
    );
    let binding = if is_mutation {
        MUTATION_RATE_LIMITER
    } else {
        READ_RATE_LIMITER
    };
    let client = req
        .headers()
        .get("cf-connecting-ip")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .unwrap_or("unknown");
    let key = if is_mutation {
        format!("{client}:{}:{}", req.method(), req.uri().path())
    } else {
        format!("{client}:read")
    };

    let unavailable_response = || {
        let mut response = (
            StatusCode::SERVICE_UNAVAILABLE,
            "Request protection is temporarily unavailable. Please try again shortly.",
        )
            .into_response();
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        response
    };

    let limiter = match env.get_binding::<RateLimiter>(binding) {
        Ok(limiter) => limiter,
        Err(error) => {
            console_error!("rate limit binding unavailable: {error}");
            return Some(unavailable_response());
        }
    };

    match limiter.limit(key).await {
        Ok(outcome) if outcome.success => None,
        Ok(_) => {
            let mut response = (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many requests. Please try again in a minute.",
            )
                .into_response();
            response.headers_mut().insert(
                header::RETRY_AFTER,
                HeaderValue::from_static(RATE_LIMIT_RETRY_AFTER_SECS),
            );
            response
                .headers_mut()
                .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
            Some(response)
        }
        Err(error) => {
            console_error!("rate limit check failed: {error}");
            Some(unavailable_response())
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[event(fetch)]
async fn main(req: HttpRequest, env: Env, _ctx: Context) -> Result<axum::response::Response> {
    console_error_panic_hook::set_once();

    if let Some(response) = enforce_rate_limit(&req, &env).await {
        return Ok(response);
    }

    shared::init_env(&env);
    shared::init_kv(&env);

    // Handle POST /newsletter before the axum router: KvBuilder is !Send so it cannot
    // cross an await point inside an axum Handler future (which requires Send).
    if req.method() == axum::http::Method::POST && req.uri().path() == "/newsletter/send" {
        use axum::response::IntoResponse;
        let headers = req.headers().clone();
        return Ok(app::newsletter::newsletter_send_handler(headers)
            .await
            .into_response());
    }

    if req.method() == axum::http::Method::POST && req.uri().path() == "/newsletter" {
        use axum::response::IntoResponse;
        let (parts, body) = req.into_parts();
        use http_body_util::BodyExt;
        let bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .unwrap_or_default();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        console_log!(
            "newsletter: POST /newsletter — body length {}",
            body_str.len()
        );
        return Ok(
            app::newsletter::newsletter_post_handler(parts.headers, body_str)
                .await
                .into_response(),
        );
    }

    if let Some(resp) = app::wasm_dynamic_response(&req).await {
        return Ok(resp);
    }

    let router = app::create_router();
    router
        .oneshot(req)
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[event(scheduled)]
async fn scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();
    shared::init_env(&env);
    shared::init_kv(&env);
    app::newsletter::send_newsletter().await;
}
