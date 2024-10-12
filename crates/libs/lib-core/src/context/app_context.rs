use std::io::prelude::*;
use std::ops::Deref;
use std::sync::Arc;

use axum::extract;
use tokio_postgres::Client;


pub struct ModelManager {
    client: Arc<Client>,
}

impl ModelManager {
    pub async fn create(client: Arc<Client>) -> ModelManager {
        ModelManager {
            client
        }
    }

    pub fn client(&self) -> &Client {
        self.client.deref()
    }
}