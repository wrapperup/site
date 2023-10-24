use std::time::Duration;

use askama_axum::Template;
use axum::{
    middleware::{self},
    response::Html,
    routing::get,
    Router,
};
use context::{context, context_provider_layer, Context};
use tower_livereload::LiveReloadLayer;

use crate::context::CONTEXT;

mod context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index_handler))
        .layer(middleware::from_fn(context_provider_layer)) // Add our middleware, which sets the context
        .layer(LiveReloadLayer::new().reload_interval(Duration::from_millis(100)));

    match context() {
        Some(x) => {
            println!("Context: {}, {}", x.name, x.age);
        }
        _ => {
            println!("No context");
        }
    };

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

async fn index_handler() -> Index {
    Index {}
}
