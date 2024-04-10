use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub use resolver_api_derive as derive;

mod error;

pub use error::Error;

pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static {
  type Response: Serialize + DeserializeOwned + std::fmt::Debug;
  fn req_type() -> &'static str;
  fn res_type() -> &'static str;
}

#[async_trait]
pub trait Resolve<Req: HasResponse, Args: Send + 'static = (), Err: std::fmt::Debug = anyhow::Error>
{
  async fn resolve(&self, req: Req, args: Args) -> Result<Req::Response, Err>;
  async fn resolve_response(&self, req: Req, args: Args) -> Result<String, Error<Err>> {
    let res = self.resolve(req, args).await.map_err(Error::Inner)?;
    let res = serde_json::to_string(&res).map_err(Error::Serialization)?;
    Ok(res)
  }
}

#[async_trait]
pub trait ResolveToString<Req: HasResponse, Args = (), Err: std::fmt::Debug = anyhow::Error> {
  async fn resolve_to_string(&self, req: Req, args: Args) -> Result<String, Err>;
}

#[async_trait]
pub trait Resolver<ReqEnum, Args = (), Err: std::fmt::Debug = anyhow::Error> {
  async fn resolve_request(&self, request: ReqEnum, args: Args) -> Result<String, Error<Err>>;
}
