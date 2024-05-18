use resolver_api_derive::Resolver;
use serde::Deserialize;

use crate::State;

use self::{get_num::GetNum, health_check::HealthCheck, to_string::ToStringTest};

mod get_num;
mod health_check;
mod to_string;

#[derive(Deserialize, Resolver)]
#[resolver_target(State)]
pub enum Request {
  HealthCheck(HealthCheck),
  GetNum(GetNum),
  #[to_string_resolver]
  ToStringTest(ToStringTest),
}
