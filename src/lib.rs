use anyhow::Context;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub use resolver_api_derive as derive;

#[async_trait]
pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static {
    type Response: Serialize + DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[async_trait]
pub trait Resolve<Req: HasResponse> {
    async fn resolve(&self, req: Req) -> anyhow::Result<Req::Response>;
    async fn resolve_response(&self, req: Req) -> anyhow::Result<String> {
        let res = self.resolve(req).await?;
        let res = serde_json::to_string(&res).context("failed at serializing response to json")?;
        Ok(res)
    }
}

#[async_trait]
pub trait ResolveToString<Req: HasResponse> {
    async fn resolve_to_string(&self, req: Req) -> anyhow::Result<String>;
}

#[async_trait]
pub trait Resolver<ReqEnum: Send + Sync> {
    async fn resolve_request(&self, request: ReqEnum) -> anyhow::Result<String>;
}
