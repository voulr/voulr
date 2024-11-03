use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::{error::Error, time::Duration};
use tower_http::cors::CorsLayer;

mod utils;

const MAX_AGE: u64 = 300; // 5 min
const ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://voulr.com".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::ACCEPT, header::CACHE_CONTROL, header::CONTENT_TYPE])
        .max_age(Duration::from_secs(MAX_AGE));

    let app = Router::new()
        .route("/", get(|| async { "voulr server!" }))
        .route("/health", get(|| async { "ok" }))
        .layer(cors);

    println!("\nlistening on http://{}\n", ADDR);

    #[cfg(feature = "lambda")]
    {
        lambda_http::tracing::init_default_subscriber();
        lambda_http::run(app).await;
    }

    #[cfg(not(feature = "lambda"))]
    {
        let listener = tokio::net::TcpListener::bind(&ADDR).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(utils::axum_shutdown_signal())
            .await?;
    }

    Ok(())
}
