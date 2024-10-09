#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use axum::{extract::{Json, State}, Router, routing::{get, post}};
use axum::http::StatusCode;
use java_properties::read;
use tokio_postgres::{Client, NoTls};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use common_pet_structs::book::book::{Book, Books};
use common_pet_structs::user::user::User;
use repository::user::user_repository::UserRepository;

use crate::context::app_context::AppContext;
use crate::repository::book::book_repository::BookRepository;

mod service;
mod context;
mod repository;
mod test;

#[tokio::main]
async fn main() {
    let db_url = read_db_url("local.properties");
    let client = get_client(db_url).await;

    let app_context: Arc<AppContext> = Arc::new(AppContext::create(Arc::new(client)).await);

    app_context.user_repository().init().await;
    app_context.book_repository().init().await;

    let app = app_nils(app_context).await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn app_nils(app_context: Arc<AppContext>) -> Router {
    Router::new()
        .route("/create-user", post(create_user))
        .route("/create-book", post(create_book))
        .route("/get-books", get(get_books))
        .with_state(app_context)
}

async fn create_user(
    State(app_context): State<Arc<AppContext>>,
    Json(payload): Json<User>,
) {
    println!("{:?}", payload);

    let mut user_repo: &UserRepository = app_context.user_repository();

    user_repo.insert_user(payload).await;
}

async fn create_book(
    State(app_context): State<Arc<AppContext>>,
    Json(payload): Json<Book>,
) {
    println!("{:?}", payload);

    let mut book_repo: &BookRepository = app_context.book_repository();

    let res = book_repo.insert_book(payload).await.unwrap();

    println!("{:?}", res)
}

async fn get_books(
    State(app_context): State<Arc<AppContext>>,
) -> Result<Json<Books>, StatusCode> {
    println!("{:?}", "get books");

    let mut book_repo: &BookRepository = app_context.book_repository();

    Ok(Json::from(book_repo.get_books().await.unwrap()))
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

