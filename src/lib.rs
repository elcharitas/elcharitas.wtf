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
#[event(fetch)]
async fn main(req: HttpRequest, env: Env, _ctx: Context) -> Result<axum::response::Response> {
    console_error_panic_hook::set_once();

    shared::init_env(&env);

    let router = app::create_router();
    router
        .call(req)
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))
}
