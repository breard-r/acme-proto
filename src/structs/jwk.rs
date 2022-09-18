use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Jwk {
	Rsa {
		#[serde(skip_serializing_if = "Option::is_none")]
		alg: Option<String>,
		kty: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		#[serde(rename = "use")]
		key_use: Option<String>,
		e: String,
		n: String,
	},
	Ecdsa {
		#[serde(skip_serializing_if = "Option::is_none")]
		alg: Option<String>,
		crv: String,
		kty: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		#[serde(rename = "use")]
		key_use: Option<String>,
		x: String,
		y: String,
	},
	Eddsa {
		#[serde(skip_serializing_if = "Option::is_none")]
		alg: Option<String>,
		crv: String,
		kty: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		#[serde(rename = "use")]
		key_use: Option<String>,
		x: String,
	},
}

#[cfg(test)]
mod tests {
	use super::Jwk;

	#[test]
	fn serialize_rsa() {
		let ref_s = "{\"alg\":\"RS256\",\"kty\":\"RSA\",\"use\":\"sig\",\"e\":\"AQAB\",\"n\":\"jfjEAtwZv7IdEu2YvS2Y95a-y1tw_HvNFIqtuVc0On6X4UsSSH1QEVOAKgkAVJOEsMWcumnUNe9etOfbCpO9d6E4NEw8tOY2b1bMpGHGZPV-Yu1Ik3QxgvXU7MGg0T2vQMd-migVsNQpLIol47TUWFydpGYoBcFWnogWcaBIy4H9B_2hnyp8Z-zwoG3gEY24FFtfw3Y-tnQB4MY1IPW3dqMnZ2-AUqw8KV9TH0LhnnRhuwL47rXYOXolJG4IgOZeereyCqaTLasZ_EY_ZZ7wYqAsrt7GkwWnziRVNg3uGlQyaHo1pO7V0YAQ2oEerMjpycNPLYGZggm_V7zHqxDHAQ\"}";
		let jwk = Jwk::Rsa {
            alg: Some("RS256".to_string()),
            kty: "RSA".to_string(),
            key_use: Some("sig".to_string()),
            e: "AQAB".to_string(),
            n: "jfjEAtwZv7IdEu2YvS2Y95a-y1tw_HvNFIqtuVc0On6X4UsSSH1QEVOAKgkAVJOEsMWcumnUNe9etOfbCpO9d6E4NEw8tOY2b1bMpGHGZPV-Yu1Ik3QxgvXU7MGg0T2vQMd-migVsNQpLIol47TUWFydpGYoBcFWnogWcaBIy4H9B_2hnyp8Z-zwoG3gEY24FFtfw3Y-tnQB4MY1IPW3dqMnZ2-AUqw8KV9TH0LhnnRhuwL47rXYOXolJG4IgOZeereyCqaTLasZ_EY_ZZ7wYqAsrt7GkwWnziRVNg3uGlQyaHo1pO7V0YAQ2oEerMjpycNPLYGZggm_V7zHqxDHAQ".to_string(),
        };
		let res = serde_json::to_string(&jwk);
		assert!(res.is_ok());
		let s = res.unwrap();
		assert_eq!(s, ref_s);
	}

	#[test]
	fn serialize_rsa_thumbprint() {
		let ref_s = "{\"kty\":\"RSA\",\"e\":\"AQAB\",\"n\":\"hgUtxyPag2X4YmazM26LZa80J_VcfNqE6VhOPj7UuCh1wkugy5sbVxC9tufjXwmJ0YyJphGGp3VNCg6UqMTQwyXwj-1EtVafmm2UJt5cqfN2sR6fjtxUy2OxQH0XrV2PeLB4nDipJPZtnDt0hKY95FSXOXrtViSdr6aznzQOho1ApTBJsGRN37cRzzZNpECBzDeLKpj6OHAraLZdUTlIByMTMa9JhJmUPlerjGqd665mjeBEnIaP9-CeF9_Pq0-s7HaHq87vx1nfOXiAMGJ4uONL0qU6pe2ZXxLQ7uwynl6EdoKf8bHrah3nn3PUNfxoiJ33oCo81JWiKKkWi2ygpmxn5FuMMun-3vmEG_n2xhTXuFOKrbOX-rM7fALBC-zmgzGe1Cb1eRcoePr-0AgOEeSNJgPUmwqF1dv7WB6XWR7UmRxn1tM6qiAuAeXIDdNbbbkQkhFEvjDVhKwyR9kesgau0KZKonTlDT7k4w-GhsG6Am-iIl61JbzqapA-lBnGqxfxEgsKbJpSsHujKXJnXlT1PM4skuXwlKB9ZHJdPjXhNGSHw7JdeUPXmIez3w6FOGF7ur_ilTNIRvknAQSdFU9LJr2yS6QucTUqo6PSwlM-0D5nIaKaplooLvBTybk9q1miHQnLoOHBhHVOzj3gPCJU75R9fZCgyafhu_a2XD8\"}";
		let jwk = Jwk::Rsa {
            alg: None,
            kty: "RSA".to_string(),
            key_use: None,
            e: "AQAB".to_string(),
            n: "hgUtxyPag2X4YmazM26LZa80J_VcfNqE6VhOPj7UuCh1wkugy5sbVxC9tufjXwmJ0YyJphGGp3VNCg6UqMTQwyXwj-1EtVafmm2UJt5cqfN2sR6fjtxUy2OxQH0XrV2PeLB4nDipJPZtnDt0hKY95FSXOXrtViSdr6aznzQOho1ApTBJsGRN37cRzzZNpECBzDeLKpj6OHAraLZdUTlIByMTMa9JhJmUPlerjGqd665mjeBEnIaP9-CeF9_Pq0-s7HaHq87vx1nfOXiAMGJ4uONL0qU6pe2ZXxLQ7uwynl6EdoKf8bHrah3nn3PUNfxoiJ33oCo81JWiKKkWi2ygpmxn5FuMMun-3vmEG_n2xhTXuFOKrbOX-rM7fALBC-zmgzGe1Cb1eRcoePr-0AgOEeSNJgPUmwqF1dv7WB6XWR7UmRxn1tM6qiAuAeXIDdNbbbkQkhFEvjDVhKwyR9kesgau0KZKonTlDT7k4w-GhsG6Am-iIl61JbzqapA-lBnGqxfxEgsKbJpSsHujKXJnXlT1PM4skuXwlKB9ZHJdPjXhNGSHw7JdeUPXmIez3w6FOGF7ur_ilTNIRvknAQSdFU9LJr2yS6QucTUqo6PSwlM-0D5nIaKaplooLvBTybk9q1miHQnLoOHBhHVOzj3gPCJU75R9fZCgyafhu_a2XD8".to_string(),
        };
		let res = serde_json::to_string(&jwk);
		assert!(res.is_ok());
		let s = res.unwrap();
		assert_eq!(s, ref_s);
	}

	#[test]
	fn serialize_ecdsa() {
		let ref_s = "{\"alg\":\"ES384\",\"crv\":\"P-384\",\"kty\":\"EC\",\"use\":\"sig\",\"x\":\"O6XNy1zaY2CSZrWsuxOlXuK0xxmILGC3tXWRgmEyoimjZx5loc5lsXEXg0WYHTLa\",\"y\":\"4Dv8OtS5tx9x3l4l_fuLjZxj3aBZNF8SibKaQkRWHiDdbVRNYe9jKC3zhfRX3b-S\"}";
		let jwk = Jwk::Ecdsa {
			alg: Some("ES384".to_string()),
			crv: "P-384".to_string(),
			kty: "EC".to_string(),
			key_use: Some("sig".to_string()),
			x: "O6XNy1zaY2CSZrWsuxOlXuK0xxmILGC3tXWRgmEyoimjZx5loc5lsXEXg0WYHTLa".to_string(),
			y: "4Dv8OtS5tx9x3l4l_fuLjZxj3aBZNF8SibKaQkRWHiDdbVRNYe9jKC3zhfRX3b-S".to_string(),
		};
		let res = serde_json::to_string(&jwk);
		assert!(res.is_ok());
		let s = res.unwrap();
		assert_eq!(s, ref_s);
	}

	#[test]
	fn serialize_ecdsa_thumbprint() {
		let ref_s = "{\"crv\":\"P-256\",\"kty\":\"EC\",\"x\":\"ilsljbd6og8KH4DvHSYMv-E8EyQsVbiJY2-nVzoJINc\",\"y\":\"aDiZYC1DiILIub_MNmFtNbDtPbe68FsCnUIRNnDKpcI\"}";
		let jwk = Jwk::Ecdsa {
			alg: None,
			crv: "P-256".to_string(),
			kty: "EC".to_string(),
			key_use: None,
			x: "ilsljbd6og8KH4DvHSYMv-E8EyQsVbiJY2-nVzoJINc".to_string(),
			y: "aDiZYC1DiILIub_MNmFtNbDtPbe68FsCnUIRNnDKpcI".to_string(),
		};
		let res = serde_json::to_string(&jwk);
		assert!(res.is_ok());
		let s = res.unwrap();
		assert_eq!(s, ref_s);
	}

	#[test]
	fn serialize_eddsa() {
		let ref_s = "{\"alg\":\"EdDSA\",\"crv\":\"Ed25519\",\"kty\":\"OKP\",\"use\":\"sig\",\"x\":\"nITdIc8eoqRsy9pHvlhEHwAKku0jA1j0gSR_f6BfyjA\"}";
		let jwk = Jwk::Eddsa {
			alg: Some("EdDSA".to_string()),
			crv: "Ed25519".to_string(),
			kty: "OKP".to_string(),
			key_use: Some("sig".to_string()),
			x: "nITdIc8eoqRsy9pHvlhEHwAKku0jA1j0gSR_f6BfyjA".to_string(),
		};
		let res = serde_json::to_string(&jwk);
		assert!(res.is_ok());
		let s = res.unwrap();
		assert_eq!(s, ref_s);
	}

	#[test]
	fn serialize_eddsa_thumbprint() {
		let ref_s = "{\"crv\":\"Ed448\",\"kty\":\"OKP\",\"x\":\"-4VPMWQCo8Ykyir7omFtQD2Lznko1A3QjA9-wCwjNC49PeUFzBgp5b-GNCMH-0RhffZkx9Ce14kA\"}";
		let jwk = Jwk::Eddsa {
			alg: None,
			crv: "Ed448".to_string(),
			kty: "OKP".to_string(),
			key_use: None,
			x: "-4VPMWQCo8Ykyir7omFtQD2Lznko1A3QjA9-wCwjNC49PeUFzBgp5b-GNCMH-0RhffZkx9Ce14kA"
				.to_string(),
		};
		let res = serde_json::to_string(&jwk);
		assert!(res.is_ok());
		let s = res.unwrap();
		assert_eq!(s, ref_s);
	}
}
