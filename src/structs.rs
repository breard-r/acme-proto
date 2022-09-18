#[macro_export]
macro_rules! deserialize_from_str {
	($t: ty, $label: expr) => {
		impl std::str::FromStr for $t {
			type Err = $crate::errors::DeserializeError;

			fn from_str(data: &str) -> Result<Self, Self::Err> {
				let res = serde_json::from_str(data).map_err(|e| Self::Err {
					data_type: $label.to_string(),
					err_msg: e.to_string(),
				})?;
				Ok(res)
			}
		}
	};
}

mod directory;
mod error;
mod jwk;

pub use directory::{Directory, DirectoryMeta};
pub use error::Error;
pub use jwk::Jwk;
