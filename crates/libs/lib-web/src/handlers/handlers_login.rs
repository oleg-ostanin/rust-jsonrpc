use std::ops::Deref;
use std::sync::Arc;
use crate::error::{Error, Result};
use crate::utils::token;
use axum::extract::State;
use axum::Json;
use lib_auth::pwd::{self, ContentToHash, SchemeStatus};
use lib_core::ctx::Ctx;

use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;
use lib_core::context::app_context::ModelManager;
use lib_core::model::store::user::UserBmc;
use lib_core::model::user::{UserForLogin, UserForSignIn};

// region:    --- Login
pub async fn api_login_handler(
	State(mm): State<Arc<ModelManager>>,
	cookies: Cookies,
	Json(payload): Json<UserForSignIn>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_login_handler", "HANDLER");

	let UserForSignIn {
		identity,
		password,
	} = payload;
	let root_ctx = Ctx::root_ctx();

	// -- Get the user.
	//let user: UserForLogin = UserBmc::get_for_login(&root_ctx, &mm, &username)
	let user: UserForLogin = UserBmc::get_for_login(&mm.deref(), &identity).await.unwrap(); // todo ununwrap
		//.await?
		//.ok_or(Error::LoginFailUsernameNotFound)?;
	let user_id = user.id;

	// -- Validate the password.
	let Some(pwd) = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd { user_id });
	};

	// todo revert to validate_pwd
	let scheme_status = pwd::validate_pwd(
		ContentToHash {
			content: password.clone(),
			salt: user.pwd_salt,
		},
		pwd,
	)
	.await
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	// -- Update password scheme if needed
	if let SchemeStatus::Outdated = scheme_status {
		debug!("pwd encrypt scheme outdated, upgrading.");
		// todo uncomment UserBmc::update_pwd(&root_ctx, &mm, user.id, &password).await?;
	}

	// -- Set web token.
	token::set_token_cookie(&cookies, &user.identity, user.token_salt)?;

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
	username: String,
	pwd: String,
}
// endregion: --- Login

// region:    --- Logoff
pub async fn api_logoff_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_logoff_handler", "HANDLER");
	let should_logoff = payload.logoff;

	if should_logoff {
		token::remove_token_cookie(&cookies)?;
	}

	// Create the success body.
	let body = Json(json!({
		"result": {
			"logged_off": should_logoff
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct LogoffPayload {
	logoff: bool,
}
// endregion: --- Logoff


// // todo revert to validate_pwd
// let scheme_status = pwd::scheme::hash::validate(
// &ContentToHash {
// content: password.clone(),
// salt: user.pwd_salt,
// },
// &pwd,
// )
// .map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;