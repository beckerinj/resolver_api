use std::future::Future;

use serde::{de::DeserializeOwned, Serialize};

pub use resolver_api_derive as derive;

mod error;

pub use error::Error;

/// This trait is implemented on all Request structs.
/// It defines an associated response type for the Request.
pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug {
  type Response: Serialize + DeserializeOwned + std::fmt::Debug;
  fn req_type() -> &'static str;
  fn res_type() -> &'static str;
}

/// This trait is implemented on some State struct for all Request structs.
/// It defines how State resolves the response.
pub trait Resolve<
  Req: HasResponse + Send,
  Args: Send = (),
  Err: std::fmt::Debug = anyhow::Error,
> where
  Self: Send + Sync,
{
  fn resolve(
    &self,
    req: Req,
    args: Args,
  ) -> impl Future<Output = Result<Req::Response, Err>> + Send;

  fn resolve_response(
    &self,
    req: Req,
    args: Args,
  ) -> impl Future<Output = Result<String, Error<Err>>> + Send {
    async {
      let res = self.resolve(req, args).await.map_err(Error::Inner)?;
      let res = serde_json::to_string(&res).map_err(Error::Serialization)?;
      Ok(res)
    }
  }
}

/// Alternate trait to Resolve which skips auto serialization,
/// and allows the developer to return an already serialized response.
pub trait ResolveToString<Req: HasResponse, Args = (), Err: std::fmt::Debug = anyhow::Error> {
  fn resolve_to_string(
    &self,
    req: Req,
    args: Args,
  ) -> impl Future<Output = Result<String, Err>> + Send;
}

/// This trait is defined on master request enums using the [Resolver][derive::Resolver] macro.
pub trait Resolver<ReqEnum, Args = (), Err: std::fmt::Debug = anyhow::Error> {
  fn resolve_request(
    &self,
    request: ReqEnum,
    args: Args,
  ) -> impl Future<Output = Result<String, Error<Err>>> + Send;
}
