use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use tower_cookies::Cookies;
use lib_core::context::app_context::ModelManager;
use lib_core::model::user::UserForCreate;

pub async fn sign_up(
    State(app_context): State<Arc<ModelManager>>,
    Json(payload): Json<UserForCreate>,
) {
    println!("{:?}", payload);


}