use std::future::Future;

extern crate resolver_api_derive;
pub use resolver_api_derive::Resolve;

pub trait HasResponse {
  type State;
  type Response;

  fn req_type() -> &'static str;
  fn res_type() -> &'static str;
}

pub trait Resolve: HasResponse {
  fn resolve(self, state: &Self::State) -> impl Future<Output = Self::Response>;
}
