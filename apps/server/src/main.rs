use axum::{http::HeaderValue, routing::get, Router};
use std::{error::Error, time::Duration};
use tower_http::cors::{Any, CorsLayer};

mod utils;

const MAX_AGE: u64 = 300; // 5 min
const ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://voulr.com".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
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
            .with_graceful_shutdown(utils::graceful_shutdown())
            .await?;
    }

    Ok(())
}
