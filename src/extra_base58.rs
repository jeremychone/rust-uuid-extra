use crate::extra_uuid::{new_v4, new_v7};

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
}

// endregion: --- Tests
