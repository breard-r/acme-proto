use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
	#[serde(rename = "type")]
	error_type: String,
	detail: Option<String>,
	subproblems: Option<Vec<Subproblem>>,
}

deserialize_from_str!(Error, "error");

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(
			f,
			"server returned the following error: {}",
			self.error_type
		)?;
		if let Some(detail) = &self.detail {
			writeln!(f, "{}", detail)?;
		}
		if let Some(subproblems) = &self.subproblems {
			if !subproblems.is_empty() {
				writeln!(f, "The following sub-problems were also reported:")?;
				for sp in subproblems {
					writeln!(f, " - {}", sp)?;
				}
			}
		}
		Ok(())
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Subproblem {
	#[serde(rename = "type")]
	error_type: String,
	detail: Option<String>,
	identifier: Option<String>, // TODO: replace by the real type
}

impl fmt::Display for Subproblem {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(identifier) = &self.identifier {
			writeln!(f, "{}: ", identifier)?;
		}
		write!(f, "{}", self.error_type)?;
		if let Some(detail) = &self.detail {
			writeln!(f, ": {}", detail)?;
		}
		writeln!(f)
	}
}
