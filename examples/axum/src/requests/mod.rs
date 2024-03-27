use resolver_api::{derive::Resolver, Resolve};
use serde::Deserialize;

use crate::State;

use self::{get_num::GetNum, health_check::HealthCheck};

mod get_num;
mod health_check;

#[derive(Deserialize, Resolver)]
#[resolver_target(State)]
pub enum Request {
  HealthCheck(HealthCheck),
  GetNum(GetNum),
}
