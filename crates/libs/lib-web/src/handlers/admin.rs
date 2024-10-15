use std::ops::Deref;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use tower_cookies::Cookies;
use lib_core::context::app_context::ModelManager;
use lib_core::model::store::user::UserBmc;
use lib_core::model::user::UserForCreate;

#[derive(Deserialize)]
pub struct Params {
    user_id: i64,
}

pub async fn get_by_id(
    State(app_context): State<Arc<ModelManager>>,
    cookies: Cookies,
    Path(Params { user_id }): Path<Params>,
) -> Result<String, StatusCode> {
    println!("{:?}", user_id);
    let res = UserBmc::get_by_id(app_context.deref(), user_id).await;
    println!("{:?}", res);
    Ok("ok".to_string())
}