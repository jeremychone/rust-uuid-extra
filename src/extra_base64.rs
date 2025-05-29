use crate::extra_uuid::{new_v4, new_v7};
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

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;
	use base64::{Engine as _, engine::general_purpose as b64_gp};
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
}

// endregion: --- Tests
