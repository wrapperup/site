use std::{future::Future, sync::Arc};

use axum::{http::Request, middleware::Next, response::Response};

#[derive(Clone)]
pub struct Context {
    pub name: String,
    pub age: i32,
}

tokio::task_local! {
    pub(crate) static CONTEXT: Context;
}

pub async fn context_provider_layer<B>(request: Request<B>, next: Next<B>) -> Response {
    let context = Context {
        name: "Hello".to_string(),
        age: 42,
    };

    // Set the context for this request.
    // This will be available in the template as `ctx`.
    provide_context(context, next.run(request)).await
}

pub async fn provide_context<F: Future<Output = O>, O>(context: Context, f: F) -> O {
    CONTEXT.scope(context, f).await
}

pub fn context<'a>() -> Option<&'a Context> {
    match CONTEXT.try_with(|c| c as *const Context) {
        Ok(ctx) => Some(unsafe { &*ctx }),
        Err(_) => None,
    }
}
