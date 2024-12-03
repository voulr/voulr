use axum::{http::HeaderValue, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};

mod routes;

const MAX_AGE: u64 = 300; // 5 min
const PORT: u16 = 9000;

#[tokio::main]
async fn main() {
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

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    println!("\nlistening on http://{}\n", addr);

    cfg_if::cfg_if! {
        if #[cfg(feature = "lambda")] {
            bind_lambda(app).await;
        } else {
            bind(app, addr).await;
        }
    }
}

#[cfg(feature = "lambda")]
async fn bind_lambda(app: Router) {
    lambda_http::tracing::init_default_subscriber();
    lambda_http::run(app).await.unwrap();
}

#[cfg(not(feature = "lambda"))]
async fn bind(app: Router, addr: SocketAddr) {
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("\nshutting down gracefully...\n");
        })
        .await
        .unwrap();
}
