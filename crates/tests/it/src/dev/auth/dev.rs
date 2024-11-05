use http_body_util::BodyExt;
use hyper::body::Buf;
use tower::{Service, ServiceExt};

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use lib_core::model::user::{UserForCreate, UserForLogin, UserForSignIn};
    use crate::context::context::TestContext;

    #[tokio::test]
    async fn nils_second_att() {
        let mut ctx = TestContext::new().await;
        let user_to_create = UserForCreate::new("2128506", "pwd", "John", "Doe");
        let response = ctx.create_user(&user_to_create).await;
        assert_eq!(response.status(), StatusCode::OK);

        //let response = ctx.create_user(&user_to_create).await;
        //println!("{:?}", &response);

        assert_eq!(response.status(), StatusCode::OK);

        let user_body = UserForSignIn::new("2128506", "pwd");

        let auth_token = ctx.sign_in_user(user_body).await;
        assert!(auth_token.is_some());
        ctx.get_user_by_id(1, auth_token.unwrap()).await;
    }

}