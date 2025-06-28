use crate::extra_uuid::{new_v4, new_v7};
use crate::{Error, Result, support};
use uuid::Uuid;

// region:    --- v4

/// Generates a new UUID version 4 and encodes it using Base58.
pub fn new_v4_b58() -> String {
	let uuid = new_v4();
	bs58::encode(uuid.as_bytes()).into_string()
}

// endregion: --- v4

// region:    --- v7

/// Generates a new UUID version 7 and encodes it using Base58.
pub fn new_v7_b58() -> String {
	let uuid = new_v7();
	bs58::encode(uuid.as_bytes()).into_string()
}

// endregion: --- v7

// region:    --- From String

/// Decodes a Base58 encoded string into a UUID.
pub fn from_b58(s: &str) -> Result<Uuid> {
	let decoded_bytes = bs58::decode(s).into_vec().map_err(Error::custom_from_err)?;
	support::from_vec_u8(decoded_bytes, "base58")
}

// endregion: --- From String

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;
	use uuid::{Uuid, Version};

	#[test]
	fn test_extra_base58_new_v4_b58_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b58_uuid_str = new_v4_b58();

		// -- Check
		// Basic check for Base58 character set (no padding, no '+', '/', '=', '0', 'O', 'I', 'l')
		assert!(!b58_uuid_str.contains('+'), "Base58 string should not contain '+'");
		assert!(!b58_uuid_str.contains('/'), "Base58 string should not contain '/'");
		assert!(!b58_uuid_str.contains('='), "Base58 string should not contain '='");
		assert!(!b58_uuid_str.contains('0'), "Base58 string should not contain '0'");
		assert!(!b58_uuid_str.contains('O'), "Base58 string should not contain 'O'");
		assert!(!b58_uuid_str.contains('I'), "Base58 string should not contain 'I'");
		assert!(!b58_uuid_str.contains('l'), "Base58 string should not contain 'l'");

		// Decode and verify UUID
		let decoded_bytes = bs58::decode(&b58_uuid_str).into_vec()?;
		assert_eq!(decoded_bytes.len(), 16, "Decoded Base58 UUID bytes should be 16");
		let uuid = Uuid::from_bytes(
			decoded_bytes
				.try_into()
				.map_err(|_| "Failed to convert vec to array for UUID v4")?,
		);
		assert_eq!(uuid.get_version(), Some(Version::Random));

		Ok(())
	}

	#[test]
	fn test_extra_base58_new_v7_b58_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let b58_uuid_str = new_v7_b58();

		// -- Check
		// Basic check for Base58 character set
		assert!(!b58_uuid_str.contains('+'), "Base58 string should not contain '+'");
		assert!(!b58_uuid_str.contains('/'), "Base58 string should not contain '/'");
		assert!(!b58_uuid_str.contains('='), "Base58 string should not contain '='");
		assert!(!b58_uuid_str.contains('0'), "Base58 string should not contain '0'");
		assert!(!b58_uuid_str.contains('O'), "Base58 string should not contain 'O'");
		assert!(!b58_uuid_str.contains('I'), "Base58 string should not contain 'I'");
		assert!(!b58_uuid_str.contains('l'), "Base58 string should not contain 'l'");

		// Decode and verify UUID
		let decoded_bytes = bs58::decode(&b58_uuid_str).into_vec()?;
		assert_eq!(decoded_bytes.len(), 16, "Decoded Base58 UUID bytes should be 16");
		let uuid = Uuid::from_bytes(
			decoded_bytes
				.try_into()
				.map_err(|_| "Failed to convert vec to array for UUID v7")?,
		);
		assert_eq!(uuid.get_version(), Some(Version::SortRand));

		Ok(())
	}

	// region:    --- Tests for from_... functions

	#[test]
	fn test_extra_base58_from_b58_ok() -> Result<()> {
		// -- Setup & Fixtures
		let original_uuid = new_v7(); // Using v7 for test, v4 would also work
		let b58_string = bs58::encode(original_uuid.as_bytes()).into_string();

		// -- Exec
		let decoded_uuid_res = from_b58(&b58_string);

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
	fn test_extra_base58_from_b58_err_invalid_char() -> Result<()> {
		// -- Setup & Fixtures
		let invalid_b58_string = "ThisIsInvalid0"; // '0' is an invalid Base58 character

		// -- Exec
		let decoded_uuid_res = from_b58(invalid_b58_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for invalid characters");
		let err_msg = decoded_uuid_res.err().unwrap().to_string();
		// bs58::decode::Error::InvalidCharacter Display is "provided string contained invalid character '{character:?}' at byte {index:?}"
		// This gets wrapped in Error::Custom(String), so the string will be like Custom("...")
		assert!(
			err_msg.contains("provided string contained invalid character"),
			"Error message should indicate invalid Base58 character. Got: {err_msg}",
		);
		Ok(())
	}

	#[test]
	fn test_extra_base58_from_b58_err_wrong_len() -> Result<()> {
		// -- Setup & Fixtures
		// "short" (5 bytes) encodes to "4LroS" in Base58
		let short_b58_string = bs58::encode(b"short").into_string();

		// -- Exec
		let decoded_uuid_res = from_b58(&short_b58_string);

		// -- Check
		assert!(decoded_uuid_res.is_err(), "Decoding should fail for wrong length");
		let err = decoded_uuid_res.err().unwrap();
		match err {
			crate::Error::FailToDecode16U8 { context, actual_length } => {
				assert_eq!(context, "base58", "Error context should be 'base58'");
				assert_eq!(actual_length, 5, "Actual length should be 5 for 'short'");
			}
			_ => panic!("Expected FailToDecode16U8, got {err:?}"),
		}
		Ok(())
	}

	// endregion: --- Tests for from_... functions
}

// endregion: --- Tests
