use derive_more::{Display, From};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),

	FailToDecode16U8 {
		context: &'static str,
		actual_length: usize,
	},

	FailExtractTimeNoUuidV7(Uuid),

	// -- Externals
	#[from]
	Io(std::io::Error), // as example
}

// region:    --- Custom

impl Error {
	pub fn custom_from_err(err: impl std::error::Error) -> Self {
		Self::Custom(err.to_string())
	}

	pub fn custom(val: impl Into<String>) -> Self {
		Self::Custom(val.into())
	}
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
