use uuid::Uuid;

// region:    --- Raw Uuid

/// Generates a new UUID version 4.
///
/// UUID version 4 is randomly generated.
pub fn new_v4() -> Uuid {
	Uuid::new_v4()
}

/// Alias to `now_v7`
pub fn new_v7() -> Uuid {
	Uuid::now_v7()
}

/// Generates a new UUID version 7 with the now time.
///
/// UUID version 7 is a time-ordered UUID which is well-suited for use as a database key.
/// This function uses `Uuid::now_v7()` which is suitable for most cases.
pub fn now_v7() -> Uuid {
	Uuid::now_v7()
}
// endregion: --- Raw Uuid

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;
	use uuid::Version;

	#[test]
	fn test_extra_uuid_new_v4_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let uuid_v4 = new_v4();

		// -- Check
		assert_eq!(uuid_v4.get_version(), Some(Version::Random));
		Ok(())
	}

	#[test]
	fn test_extra_uuid_new_v7_simple() -> Result<()> {
		// -- Setup & Fixtures
		// (no specific setup needed for this test)

		// -- Exec
		let uuid_v7_1 = new_v7();
		// Ensure a slight delay for time-based ordering if testing monotonicity strictly,
		// but for basic version check, it's not critical.
		// For now, we just check the version.
		std::thread::sleep(std::time::Duration::from_micros(100)); // Ensure time moves for next v7
		let uuid_v7_2 = new_v7();

		// -- Check
		assert_eq!(uuid_v7_1.get_version(), Some(Version::SortRand));
		assert_eq!(uuid_v7_2.get_version(), Some(Version::SortRand));
		// V7 UUIDs generated sequentially should be ordered
		// Note: Uuid::now_v7() guarantees monotonicity for calls within the same millisecond
		// by incrementing a sequence counter. If calls are in different milliseconds,
		// the time component will ensure ordering.
		assert!(uuid_v7_1 < uuid_v7_2, "UUID {} should be less than UUID {}", uuid_v7_1, uuid_v7_2);

		Ok(())
	}
}

// endregion: --- Tests

