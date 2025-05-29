use crate::extra_uuid::{new_v4, new_v7};
use crate::{Error, Result, support};
use base64::{Engine as _, engine::general_purpose};
use uuid::Uuid;

// region:    --- v4

/// Generates a new UUID version 4 and encodes it using standard Base64.
pub fn new_v4_b64() -> String {
	let uuid = new_v4();
	general_purpose::STANDARD.encode(uuid.as_bytes())
}

/// Generates a new UUID version 4 and encodes it using URL-safe Base64.
pub fn new_v4_b64url() -> String {
	let uuid = new_v4();
	general_purpose::URL_SAFE.encode(uuid.as_bytes())
}

/// Generates a new UUID version 4 and encodes it using URL-safe Base64 without padding.
pub fn new_v4_b64url_nopad() -> String {
	let uuid = new_v4();
	general_purpose::URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

// endregion: --- v4

// region:    --- v7

/// Generates a new UUID version 7 and encodes it using standard Base64.
pub fn new_v7_b64() -> String {
	let uuid = new_v7();
	general_purpose::STANDARD.encode(uuid.as_bytes())
}

/// Generates a new UUID version 7 and encodes it using URL-safe Base64.
pub fn new_v7_b64url() -> String {
	let uuid = new_v7();
	general_purpose::URL_SAFE.encode(uuid.as_bytes())
}

/// Generates a new UUID version 7 and encodes it using URL-safe Base64 without padding.
pub fn new_v7_b64url_nopad() -> String {
	let uuid = new_v7();
	general_purpose::URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

// endregion: --- v7

// region:    --- From String

/// Decodes a standard Base64 encoded string into a UUID.
pub fn from_b64(s: &str) -> Result<Uuid> {
	let decoded_bytes = general_purpose::STANDARD.decode(s).map_err(Error::custom_from_err)?;
	support::from_vec_u8(decoded_bytes, "base64")
}

/// Decodes a URL-safe Base64 encoded string (with padding) into a UUID.
pub fn from_b64url(s: &str) -> Result<Uuid> {
	let decoded_bytes = general_purpose::URL_SAFE.decode(s).map_err(Error::custom_from_err)?;
	support::from_vec_u8(decoded_bytes, "base64url")
}

/// Decodes a URL-safe Base64 encoded string (without padding) into a UUID.
pub fn from_b64url_nopad(s: &str) -> Result<Uuid> {
	let decoded_bytes = general_purpose::URL_SAFE_NO_PAD.decode(s).map_err(Error::custom_from_err)?;
	support::from_vec_u8(decoded_bytes, "base64url-nopad")
}

// endregion: --- From String

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;
	use base64::engine::general_purpose as b64_gp;
	use uuid::{Uuid, Version};

	#[test]
	fn test_extra_base64_new_v4_b64_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b64_uuid = new_v4_b64();

		// -- Check
		assert_eq!(
			b64_uuid.len(),
			24,
			"Standard Base64 of UUID should be 24 chars with padding"
		);
		assert!(
			b64_uuid.ends_with("=="),
			"Standard Base64 should be padded with '==' for 16 bytes"
		);
		assert!(
			!b64_uuid.contains('-') && !b64_uuid.contains('_'),
			"Standard Base64 should not contain URL-safe characters '-' or '_'"
		);

		// Decode and verify UUID
		let decoded_bytes = b64_gp::STANDARD.decode(&b64_uuid)?;
		let uuid = Uuid::from_bytes(decoded_bytes.try_into().map_err(|_| "Failed to convert vec to array")?);
		assert_eq!(uuid.get_version(), Some(Version::Random));
		Ok(())
	}

	#[test]
	fn test_extra_base64_new_v4_b64url_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b64url_uuid = new_v4_b64url();

		// -- Check
		assert_eq!(
			b64url_uuid.len(),
			24,
			"URL-safe Base64 of UUID should be 24 chars with padding"
		);
		assert!(
			b64url_uuid.ends_with("=="),
			"URL-safe Base64 with padding should be padded with '==' for 16 bytes"
		);
		assert!(
			!b64url_uuid.contains('+') && !b64url_uuid.contains('/'),
			"URL-safe Base64 should not contain '+' or '/'"
		);

		// Decode and verify UUID
		let decoded_bytes = b64_gp::URL_SAFE.decode(&b64url_uuid)?;
		let uuid = Uuid::from_bytes(decoded_bytes.try_into().map_err(|_| "Failed to convert vec to array")?);
		assert_eq!(uuid.get_version(), Some(Version::Random));
		Ok(())
	}

	#[test]
	fn test_extra_base64_new_v4_b64url_nopad_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b64url_nopad_uuid = new_v4_b64url_nopad();

		// -- Check
		assert_eq!(
			b64url_nopad_uuid.len(),
			22,
			"URL-safe Base64 (no pad) of UUID should be 22 chars"
		);
		assert!(
			!b64url_nopad_uuid.ends_with('='),
			"URL-safe Base64 (no pad) should not have padding"
		);
		assert!(
			!b64url_nopad_uuid.contains('+') && !b64url_nopad_uuid.contains('/'),
			"URL-safe Base64 (no pad) should not contain '+' or '/'"
		);

		// Decode and verify UUID
		let decoded_bytes = b64_gp::URL_SAFE_NO_PAD.decode(&b64url_nopad_uuid)?;
		let uuid = Uuid::from_bytes(decoded_bytes.try_into().map_err(|_| "Failed to convert vec to array")?);
		assert_eq!(uuid.get_version(), Some(Version::Random));
		Ok(())
	}

	#[test]
	fn test_extra_base64_new_v7_b64_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b64_uuid = new_v7_b64();

		// -- Check
		assert_eq!(
			b64_uuid.len(),
			24,
			"Standard Base64 of V7 UUID should be 24 chars with padding"
		);
		assert!(
			b64_uuid.ends_with("=="),
			"Standard Base64 should be padded with '==' for 16 bytes"
		);
		assert!(
			!b64_uuid.contains('-') && !b64_uuid.contains('_'),
			"Standard Base64 should not contain URL-safe characters '-' or '_'"
		);

		// Decode and verify UUID
		let decoded_bytes = b64_gp::STANDARD.decode(&b64_uuid)?;
		let uuid = Uuid::from_bytes(decoded_bytes.try_into().map_err(|_| "Failed to convert vec to array")?);
		assert_eq!(uuid.get_version(), Some(Version::SortRand));
		Ok(())
	}

	#[test]
	fn test_extra_base64_new_v7_b64url_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b64url_uuid = new_v7_b64url();

		// -- Check
		assert_eq!(
			b64url_uuid.len(),
			24,
			"URL-safe Base64 of V7 UUID should be 24 chars with padding"
		);
		assert!(
			b64url_uuid.ends_with("=="),
			"URL-safe Base64 with padding should be padded with '==' for 16 bytes"
		);
		assert!(
			!b64url_uuid.contains('+') && !b64url_uuid.contains('/'),
			"URL-safe Base64 should not contain '+' or '/'"
		);

		// Decode and verify UUID
		let decoded_bytes = b64_gp::URL_SAFE.decode(&b64url_uuid)?;
		let uuid = Uuid::from_bytes(decoded_bytes.try_into().map_err(|_| "Failed to convert vec to array")?);
		assert_eq!(uuid.get_version(), Some(Version::SortRand));
		Ok(())
	}

	#[test]
	fn test_extra_base64_new_v7_b64url_nopad_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b64url_nopad_uuid = new_v7_b64url_nopad();

		// -- Check
		assert_eq!(
			b64url_nopad_uuid.len(),
			22,
			"URL-safe Base64 (no pad) of V7 UUID should be 22 chars"
		);
		assert!(
			!b64url_nopad_uuid.ends_with('='),
			"URL-safe Base64 (no pad) should not have padding"
		);
		assert!(
			!b64url_nopad_uuid.contains('+') && !b64url_nopad_uuid.contains('/'),
			"URL-safe Base64 (no pad) should not contain '+' or '/'"
		);

		// Decode and verify UUID
		let decoded_bytes = b64_gp::URL_SAFE_NO_PAD.decode(&b64url_nopad_uuid)?;
		let uuid = Uuid::from_bytes(decoded_bytes.try_into().map_err(|_| "Failed to convert vec to array")?);
		assert_eq!(uuid.get_version(), Some(Version::SortRand));
		Ok(())
	}

	// region:    --- Tests for from_... functions

	#[test]
	fn test_extra_base64_from_b64_ok() -> Result<()> {
		// -- Setup & Fixtures
		let original_uuid = Uuid::new_v4();
		let b64_string = b64_gp::STANDARD.encode(original_uuid.as_bytes());

		// -- Exec
		let decoded_uuid_res = from_b64(&b64_string);

		// -- Check
		assert!(decoded_uuid_res.is_ok(), "Decoding should succeed");
		assert_eq!(
			decoded_uuid_res.unwrap(),
			original_uuid,
			"Decoded UUID should match original"
		);
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64_err_invalid_char() -> Result<()> {
		// -- Setup & Fixtures
		let invalid_b64_string = "ThisIsNotValidBase64!=";

		// -- Exec
		let decoded_uuid_res = from_b64(invalid_b64_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for invalid characters");
		let err_msg = decoded_uuid_res.err().unwrap().to_string();
		assert!(
			err_msg.contains("Invalid symbol"),
			"Error message should indicate 'Invalid symbol'"
		);
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64_err_wrong_len() -> Result<()> {
		// -- Setup & Fixtures
		let short_b64_string = b64_gp::STANDARD.encode("short"); // Decodes to 5 bytes

		// -- Exec
		let decoded_uuid_res = from_b64(&short_b64_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for wrong length");
		let err_msg = decoded_uuid_res.err().unwrap().to_string();
		println!("->> {err_msg}");
		assert!(
			err_msg.contains("FailToDecode16U8"),
			"Error message should indicate wrong length"
		);
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64url_ok() -> Result<()> {
		// -- Setup & Fixtures
		let original_uuid = Uuid::new_v4();
		let b64url_string = b64_gp::URL_SAFE.encode(original_uuid.as_bytes());

		// -- Exec
		let decoded_uuid_res = from_b64url(&b64url_string);

		// -- Check
		assert!(decoded_uuid_res.is_ok(), "Decoding should succeed");
		assert_eq!(
			decoded_uuid_res.unwrap(),
			original_uuid,
			"Decoded UUID should match original"
		);
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64url_err_invalid_char() -> Result<()> {
		// -- Setup & Fixtures
		let invalid_b64url_string = "ThisIsNotValidBase64Url+"; // '+' is not valid for URL_SAFE if not part of encoding

		// -- Exec
		let decoded_uuid_res = from_b64url(invalid_b64url_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for invalid characters");
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64url_err_wrong_len() -> Result<()> {
		// -- Setup & Fixtures
		let short_b64url_string = b64_gp::URL_SAFE.encode("short"); // Decodes to 5 bytes

		// -- Exec
		let decoded_uuid_res = from_b64url(&short_b64url_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for wrong length");
		let err_msg = decoded_uuid_res.err().unwrap().to_string();
		assert!(
			err_msg.contains("FailToDecode16U8 { context: \"base64url\""),
			"Error message should indicate FailToDecode16U8 for base64url"
		);
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64url_nopad_ok() -> Result<()> {
		// -- Setup & Fixtures
		let original_uuid = Uuid::new_v4();
		let b64url_nopad_string = b64_gp::URL_SAFE_NO_PAD.encode(original_uuid.as_bytes());

		// -- Exec
		let decoded_uuid_res = from_b64url_nopad(&b64url_nopad_string);

		// -- Check
		assert!(decoded_uuid_res.is_ok(), "Decoding should succeed");
		assert_eq!(
			decoded_uuid_res.unwrap(),
			original_uuid,
			"Decoded UUID should match original"
		);
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64url_nopad_err_invalid_char() -> Result<()> {
		// -- Setup & Fixtures
		let invalid_b64url_nopad_string = "ThisIsNotValidBase64UrlNoPad="; // '=' is not valid for NO_PAD

		// -- Exec
		let decoded_uuid_res = from_b64url_nopad(invalid_b64url_nopad_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for invalid characters");
		Ok(())
	}

	#[test]
	fn test_extra_base64_from_b64url_nopad_err_wrong_len() -> Result<()> {
		// -- Setup & Fixtures
		let short_b64url_nopad_string = b64_gp::URL_SAFE_NO_PAD.encode("short"); // Decodes to 5 bytes

		// -- Exec
		let decoded_uuid_res = from_b64url_nopad(&short_b64url_nopad_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for wrong length");
		let err_msg = decoded_uuid_res.err().unwrap().to_string();
		assert!(
			err_msg.contains("FailToDecode16U8 { context: \"base64url-nopad\""),
			"Error message should indicate FailToDecode16U8 for base64url-nopad"
		);
		Ok(())
	}

	// endregion: --- Tests for from_... functions
}

// endregion: --- Tests
