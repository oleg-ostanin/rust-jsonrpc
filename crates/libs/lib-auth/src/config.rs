use lib_utils::envs::{get_env_b64u_as_u8s, get_env_parse};
use std::sync::OnceLock;
use lib_utils::b64::b64u_decode;

pub fn auth_config() -> &'static AuthConfig {
	static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		AuthConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct AuthConfig {
	// -- Crypt
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,
}

impl AuthConfig {
	fn load_from_env() -> lib_utils::envs::Result<AuthConfig> {
		Ok(AuthConfig {
			// -- Crypt
			PWD_KEY: b64u_decode("CKUGFOD9_2Qf6Pn3ZFRYgPYb8ht4vKqEG9PGMXTB7497bT0367DjoaD6ydFnEVaIRda0kKeBZVCT5Hb62m2sCA").unwrap().to_vec(),

			TOKEN_KEY: b64u_decode("9FoHBmkyxbgu_xFoQK7e0jz3RMNVJWgfvbVn712FBNH9LLaAWS3CS6Zpcg6RveiObvCUb6a2z-uAiLjhLh2igw").unwrap().to_vec(),
			TOKEN_DURATION_SEC: 1000.0
		})
	}
}
