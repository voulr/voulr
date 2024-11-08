use axum::{http::HeaderValue, routing::get, Router};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    env::var,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use tower_http::cors::{Any, CorsLayer};

mod error;
mod routes;

const MAX_AGE: u64 = 300; // 5 min
const PORT: u16 = 9000;

#[derive(Clone)]
pub struct Ctx {
    pub db: Pool<Postgres>,
    pub oauth: BasicClient,
}

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
        .max_age(Duration::from_secs(MAX_AGE));

    let db = {
        let url = var("DATABASE_URL").map_err(|_| error::AppError::EnvVarNotSet("DATABASE_URL"))?;
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .map_err(|e| error::AppError::Database(e.to_string()))?
    };

    let oauth = {
        let client_id = ClientId::new(
            var("GH_OAUTH_CLIENT_ID")
                .map_err(|_| error::AppError::EnvVarNotSet("GH_OAUTH_CLIENT_ID"))?,
        );
        let client_secret = ClientSecret::new(
            var("GH_OAUTH_CLIENT_SECRET")
                .map_err(|_| error::AppError::EnvVarNotSet("GH_OAUTH_CLIENT_SECRET"))?,
        );
    };

    let ctx = Ctx { db, oauth };

    let app = Router::new()
        .route("/", get(|| async { "voulr server!" }))
        .route("/health", get(|| async { "ok" }))
        .layer(cors)
        .with_state(ctx);

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
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install ctrl+c handler");
            println!("\nshutting down gracefully...\n");
        })
        .await
        .map_err(|e| error::AppError::Server(e.to_string()))?;
    Ok(())
}
