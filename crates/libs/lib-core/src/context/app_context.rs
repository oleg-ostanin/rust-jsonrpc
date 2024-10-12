use std::io::prelude::*;
use std::sync::Arc;

use axum::extract;
use tokio_postgres::Client;


#[derive(Clone)]
pub struct ModelManager {
    client: Arc<Client>,
}

impl ModelManager {
    pub async fn create(client: Arc<Client>) -> ModelManager {
        ModelManager {
            client
        }
    }
}