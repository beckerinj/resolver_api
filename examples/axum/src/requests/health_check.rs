use axum::Json;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(Json<HealthCheckResponse>)]
#[args(State)]
pub struct HealthCheck {}

#[derive(Serialize, Debug)]
pub struct HealthCheckResponse {}

impl Resolve for HealthCheck {
  async fn resolve(self, _: &State) -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {})
  }
}
