use anyhow::anyhow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<Inner: std::fmt::Debug> {
  /// This error occurs when the resolver response fails to serialize to json
  #[error("response serialization error: {0:?}")]
  Serialization(#[from] serde_json::Error),

  /// This passes through the inner error
  #[error("inner error: {0:#}")]
  Inner(Inner),
}

/// Useful to convert resolver errors back into inner anyhow::Error
/// without disrupting existing context.
/// ```
/// state.resolve_response(request, ()).await.map_err(into_anyhow_error)?
/// ```
pub fn into_anyhow_error(e: Error<anyhow::Error>) -> anyhow::Error {
  match e {
    Error::Serialization(e) => anyhow!("{e:?}"),
    Error::Inner(e) => e,
  }
}
