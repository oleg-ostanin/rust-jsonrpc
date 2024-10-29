use super::{Error, Result};
use crate::auth_config;
use crate::pwd::ContentToHash;
use hmac::{Hmac, Mac};
use lib_utils::b64::b64u_encode;
use sha2::Sha512;

pub fn hash(to_hash: &ContentToHash) -> Result<String> {
    let key = &auth_config().PWD_KEY;

    let ContentToHash { content, salt } = to_hash;

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::Key)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    let result = b64u_encode(result_bytes);

    Ok(result)
}

pub fn validate(to_hash: &ContentToHash, to_validate: &str) -> Result<()> {
    let key = &auth_config().PWD_KEY;

    let ContentToHash { content, salt } = to_hash;

    // -- Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::Key)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    let result = b64u_encode(result_bytes);

    if &result == to_validate {
        Ok(())
    }
    else {
        Err(Error::PwdValidate)
    }
}