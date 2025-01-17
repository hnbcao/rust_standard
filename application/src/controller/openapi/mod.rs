use salvo::{handler, Router};

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

pub(crate) fn router() -> Router {
    let router = Router::new().path("open").get(hello);
    router
}
