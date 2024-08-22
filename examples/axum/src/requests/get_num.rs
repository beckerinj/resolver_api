use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use super::Json;
use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(Json<GetNumResponse>)]
#[state(State)]
pub struct GetNum {}

#[derive(Serialize, Debug)]
pub struct GetNumResponse {
  pub num: u16,
}

impl Resolve for GetNum {
  async fn resolve(self, state: &State) -> Json<GetNumResponse> {
    Json(GetNumResponse { num: state.num })
  }
}
