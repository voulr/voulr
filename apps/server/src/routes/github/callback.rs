use crate::{
    routes::github::login::{CSRF_TOKEN, PKCE_VERIFIER},
    Ctx,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use axum_extra::{headers, TypedHeader};
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct GithubCallbackParams {
    code: String,
    state: String,
}

pub async fn mount(
    State(ctx): State<Ctx>,
    Query(params): Query<GithubCallbackParams>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let csrf_token = cookies.get(CSRF_TOKEN).unwrap();
    if csrf_token != params.state {
        return Err("CSRF token mismatch");
    }

    let pkce_verifier = PkceCodeVerifier::new(cookies.get(PKCE_VERIFIER).unwrap().to_string());
    let token = ctx
        .github
        .exchange_code(AuthorizationCode::new(params.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .unwrap();

    let email = ctx
        .reqwest
        .get("https://api.github.com/user/emails")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<Vec<GitHubEmail>>()
        .await
        .unwrap();
    let primary_email = email.into_iter().find(|e| e.primary).unwrap();
    println!("GitHub Primary Email: {}", primary_email.email);

    Ok(())
}
