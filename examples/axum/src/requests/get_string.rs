use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(String)]
#[state(State)]
pub struct GetString {}

impl Resolve for GetString {
  async fn resolve(self, _state: &State) -> String {
    // This could be pulled out of a cache of serialized responses
    String::from("{\"value\":14}")
  }
}
