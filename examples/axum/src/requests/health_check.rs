use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use super::Json;
use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(Json<HealthCheckResponse>)]
#[state(State)]
pub struct HealthCheck {}

#[derive(Serialize, Debug)]
pub struct HealthCheckResponse {}

impl Resolve for HealthCheck {
  async fn resolve(self, _: &State) -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {})
  }
}
