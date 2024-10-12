
#[derive(Debug)]
pub struct UserForCreate {
    identity: UserIdentity,
    password: String,
    first_name: String,
    last_name: String,
}

pub struct UserStored {
    id: i64,
    identity: UserIdentity,
}

#[derive(Debug)]
pub enum UserIdentity {
    Phone(String),
    Email(String),
}