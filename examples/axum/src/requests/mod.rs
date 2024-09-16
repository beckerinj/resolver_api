use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

mod get_num;
mod get_string;
mod health_check;

pub struct Response {
  pub response: axum::response::Response,
}

impl<T> From<T> for Response
where
  T: axum::response::IntoResponse,
{
  fn from(value: T) -> Self {
    Response {
      response: value.into_response(),
    }
  }
}

#[derive(Deserialize, Resolve)]
#[response(Response)]
#[args(State)]
pub enum Request {
  HealthCheck(health_check::HealthCheck),
  GetNum(get_num::GetNum),
  GetString(get_string::GetString),
}
