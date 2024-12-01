use http_body_util::BodyExt;
use serde_json::to_string;
use tokio_postgres::types::ToSql;
use uuid::Uuid;
use lib_auth::pwd;
use lib_auth::pwd::{ContentToHash, hash_pwd};
use lib_utils::b64::b64u_encode;

use lib_auth::pwd::scheme::hash::hash;

use crate::context::app_context::ModelManager;
use crate::model::user::{UserForAuth, UserForCreate, UserForLogin, UserStored};

use super::{Error, Result};

pub struct UserBmc;
//todo return identity type
const INSERT_USER: &str = r#"
INSERT INTO users
(identity, first_name, last_name, pwd
  , pwd_salt, token_salt
  --, created_at, updated_at todo
)
VALUES
($1, $2, $3, $4
  , $5, $6
  -- , $7, $8 todo
)
RETURNING id;
"#;

const SELECT_BY_ID: &str = r#"
SELECT * FROM users WHERE id=$1;
"#;

const SELECT_BY_IDENTITY: &str = r#"
SELECT * FROM users WHERE identity=$1;
"#;

impl UserBmc {
    pub async fn create(
        mm: &ModelManager,
        user: UserForCreate,
    ) -> Result<()> {
        let pwd_salt = Uuid::new_v4();
        let to_hash = ContentToHash {
            content: user.password.clone(),
            salt: pwd_salt,
        };

        let pwd_hashed = hash_pwd(to_hash).await?;

        let token_salt = Uuid::new_v4().to_string();

        mm.client().execute(INSERT_USER, &[&user.identity, &user.first_name,
            &user.last_name, &pwd_hashed, &pwd_salt.to_string(), &token_salt]).await?;

        Ok(())
    }

    pub async fn get_by_id(
        mm: &ModelManager,
        id: i64,
    ) -> Result<UserStored> {
        //let res = db_client.execute(&statement, &[&user.uuid, &user.pass]).await?;
        let res = mm.client().query(SELECT_BY_ID, &[&id]).await?;

        println!("{:?}", &res);

        let v = res.get(0).ok_or(Error::StoreError("not_found".to_string()))?;

        UserStored::try_from(v)
    }

    pub async fn get_for_auth(
        mm: &ModelManager,
        identity: &String,
    ) -> Result<UserForAuth> {
        //let res = db_client.execute(&statement, &[&user.uuid, &user.pass]).await?;
        let res = mm.client().query(SELECT_BY_IDENTITY, &[identity]).await?;

        println!("{:?}", &res);

        let v = res.get(0).ok_or(Error::StoreError("not_found".to_string()))?;

        UserForAuth::try_from(v)
    }

    // todo make these two functions generic
    pub async fn get_for_login(
        mm: &ModelManager,
        identity: &String,
    ) -> Result<UserForLogin> {
        //let res = db_client.execute(&statement, &[&user.uuid, &user.pass]).await?;
        let res = mm.client().query(SELECT_BY_IDENTITY, &[identity]).await?;

        println!("{:?}", &res);

        let v = res.get(0).ok_or(Error::StoreError("not_found".to_string()))?;

        UserForLogin::try_from(v)
    }
}

// fn to_params(user: UserForCreate) -> &[&dyn ToSql + Sync] {
//     unimplemented!()
// }