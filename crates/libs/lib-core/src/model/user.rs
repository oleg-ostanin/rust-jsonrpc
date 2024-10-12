use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use crate::model::store;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserForCreate {
    identity: String,
    password: String,
    first_name: String,
    last_name: String,
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

pub struct UserStored {
    id: i64,
    identity: UserIdentity,
}

#[derive(Debug)]
pub enum UserIdentity {
    Phone(String),
    Email(String),
}