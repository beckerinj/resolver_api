use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response((TypedHeader<ContentType>, String))]
pub struct GetString {}

impl Resolve<State> for GetString {
  async fn resolve(self, state: &State) -> (TypedHeader<ContentType>, String) {
    // This could be pulled out of a cache of serialized responses
    (TypedHeader(ContentType::json()), state.json_string.clone())
  }
}
