use salvo::{handler, Router};

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

pub(crate) fn router() -> Router {
    let router = Router::with_path("open").get(hello);
    router
}