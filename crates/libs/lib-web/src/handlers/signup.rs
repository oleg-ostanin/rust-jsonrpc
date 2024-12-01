use std::ops::Deref;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tower_cookies::Cookies;
use lib_core::context::app_context::ModelManager;
use lib_core::model::user::UserForCreate;



use lib_core::model::store::user::UserBmc;
use crate::error::Error;

pub async fn sign_up(
    State(app_context): State<Arc<ModelManager>>,
    cookies: Cookies,
    Json(payload): Json<UserForCreate>,
) -> crate::error::Result<()> {
    println!("{:?}", payload);
    Ok(UserBmc::create(app_context.deref(), payload).await?)
}