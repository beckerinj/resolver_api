use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Serialize, Deserialize, Debug, Request)]
#[response(HealthCheckResponse)]
pub struct HealthCheck {}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthCheckResponse {}

impl Resolve<HealthCheck> for State {
  async fn resolve(&self, _: HealthCheck, _: ()) -> anyhow::Result<HealthCheckResponse> {
    Ok(HealthCheckResponse {})
  }
}
