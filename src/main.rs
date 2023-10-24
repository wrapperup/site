use std::time::Duration;

use askama_axum::Template;
use axum::{
    middleware::{self},
    routing::get,
    Router,
};
use context::context_provider_layer;
use tower_livereload::LiveReloadLayer;

mod context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index_handler))
        .layer(middleware::from_fn(context_provider_layer)) // Add our middleware, which sets the context
        .layer(LiveReloadLayer::new().reload_interval(Duration::from_millis(100)));

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
