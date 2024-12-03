use http_body_util::BodyExt;
use hyper::body::Buf;
use tower::{Service, ServiceExt};

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use hyper::body::Buf;
    use tower::{Service, ServiceExt};
    use axum::http::StatusCode;
    use serde_json::Value;
    use lib_core::model::user::{UserForCreate, UserForLogin, UserForSignIn};
    use crate::context::context::TestContext;
    use crate::utils::body_utils::get_message;

    #[tokio::test]
    async fn nils_second_att() {
        let mut ctx = TestContext::new().await;

        // when no token then forbidden
        let no_token_response = ctx.get_user_response_by_id(1).await;
        assert_eq!(no_token_response.status(), StatusCode::FORBIDDEN);
        let body = no_token_response.collect().await.unwrap().aggregate();
        let no_token_result: Value = serde_json::from_reader(body.reader()).unwrap();
        let message = get_message(no_token_result);
        assert_eq!(message, "NO_AUTH");

        // when new user then OK
        let user_to_create = UserForCreate::new("2128506", "pwd", "John", "Doe");
        let response = ctx.create_user(&user_to_create).await;
        assert_eq!(response.status(), StatusCode::OK);

        // when existing user then 400
        let already_exists_response = ctx.create_user(&user_to_create).await;
        println!("error: {:?}", &already_exists_response);
        assert_eq!(already_exists_response.status(), StatusCode::BAD_REQUEST);
        let body = already_exists_response.collect().await.unwrap().aggregate();
        let bad_request_result: Value = serde_json::from_reader(body.reader()).unwrap();
        let message = get_message(bad_request_result);
        assert_eq!(message, "USER_EXISTS");

        let user_body = UserForSignIn::new("2128506", "pwd");
        let sign_in_response = ctx.sign_in_user(user_body).await;
        assert_eq!(response.status(), StatusCode::OK);
        let auth_token = ctx.get_auth_cookie(sign_in_response).await;
        assert!(auth_token.is_some());

        let user_body = UserForSignIn::new("2128506", "wrong password");
        let sign_in_response = ctx.sign_in_user(user_body).await;
        assert_eq!(response.status(), StatusCode::OK);
        let auth_token = ctx.get_auth_cookie(sign_in_response).await;
        assert!(auth_token.is_none());

        ctx.get_user_by_id(1).await;
    }

}