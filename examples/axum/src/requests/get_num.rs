use async_trait::async_trait;
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Serialize, Deserialize, Debug, Request)]
#[response(GetNumResponse)]
pub struct GetNum {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNumResponse {
  pub num: u16,
}

#[async_trait]
impl Resolve<GetNum> for State {
  async fn resolve(&self, _: GetNum, _: ()) -> anyhow::Result<GetNumResponse> {
    Ok(GetNumResponse { num: self.num })
  }
}
