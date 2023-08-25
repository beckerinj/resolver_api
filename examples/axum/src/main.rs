use std::{net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Context;
use axum::{
    headers::ContentType, http::StatusCode, routing::post, Extension, Json, Router, TypedHeader,
};
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

    axum::Server::bind(&SocketAddr::from_str("127.0.0.1:5555")?)
        .serve(app.into_make_service())
        .await
        .context("server crashed")?;

    Ok(())
}
