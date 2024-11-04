use axum::{http::HeaderValue, routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::cors::{Any, CorsLayer};

mod error;

const MAX_AGE: u64 = 300; // 5 min
const PORT: u16 = 9000;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://voulr.com".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(MAX_AGE));

    let app = Router::new()
        .route("/", get(|| async { "voulr server!" }))
        .route("/health", get(|| async { "ok" }))
        .layer(cors);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);
    println!("\nlistening on http://{}\n", addr);

    cfg_if::cfg_if! {
        if #[cfg(feature = "lambda")] {
            bind_lambda(app).await?;
        } else {
            bind(app, addr).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "lambda")]
async fn bind_lambda(app: Router) -> Result<(), error::AppError> {
    lambda_http::tracing::init_default_subscriber();
    lambda_http::run(app).await.expect("failed start lambda");
    Ok(())
}

#[cfg(not(feature = "lambda"))]
async fn bind(app: Router, addr: SocketAddr) -> Result<(), error::AppError> {
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| error::AppError::Internal(e.to_string()))?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| error::AppError::Server(e.to_string()))?;

    Ok(())
}

#[cfg(not(feature = "lambda"))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");
    println!("\nshutting down gracefully...\n");
}
