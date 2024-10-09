use std::io::prelude::*;
use std::sync::Arc;

use axum::extract;
use tokio_postgres::Client;


pub struct AppContext {
    client: Arc<Client>,
}

impl AppContext {
    pub async fn create(client: Arc<Client>) -> AppContext {
        AppContext {
            client
        }
    }
}