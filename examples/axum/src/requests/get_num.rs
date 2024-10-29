use axum::{http::StatusCode, Json};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(Json<GetNumResponse>)]
#[error(StatusCode)]
pub struct GetNum {}

#[derive(Serialize, Debug)]
pub struct GetNumResponse {
  pub num: u16,
}

impl Resolve<State> for GetNum {
  async fn resolve(self, state: &State) -> Result<Json<GetNumResponse>, StatusCode> {
    Ok(Json(GetNumResponse { num: state.num }))
  }
}
