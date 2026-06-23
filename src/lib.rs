#[cfg(target_arch = "wasm32")]
mod wasm_handler {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn main() {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
    }
}
