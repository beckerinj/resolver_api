use resolver_api::{derive::Request, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Serialize, Deserialize, Debug, Request)]
#[response(ToStringTestResponse)]
pub struct ToStringTest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToStringTestResponse {
  pub value: u16,
}

impl ResolveToString<ToStringTest> for State {
  async fn resolve_to_string(&self, _: ToStringTest, _: ()) -> anyhow::Result<String> {
    // This could be pulled out of a cache of serialized responses
    Ok(String::from("{\"value\":14}"))
  }
}
