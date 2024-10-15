use tokio_postgres::types::ToSql;
use crate::context::app_context::ModelManager;
use crate::model::user::{UserForCreate, UserStored};

use super::{Error, Result};

pub struct UserBmc;
//todo return identity type
const INSERT_USER: &str = r#"
INSERT INTO users
(identity, first_name, last_name, pwd
  --, pwd_salt, token_salt
  --, created_at, updated_at todo
)
VALUES
($1, $2, $3, $4
  -- , $5, $6
  -- , $7, $8 todo
)
RETURNING id;
"#;

const SELECT_BY_ID: &str = r#"
SELECT * FROM users WHERE id=$1;
"#;

impl UserBmc {

    pub async fn create(
        mm: &ModelManager,
        user: UserForCreate,
    ) -> Result<u64> {
        //let res = db_client.execute(&statement, &[&user.uuid, &user.pass]).await?;
        let res = mm.client().execute(INSERT_USER, &[&user.identity, &user.first_name,
            &user.last_name, &user.password, ]).await;

        // todo propagate error
        let result = match res {
            Ok(val) => {
                println!("{:?}", val);
                Ok(val)
            }
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        };

        Ok(0)
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
}

// fn to_params(user: UserForCreate) -> &[&dyn ToSql + Sync] {
//     unimplemented!()
// }