use wasm_bindgen::prelude::*;
use worker::*;

mod app;
mod analytics;
mod components;
mod requests;
mod shared;

use app::create_router;

#[wasm_bindgen]
pub async fn handle(req: Request) -> Result<Response> {
    utils::set_panic_hook();

    let router = create_router();

    match router.oneshot(req).await {
        Ok(res) => Ok(res),
        Err(_) => Response::error("Internal Server Error", 500),
    }
}

pub fn utils_set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
