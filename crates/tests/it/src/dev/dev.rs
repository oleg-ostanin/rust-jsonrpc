use http_body_util::BodyExt;
use hyper::body::Buf;
use tower::{Service, ServiceExt};

#[cfg(test)]
mod tests {
    use lib_core::model::user::{UserForCreate, UserForLogin, UserForSignIn};
    use crate::context::context::TestContext;

    #[tokio::test]
    async fn nils_second_att() {
        let ctx = TestContext::new().await;
        let user_body = UserForCreate::new("2128506", "pwd", "John", "Doe");
        ctx.create_user(user_body).await;
        let user_body = UserForSignIn::new("2128506", "pwd");

        let auth_token = ctx.sign_in_user(user_body).await;
        assert!(auth_token.is_some());
        ctx.get_user_by_id(1, auth_token.unwrap()).await;
    }

}