use std::str::FromStr;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio_postgres::{Error, Row};
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use crate::model::store;

#[derive(Debug, Deserialize, Serialize, Builder)]
pub struct UserForCreate {
    pub identity: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserForCreate {
    pub fn new(
        identity: impl Into<String>,
        password: impl Into<String>,
        first_name: impl Into<String>,
        last_name: impl Into<String>,
    ) -> Self {
        UserForCreate {
            identity: identity.into(),
            password: password.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserForSignIn {
    pub identity: String,
    pub password: String,
}

impl UserForSignIn {
    pub fn new(
        identity: impl Into<String>,
        password: impl Into<String>,

    ) -> Self {
        UserForSignIn {
            identity: identity.into(),
            password: password.into(),
        }
    }
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub identity: String,

    // -- token info
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub identity: String,
    pub pwd: Option<String>,

    // -- pwd info
    pub pwd_salt: Uuid,
    // -- token info
    pub token_salt: Uuid,
}

// impl<'user> TryFrom<&'user UserForCreate<'user>> for &'user [&'user (dyn ToSql + Sync)] {
// //impl<'user> TryFrom<&'user [&'user (dyn ToSql + Sync)]> for &'user [&'user (dyn ToSql + Sync)] {
//     type Error = store::Error;
//
//     fn try_from(value: &'user UserForCreate) -> Result<Self, Self::Error> {
//         Ok(&[
//             &value.identity,
//             &value.first_name,
//             &value.last_name,
//         ])
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct UserStored {
    pub id: i64,
    pub identity: String,
    pub first_name: String,
    pub last_name: String,
    pub pwd: String, // todo remove
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<&Row> for UserStored {
    type Error = store::Error;

    fn try_from(row: &Row) -> Result<Self, store::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            identity: row.try_get("identity")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            pwd: row.try_get("pwd")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl TryFrom<&Row> for UserForAuth {
    type Error = store::Error;

    fn try_from(row: &Row) -> Result<Self, store::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            identity: row.try_get("identity")?,
            token_salt: Uuid::parse_str(row.try_get("token_salt")?).unwrap(),
        })
    }
}

impl TryFrom<&Row> for UserForLogin {
    type Error = store::Error;

    fn try_from(row: &Row) -> Result<Self, store::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            identity: row.try_get("identity")?,
            pwd: row.try_get("pwd")?,
            pwd_salt: Uuid::parse_str(row.try_get("pwd_salt")?).unwrap(),
            token_salt: Uuid::parse_str(row.try_get("token_salt")?).unwrap(),
        })
    }
}

#[derive(Debug)]
pub enum UserIdentity {
    Phone(String),
    Email(String),
}