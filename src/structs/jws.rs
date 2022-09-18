use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Jws<T: Clone> {
	protected: JwsProtectedHeader,
	payload: T,
	signature: Option<String>,
}

impl<T: Clone> Jws<T> {
	pub fn new(protected: &JwsProtectedHeader, payload: &T) -> Self {
		Self {
			protected: protected.clone(),
			payload: payload.clone(),
			signature: None,
		}
	}

	pub fn sign(&mut self) {
		// TODO
	}
}

#[derive(Clone, Debug, Serialize)]
pub struct JwsProtectedHeader {
	alg: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	jwk: Option<serde_json::Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	kid: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	nonce: Option<String>,
	url: String,
}

impl JwsProtectedHeader {}
