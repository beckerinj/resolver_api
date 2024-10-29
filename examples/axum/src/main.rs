use std::sync::Arc;

use anyhow::Context;
use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use requests::Request;
use resolver_api::Resolve;

mod requests;

pub struct State {
  pub num: u16,
  pub json_string: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let state = State {
    num: 43,
    json_string: String::from("{\"is_json\":true}"),
  };

  let app = Router::new()
    .route(
      "/",
      post(
        |state: Extension<Arc<State>>, Json(req): Json<Request>| async move {
          match req.resolve(&state).await {
            Ok(res) => res.response,
            Err(err) => err.into_response(),
          }
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
