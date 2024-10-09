use std::io::prelude::*;
use std::sync::Arc;

use axum::extract;
use tokio_postgres::Client;

use common_pet_structs::user::user::User;

use crate::repository::book::book_repository::BookRepository;
use crate::repository::user::user_repository::UserRepository;

pub(crate) struct AppContext {
    client: Client,
}

impl AppContext {
    pub(crate) async fn create(client: Arc<Client>) -> AppContext {
        let created = AppContext {
            book_repository: BookRepository::new(client.clone()),
            user_repository: UserRepository::new(client.clone()),
        };

        created
    }

    pub(crate) fn user_repository(&self) -> &UserRepository {
        &self.user_repository
    }

    pub(crate) fn book_repository(&self) -> &BookRepository {
        &self.book_repository
    }
}