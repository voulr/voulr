use axum::{
    body::Body,
    http::{header, StatusCode},
    response::IntoResponse,
};
use tokio_util::io::ReaderStream;

pub async fn mount() -> impl IntoResponse {
    let file = match tokio::fs::File::open("data/voulr-logos.zip").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "application/zip"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"voulr-logos.zip\"",
        ),
    ];

    Ok((headers, body))
}
