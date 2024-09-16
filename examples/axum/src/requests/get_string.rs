use resolver_api::Resolve;
use serde::Deserialize;

use crate::State;

#[derive(Deserialize, Debug, Resolve)]
#[response(String)]
pub struct GetString {}

impl Resolve<State> for GetString {
  async fn resolve(self, state: &State) -> String {
    // This could be pulled out of a cache of serialized responses
    state.string.to_string()
  }
}
