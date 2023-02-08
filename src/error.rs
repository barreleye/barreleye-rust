use serde_derive::Deserialize;
use std::fmt;

/// An error encountered when communicating with the Barreleye API.
#[derive(Debug)]
pub enum Error {
	/// Could not send request.
	Unavailable,
	/// An error reported by Barreleye in the response body.
	Barreleye(RequestError),
	/// Internal error in this library.
	Unexpected(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Error::Unavailable => write!(f, "server is unavailable"),
			Error::Barreleye(ref err) => write!(f, "error reported by Barreleye: {err}"),
			Error::Unexpected(ref err) => write!(f, "unexpected error: {err}"),
		}
	}
}

impl From<reqwest::Error> for Error {
	fn from(e: reqwest::Error) -> Error {
		Error::Unexpected(e.to_string())
	}
}

#[derive(Debug, Default, Deserialize)]
pub struct RequestError {
	#[serde(default)]
	pub error: Option<String>,
}

impl fmt::Display for RequestError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(ref error) = self.error {
			write!(f, ": {error}")?;
		}

		Ok(())
	}
}
