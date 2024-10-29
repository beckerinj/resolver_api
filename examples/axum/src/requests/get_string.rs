use axum::http::StatusCode;
use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response((TypedHeader<ContentType>, String))]
#[error(StatusCode)]
pub struct GetString {}

impl Resolve<State> for GetString {
  async fn resolve(self, state: &State) -> Result<(TypedHeader<ContentType>, String), Self::Error> {
    // This could be pulled out of a cache of serialized responses
    Ok((TypedHeader(ContentType::json()), state.json_string.clone()))
  }
}
