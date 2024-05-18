use std::sync::Arc;

use anyhow::Context;
use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use requests::Request;
use resolver_api::Resolver;

mod requests;

pub struct State {
  pub num: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let state = State { num: 43 };

  let app = Router::new()
    .route(
      "/",
      post(
        |state: Extension<Arc<State>>, Json(req): Json<Request>| async move {
          let res = state
            .resolve_request(req, ())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")))?;
          Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
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
