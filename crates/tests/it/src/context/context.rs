use core::net::SocketAddr;
use std::sync::Arc;

use axum::{
    body::Body
    ,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt;
use hyper::body::Buf;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
// for `collect`
use serde_json::json;
use testcontainers::{clients, Container, images::postgres::Postgres};
use tokio::net::TcpListener;
use tokio_postgres::NoTls;

use lib_core::context::app_context::ModelManager;
use lib_web::app::app::create_app_context;
use lib_web::app::app::app_nils;

use tower_cookies::{Cookie, Cookies};
use crate::context::sql::{CREATE_IDENTITY_TYPE, CREATE_USER_TABLE};
// for `call`, `oneshot`, and `ready`

pub(crate) struct TestContext<> {
    docker: &'static clients::Cli,
    pg_container: &'static(Container<'static, Postgres>),
    client: Client<HttpConnector, Body>,
    socket_addr: SocketAddr,
}

impl TestContext {
    pub(crate) async fn new() -> Self {
        let docker: &'static clients::Cli = Box::leak(Box::new(clients::Cli::default()));

        // Define a PostgreSQL container image
        let postgres_image = Postgres::default();

        let pg_container = docker.run(postgres_image);

        let pg_container: &'static(Container<Postgres>) = Box::leak(Box::new(pg_container));

        pg_container.start();

        // Get the PostgreSQL port
        let pg_port = pg_container.get_host_port_ipv4(5432);

        // Define the connection to the Postgress client
        let (pg_client, connection) = tokio_postgres::Config::new()
            .user("postgres")
            .password("postgres")
            .host("localhost")
            .port(pg_port)
            .dbname("postgres")
            .connect(NoTls)
            .await
            .unwrap();

        // Spawn connection
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                eprintln!("Connection error: {}", error);
            }
        });

        init_db(&pg_client).await;

        let app_context: Arc<ModelManager> = Arc::new(ModelManager::create(Arc::new(pg_client)).await);

        let app = app_nils(app_context).await;

        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let socket_addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let client: Client<HttpConnector, Body> =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        Self {
            docker,
            pg_container,
            client,
            socket_addr,
        }
    }

    pub(crate) async fn get_books(&self) -> () {
        let addr = &self.socket_addr;

        let mut cookie = Cookie::new("AUTH_TOKEN", "token".to_string());

        let get_response = self.client
            .request(Request::builder()
                .method(http::Method::GET)
                .uri(format!("http://{addr}/get-books"))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .header("cookie", "AUTH_TOKEN=token".to_string())

                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();

        assert_eq!(get_response.status(), StatusCode::OK);
    }
}

async fn init_db(pg_client: &tokio_postgres::Client) {
    pg_client.execute(CREATE_IDENTITY_TYPE, &[]).await.unwrap();
    pg_client.execute(CREATE_USER_TABLE, &[]).await.unwrap();
}