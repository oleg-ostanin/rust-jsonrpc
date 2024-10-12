use tokio_postgres::types::ToSql;
use crate::context::app_context::ModelManager;
use crate::model::user::UserForCreate;

use super::Result;

pub struct UserBmc;
//todo return identity type
const INSERT_USER: &str = r#"
INSERT INTO users
(identity, first_name, last_name, pwd, pwd_salt, token_salt
  --, created_at, updated_at todo
)
VALUES
($1, $2, $3, $4, $5, $6
  -- , $7, $8 todo
)
RETURNING id;
"#;

impl UserBmc {

    pub async fn create(
        mm: &ModelManager,
        user: UserForCreate,
    ) -> Result<u64> {
        //let res = db_client.execute(&statement, &[&user.uuid, &user.pass]).await?;
        let res = Ok(mm.client().execute(INSERT_USER, &[&user.identity, &user.first_name, &user.last_name, &user.password,
        &"salt", &"token_salt"])
            .await?);

        println!("{:?}", res);

        res
    }
}

// fn to_params(user: UserForCreate) -> &[&dyn ToSql + Sync] {
//     unimplemented!()
// }