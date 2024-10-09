use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use serde::Serialize;
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};
use std::sync::Arc;
use tracing::{debug, warn};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {

}