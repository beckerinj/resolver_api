use std::sync::Arc;

use anyhow::Context;
use axum::{routing::post, Extension, Json, Router};
use requests::Request;
use resolver_api::Resolve;

mod requests;

pub struct State {
  pub num: u16,
  pub string: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let state = State {
    num: 43,
    string: String::from("rando"),
  };

  let app = Router::new()
    .route(
      "/",
      post(
        |state: Extension<Arc<State>>, Json(req): Json<Request>| async move {
          req.resolve(&state).await.response
        },
      ),
    )
    .layer(Extension(Arc::new(state)));

  let listener = tokio::net::TcpListener::bind("127.0.0.1:5555")
    .await
    .context("failed to bind to tcp listener")?;

  axum::serve(listener, app).await.context("server crashed")?;

  Ok(())
}
