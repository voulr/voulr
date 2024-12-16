use crate::Ctx;
use axum::{routing::get, Router};

mod callback;
mod login;

pub fn mount(ctx: Ctx) -> Router<Ctx> {
    Router::new()
        .route("/login", get(login::mount))
        .route("/callback", get(callback::mount))
        .with_state(ctx)
}
