use salvo::{handler, Router};

mod user_api;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

pub(crate) fn router() -> Router {
    Router::new().get(hello).push(user_api::router())
}
