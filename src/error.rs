use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<Inner: std::fmt::Debug> {
  /// This error occurs when the resolver response fails to serialize to json
  #[error("deserialization error: {0:?}")]
  Serialization(#[from] serde_json::Error),

  /// This passes through the inner error
  #[error("inner error: {0:?}")]
  Inner(Inner),
}
