use axum::{http::StatusCode, Json};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(Json<HealthCheckResponse>)]
#[error(StatusCode)]
pub struct HealthCheck {}

#[derive(Serialize, Debug)]
pub struct HealthCheckResponse {}

impl Resolve<State> for HealthCheck {
  async fn resolve(self, _: &State) -> Result<Json<HealthCheckResponse>, Self::Error> {
    Ok(Json(HealthCheckResponse {}))
  }
}
