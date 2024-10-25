use std::str::FromStr;
use std::time::SystemTime;
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio_postgres::{Error, Row};
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use crate::model::store;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserForCreate {
    pub identity: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}


impl UserForCreate {
    pub fn new(
        identity: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> Self {
        UserForCreate {
            identity,
            password,
            first_name,
            last_name,
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
}

impl TryFrom<&Row> for UserStored {
    type Error = store::Error;

    fn try_from(row: &Row) -> Result<Self, store::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            identity: row.try_get("identity")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
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

#[derive(Debug)]
pub enum UserIdentity {
    Phone(String),
    Email(String),
}