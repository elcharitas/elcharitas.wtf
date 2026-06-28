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
use worker::*;
#[cfg(target_arch = "wasm32")]
use tower::ServiceExt;

#[cfg(target_arch = "wasm32")]
#[event(fetch)]
async fn main(req: HttpRequest, env: Env, _ctx: Context) -> Result<axum::response::Response> {
    console_error_panic_hook::set_once();

    shared::init_env(&env);
    shared::init_kv(&env);

    // Handle POST /newsletter before the axum router: KvBuilder is !Send so it cannot
    // cross an await point inside an axum Handler future (which requires Send).
    if req.method() == axum::http::Method::POST && req.uri().path() == "/newsletter" {
        use axum::response::IntoResponse;
        let (_, body) = req.into_parts();
        use http_body_util::BodyExt;
        let bytes = body.collect().await.map(|c| c.to_bytes()).unwrap_or_default();
        let body_str = String::from_utf8_lossy(&bytes).to_string();
        console_log!("newsletter: POST /newsletter — body length {}", body_str.len());
        return Ok(app::newsletter::newsletter_post_handler(body_str).await.into_response());
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
