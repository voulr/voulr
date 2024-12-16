use crate::Ctx;
use axum::{
    extract::State,
    http::header::{HeaderValue, SET_COOKIE},
    response::{IntoResponse, Redirect},
};
use oauth2::{CsrfToken, PkceCodeChallenge, Scope};

pub const CSRF_TOKEN: &str = "csrf_token";
pub const PKCE_VERIFIER: &str = "pkce_verifier";

pub async fn mount(State(ctx): State<Ctx>) -> impl IntoResponse {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = ctx
        .github
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let csrf_cookie = format!(
        "{}={}; SameSite=Lax; HttpOnly; Secure; Path=/",
        CSRF_TOKEN,
        csrf_token.secret()
    );
    let pkce_cookie = format!(
        "{}={}; SameSite=Lax; HttpOnly; Secure; Path=/",
        PKCE_VERIFIER,
        pkce_verifier.secret()
    );
    let headers = [
        (SET_COOKIE, HeaderValue::from_str(&csrf_cookie).unwrap()),
        (SET_COOKIE, HeaderValue::from_str(&pkce_cookie).unwrap()),
    ];

    (headers, Redirect::to(auth_url.as_ref()))
}
