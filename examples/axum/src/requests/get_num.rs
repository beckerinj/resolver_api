use axum::Json;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(Json<GetNumResponse>)]
#[args(State)]
pub struct GetNum {}

#[derive(Serialize, Debug)]
pub struct GetNumResponse {
  pub num: u16,
}

impl Resolve for GetNum {
  async fn resolve(self, args: &State) -> Json<GetNumResponse> {
    Json(GetNumResponse { num: args.num })
  }
}
