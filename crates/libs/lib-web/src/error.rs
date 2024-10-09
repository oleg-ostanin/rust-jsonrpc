use crate::middleware;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use lib_auth::{pwd, token};
use lib_core::model;
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