use anyhow::Context;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub use resolver_api_derive as derive;

pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static {
  type Response: Serialize + DeserializeOwned + std::fmt::Debug;
  fn req_type() -> &'static str;
  fn res_type() -> &'static str;
}

#[async_trait]
pub trait Resolve<Req: HasResponse, Args: Send + 'static = ()> {
  async fn resolve(&self, req: Req, args: Args) -> anyhow::Result<Req::Response>;
  async fn resolve_response(&self, req: Req, args: Args) -> anyhow::Result<String> {
    let res = self.resolve(req, args).await?;
    let res = serde_json::to_string(&res).context("failed at serializing response to json")?;
    Ok(res)
  }
}

#[async_trait]
pub trait ResolveToString<Req: HasResponse, Args = ()> {
  async fn resolve_to_string(&self, req: Req, args: Args) -> anyhow::Result<String>;
}

#[async_trait]
pub trait Resolver<ReqEnum, Args = ()> {
  async fn resolve_request(&self, request: ReqEnum, args: Args) -> anyhow::Result<String>;
}
