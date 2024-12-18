pub mod user;

use serde::{Deserialize, Serialize};

// region:    --- Modules

pub(in crate::model) mod dbx;




use chrono::{DateTime, Utc, NaiveDateTime};
// use crate::core_config;
use tokio_postgres::types::{FromSql, Type};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use lib_auth::pwd;
// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> sqlx::Result<Db> {
    // * See NOTE 1) below
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect("&core_config().DB_URL")
        .await
}

// NOTE 1) This is not an ideal situation; however, with sqlx 0.7.1, when executing `cargo test`, some tests that use sqlx fail at a
//         rather low level (in the tokio scheduler). It appears to be a low-level thread/async issue, as removing/adding
//         tests causes different tests to fail. The cause remains uncertain, but setting max_connections to 1 resolves the issue.
//         The good news is that max_connections still function normally for a regular run.
//         This issue is likely due to the unique requirements unit tests impose on their execution, and therefore,
//         while not ideal, it should serve as an acceptable temporary solution.
//         It's a very challenging issue to investigate and narrow down. The alternative would have been to stick with sqlx 0.6.x, which
//         is potentially less ideal and might lead to confusion as to why we are maintaining the older version in this blueprint.



pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    EntityExists,
    StoreError(String),
    NotFound,
    FailedToHash,
    Undefined,
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        println!("{:?}", &value);

        if let Some(db_error) = value.as_db_error() {
            let message = db_error.message();
            if message.contains("violates unique constraint") {
                return Error::StoreError("Entity already exists".to_string())
            }
        };
        Error::StoreError("undefined".to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Error::StoreError(value.to_string())
    }
}

impl From<pwd::Error> for Error {
    fn from(_: pwd::Error) -> Self {
        Error::FailedToHash
    }
}