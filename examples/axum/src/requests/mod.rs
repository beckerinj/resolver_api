use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

use self::{get_num::GetNum, health_check::HealthCheck};

mod get_num;
mod get_string;
mod health_check;

pub struct Response(pub axum::response::Response);

impl<T> From<T> for Response
where
  T: axum::response::IntoResponse,
{
  fn from(value: T) -> Self {
    Response(value.into_response())
  }
}

#[derive(Deserialize, Resolve)]
#[response(Response)]
#[args(State)]
pub enum Request {
  HealthCheck(HealthCheck),
  GetNum(GetNum),
}
