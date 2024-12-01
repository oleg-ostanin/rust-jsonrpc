use crate::log::log_request;
use crate::middleware::mw_auth::CtxW;
use crate::middleware::mw_req_stamp::ReqStamp;
use crate::handlers::handlers_rpc::RpcInfo;
use crate::error::Error;

use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, to_value};
use std::sync::Arc;
use tower_cookies::Cookies;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_response_map(
	ctx: Option<CtxW>,
	uri: Uri,
	req_method: Method,
	req_stamp: ReqStamp,
	res: Response,
) -> Response {
	let ctx = ctx.map(|ctx| ctx.0);

	debug!("{:<12} - mw_response_map", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	let rpc_info = res.extensions().get::<Arc<RpcInfo>>().map(Arc::as_ref);

	// -- Get the eventual response error.
	let web_error = res.extensions().get::<Arc<Error>>().map(Arc::as_ref);

	// if let Some(we) = web_error {
	// 	panic!()
	// }

	let client_status_error = web_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new response.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error = to_value(client_error).ok();
				let message = client_error.as_ref().and_then(|v| v.get("message"));
				let detail = client_error.as_ref().and_then(|v| v.get("detail"));

				let client_error_body = json!({
					"id": rpc_info.as_ref().map(|rpc| rpc.id.clone()),
					"error": {
						"message": message, // Variant name
						"data": {
							"req_uuid": uuid.to_string(),
							"detail": detail
						},
					}
				});

				debug!("CLIENT ERROR BODY:\n{client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// -- Build and log the server log line.
	let client_error = client_status_error.unzip().1;

	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ = log_request(
		req_method,
		uri,
		req_stamp,
		rpc_info,
		ctx,
		web_error,
		client_error,
	)
	.await;

	debug!("\n");

	error_response.unwrap_or(res)
}
