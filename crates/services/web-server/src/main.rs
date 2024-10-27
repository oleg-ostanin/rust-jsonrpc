use std::sync::Arc;
use axum::Error;
use tracing_subscriber::EnvFilter;
use lib_core::context::app_context::ModelManager;
use lib_web::app::app::create_app_context;
use lib_web::app::app::app_nils;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
      .without_time() // For early local development.
      .with_target(false)
      .with_env_filter(EnvFilter::from_default_env())
      .init();

  let app_context: Arc<ModelManager> = create_app_context().await;

  let app = app_nils(app_context).await;

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}