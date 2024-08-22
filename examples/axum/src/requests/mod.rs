use axum::response::IntoResponse;
use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

use self::{get_num::GetNum, health_check::HealthCheck};

mod get_num;
mod health_check;

pub struct Json<T>(T);

impl<T> From<Json<T>> for axum::response::Response
where
  T: serde::Serialize,
{
  fn from(val: Json<T>) -> Self {
    axum::Json(val.0).into_response()
  }
}

#[derive(Deserialize, Resolve)]
#[response(axum::response::Response)]
#[state(State)]
pub enum Request {
  HealthCheck(HealthCheck),
  GetNum(GetNum),
}
