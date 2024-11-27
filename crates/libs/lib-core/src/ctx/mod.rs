// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,
	meta_cookie: String, //todo delete

	/// Note: For the future ACS (Access Control System)
	conv_id: Option<i64>,
}

// Constructors.
impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx {
			user_id: 0,
			meta_cookie: "root_ctx_meta".to_string(),
			conv_id: None,
		}
	}

	pub fn new(user_id: i64, meta_cookie: String) -> Result<Self> {
		if user_id == 0 {
			Err(Error::CtxCannotNewRootCtx)
		} else {
			Ok(Self {
				user_id,
				meta_cookie,
				conv_id: None,
			})
		}
	}

	/// Note: For the future ACS (Access Control System)
	pub fn add_conv_id(&self, conv_id: i64) -> Ctx {
		let mut ctx = self.clone();
		ctx.conv_id = Some(conv_id);
		ctx
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}

	//. /// Note: For the future ACS (Access Control System)
	pub fn conv_id(&self) -> Option<i64> {
		self.conv_id
	}
}
