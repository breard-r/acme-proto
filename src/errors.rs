use std::fmt;

#[derive(Debug)]
pub struct DeserializeError {
	pub(crate) data_type: String,
	pub(crate) err_msg: String,
}

impl fmt::Display for DeserializeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "invalid {} object: {}", self.data_type, self.err_msg)
	}
}
