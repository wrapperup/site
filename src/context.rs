use std::future::Future;

use axum::{http::Request, middleware::Next, response::Response};
use tokio::sync::OnceCell;

#[derive(Clone)]
pub struct Context {
    pub name: String,
    pub age: i32,
}

tokio::task_local! {
    pub(crate) static CONTEXT: OnceCell<Context>;
}

pub async fn context_provider_layer<B>(request: Request<B>, next: Next<B>) -> Response {
    let context = Some(Context {
        name: "Hello".to_string(),
        age: 42,
    });

    // Set the context for this request.
    // This will be available in the template as `ctx`.
    provide_context(context, next.run(request)).await
}

pub async fn provide_context<F: Future<Output = O>, O>(context: Option<Context>, f: F) -> O {
    CONTEXT.scope(OnceCell::new_with(context), f).await
}

pub fn ctx() -> Option<Context> {
    CONTEXT.try_with(|ctx| ctx.get().cloned()).ok().flatten()
}
