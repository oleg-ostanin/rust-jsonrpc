use http_body_util::BodyExt;
use hyper::body::Buf;
use tower::{Service, ServiceExt};

#[cfg(test)]
mod tests {
    use crate::context::context::TestContext;

    #[tokio::test]
    async fn nils_second_att() {
        let ctx = TestContext::new().await;
        let books = ctx.get_books().await;
        ctx.create_book().await;
    }

}