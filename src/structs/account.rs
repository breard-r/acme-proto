use crate::structs::jwk::Jwk;
use crate::structs::jws::Jws;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountStatus {
	Valid,
	Deactivated,
	Revoked,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResource {
	pub status: AccountStatus,
	pub contact: Option<Vec<String>>,
	pub terms_of_service_agreed: Option<bool>,
	pub external_account_binding: Option<serde_json::Value>,
	#[cfg(feature = "opt_account_orders")]
	pub orders: Option<String>,
	#[cfg(not(feature = "opt_account_orders"))]
	pub orders: String,
}

deserialize_from_str!(AccountResource, "account resource");

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountManagement {
	#[serde(skip_serializing_if = "Option::is_none")]
	status: Option<AccountStatus>,
	#[serde(skip_serializing_if = "Option::is_none")]
	contact: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	terms_of_service_agreed: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	only_return_existing: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	external_account_binding: Option<Jws<Jwk>>,
}

impl AccountManagement {
	pub fn creation_request<T: ToString>(contacts: &[T], tos_agreed: bool) -> Self {
		Self {
			contact: Some(contacts.iter().map(|e| e.to_string()).collect()),
			terms_of_service_agreed: Some(tos_agreed),
			..Default::default()
		}
	}

	pub fn external_creation_request<T: ToString>(
		contacts: &[T],
		tos_agreed: bool,
		external_account: &Jws<Jwk>,
	) -> Self {
		Self {
			contact: Some(contacts.iter().map(|e| e.to_string()).collect()),
			terms_of_service_agreed: Some(tos_agreed),
			external_account_binding: Some(external_account.to_owned()),
			..Default::default()
		}
	}

	pub fn lookup_request() -> Self {
		Self {
			only_return_existing: Some(true),
			..Default::default()
		}
	}

	pub fn contact_update_request<T: ToString>(new_contacts: &[T]) -> Self {
		Self {
			contact: Some(new_contacts.iter().map(|e| e.to_string()).collect()),
			..Default::default()
		}
	}

	pub fn deactivation_request() -> Self {
		Self {
			status: Some(AccountStatus::Deactivated),
			..Default::default()
		}
	}
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountKeyRollover {
	pub account: String,
	pub old_key: Jwk,
}

#[cfg(test)]
mod tests {
	use super::{AccountManagement, AccountResource, AccountStatus};

	#[test]
	fn deserialize_account_status() {
		let tests = [
			(r#" "valid" "#, Some(AccountStatus::Valid)),
			(r#" "deactivated" "#, Some(AccountStatus::Deactivated)),
			(r#" "revoked" "#, Some(AccountStatus::Revoked)),
			(r#" "VALID" "#, None),
			(r#" "Deactivated" "#, None),
			(r#" "rEvOkEd" "#, None),
			(r#" "" "#, None),
			(r#" " valid" "#, None),
			(r#" "revoked " "#, None),
			(r#" "random stuff" "#, None),
			(r#" "vallid" "#, None),
		];
		for (status_str, status) in tests {
			let res = serde_json::from_str::<AccountStatus>(status_str);
			match status {
				Some(s_ref) => {
					assert!(res.is_ok(), "deserialization of `{}` failed", status_str);
					let s = res.unwrap();
					assert_eq!(s, s_ref);
				}
				None => {
					assert!(
						res.is_err(),
						"`{}` is incorrectly considered valid",
						status_str
					);
				}
			}
		}
	}

	#[test]
	fn deserialize_account_resource_min() {
		let ar = r#" {
			"status": "valid",
			"orders": "https://localhost:14000/list-orderz/1"
		} "#;
		let res = serde_json::from_str::<AccountResource>(ar);
		assert!(res.is_ok(), "deserialization failed");
		let ar = res.unwrap();
		assert_eq!(ar.status, AccountStatus::Valid);
		assert_eq!(ar.contact, None);
		assert_eq!(ar.terms_of_service_agreed, None);
		#[cfg(feature = "opt_account_orders")]
		assert_eq!(
			ar.orders,
			Some("https://localhost:14000/list-orderz/1".to_string())
		);
		#[cfg(not(feature = "opt_account_orders"))]
		assert_eq!(
			ar.orders,
			"https://localhost:14000/list-orderz/1".to_string()
		);
	}

	#[test]
	fn deserialize_account_resource_max() {
		let ar = r#" {
			"status": "valid",
			"contact": ["mailto:derp@example.com"],
			"orders": "https://localhost:14000/list-orderz/1",
			"termsOfServiceAgreed": true,
			"externalAccountBinding": {
				"protected": "eyJhbGciOiJIUzI1NiIsImtpZCI6ImtpZC0xIiwidXJsIjoiaHR0cHM6Ly9sb2NhbGhvc3Q6MTQwMDAvc2lnbi1tZS11cCJ9",
				"payload": "eyJhbGciOiJFUzM4NCIsImNydiI6IlAtMzg0Iiwia3R5IjoiRUMiLCJ1c2UiOiJzaWciLCJ4IjoiTzZYTnkxemFZMkNTWnJXc3V4T2xYdUsweHhtSUxHQzN0WFdSZ21FeW9pbWpaeDVsb2M1bHNYRVhnMFdZSFRMYSIsInkiOiI0RHY4T3RTNXR4OXgzbDRsX2Z1TGpaeGozYUJaTkY4U2liS2FRa1JXSGlEZGJWUk5ZZTlqS0MzemhmUlgzYi1TIn0",
				"signature": "a4em7D92pNE1_TA1ITYlerjGx2FG0aouOjijUqt9q0o"
			},
			"key": {
				"use": "sig",
				"kty": "EC",
				"crv": "P-384",
				"alg": "ES384",
				"x": "O6XNy1zaY2CSZrWsuxOlXuK0xxmILGC3tXWRgmEyoimjZx5loc5lsXEXg0WYHTLa",
				"y": "4Dv8OtS5tx9x3l4l_fuLjZxj3aBZNF8SibKaQkRWHiDdbVRNYe9jKC3zhfRX3b-S"
			}
		} "#;
		let res = serde_json::from_str::<AccountResource>(ar);
		assert!(res.is_ok(), "deserialization failed");
		let ar = res.unwrap();
		assert!(ar.external_account_binding.is_some());
		assert_eq!(ar.status, AccountStatus::Valid);
		assert_eq!(
			ar.contact,
			Some(vec!["mailto:derp@example.com".to_string()])
		);
		#[cfg(feature = "opt_account_orders")]
		assert_eq!(
			ar.orders,
			Some("https://localhost:14000/list-orderz/1".to_string())
		);
		#[cfg(not(feature = "opt_account_orders"))]
		assert_eq!(
			ar.orders,
			"https://localhost:14000/list-orderz/1".to_string()
		);
	}

	#[test]
	fn account_creation_request() {
		let contacts = ["mailto:test@example.org"];
		let req = AccountManagement::creation_request(&contacts, true);
		let req_str = serde_json::to_string(&req);
		assert!(req_str.is_ok(), "serialization failed");
		let req_str = req_str.unwrap();
		assert_eq!(
			req_str,
			r#"{"contact":["mailto:test@example.org"],"termsOfServiceAgreed":true}"#.to_string()
		);
	}

	#[test]
	fn account_contact_update_request_single() {
		let contacts = ["mailto:test@example.org"];
		let req = AccountManagement::contact_update_request(&contacts);
		let req_str = serde_json::to_string(&req);
		assert!(req_str.is_ok(), "serialization failed");
		let req_str = req_str.unwrap();
		assert_eq!(
			req_str,
			r#"{"contact":["mailto:test@example.org"]}"#.to_string()
		);
	}

	#[test]
	fn account_lookup_request() {
		let req = AccountManagement::lookup_request();
		let req_str = serde_json::to_string(&req);
		assert!(req_str.is_ok(), "serialization failed");
		let req_str = req_str.unwrap();
		assert_eq!(req_str, r#"{"onlyReturnExisting":true}"#.to_string());
	}

	#[test]
	fn account_contact_update_request_multiple() {
		let contacts = [
			"mailto:test@example.org",
			"mailto:admin@example.org",
			"mailto:acme@example.org",
		];
		let req = AccountManagement::contact_update_request(&contacts);
		let req_str = serde_json::to_string(&req);
		assert!(req_str.is_ok(), "serialization failed");
		let req_str = req_str.unwrap();
		assert_eq!(req_str, r#"{"contact":["mailto:test@example.org","mailto:admin@example.org","mailto:acme@example.org"]}"#.to_string());
	}

	#[test]
	fn account_deactivation_request() {
		let req = AccountManagement::deactivation_request();
		let req_str = serde_json::to_string(&req);
		assert!(req_str.is_ok(), "serialization failed");
		let req_str = req_str.unwrap();
		assert_eq!(req_str, r#"{"status":"deactivated"}"#.to_string());
	}
}
