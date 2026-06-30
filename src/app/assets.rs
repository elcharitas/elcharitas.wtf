use axum::{http::header, response::IntoResponse};

pub async fn styles_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../public/styles.css"),
    )
}

pub async fn favicon_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../../public/icon.png").as_slice(),
    )
}

pub async fn og_image_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../../public/og.png").as_slice(),
    )
}
