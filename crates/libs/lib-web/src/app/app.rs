#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::sync::Arc;

use axum::{extract::{Json, State}, middleware, Router, routing::{get, post}};
use axum::http::StatusCode;
use java_properties::read;
use tokio_postgres::{Client, NoTls};
use tower_cookies::{CookieManagerLayer, Cookies};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib_core::context::app_context::ModelManager;
use lib_core::model::user::UserForCreate;
use crate::middleware::mw_auth::mw_ctx_resolver;
use super::super::handlers::signup::sign_up;
use super::super::handlers::admin::get_by_id;

pub async fn create_app_context() -> Arc<ModelManager> {
    let db_url = read_db_url("local.properties");
    let client = get_client(db_url).await;

    let app_context: Arc<ModelManager> = Arc::new(ModelManager::create(Arc::new(client)).await);

    app_context
}

pub async fn app_nils(app_context: Arc<ModelManager>) -> Router {
    Router::new()
        .route("/get-books", get(get_books))
        .route("/sign-up", post(sign_up))
        .route("/get-by-id/:user_id", get(get_by_id))
        .layer(middleware::from_fn_with_state(app_context.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .with_state(app_context)
}


async fn get_books(
    State(app_context): State<Arc<ModelManager>>,
    cookies: Cookies,
) -> Result<String, StatusCode> {
    println!("{:?}", "get books");

    let token = cookies.get("AUTH_TOKEN");

    println!("{:?}", token);

    Ok("res".to_string())
}

pub async fn sign_up3(
    State(app_context): State<Arc<ModelManager>>,
    cookies: Cookies,
    Json(payload): Json<UserForCreate>,
) -> Result<String, StatusCode> {
    println!("{:?}", payload);

    Ok("ok".to_string())
}

async fn get_client(db_url: String) -> Client {
    //Unwrap because if we can't connect we must fail at once
    let (client, connection) =
        tokio_postgres::connect(db_url.as_str(), NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

fn read_db_url(path: &str) -> String {

    // Reading
    let f = File::open(path).unwrap();
    let map2 = read(BufReader::new(f)).unwrap();
    let db_url = map2.get("db.url").unwrap().to_string();
    db_url
}

