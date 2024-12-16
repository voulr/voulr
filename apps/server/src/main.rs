use axum::{http::HeaderValue, routing::get, Router};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::{env, net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};

mod routes;

const MAX_AGE: u64 = 300; // 5 min
const PORT: u16 = 9000;

#[derive(Clone)]
pub struct Ctx {
    github: BasicClient,
    reqwest: reqwest::Client,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://voulr.com".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(MAX_AGE));

    let github = {
        let client_id = env::var("GITHUB_CLIENT_ID").unwrap();
        let client_secret = env::var("GITHUB_CLIENT_SECRET").unwrap();
        let auth_url = "https://github.com/login/oauth/authorize".to_string();
        let token_url = "https://github.com/login/oauth/access_token".to_string();
        let redirect_url =
            "https://fu7hlwg3jb37nute2mtonv2jzi0kqtca.lambda-url.eu-west-2.on.aws/github/callback"
                .to_string();

        BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
    };

    let reqwest = reqwest::Client::new();

    let ctx = Ctx { github, reqwest };

    let app = Router::new()
        .route("/", get(|| async { "voulr server!" }))
        .route("/health", get(|| async { "ok" }))
        .nest("/github", routes::github::mount(ctx.clone()))
        .layer(cors)
        .with_state(ctx);

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
