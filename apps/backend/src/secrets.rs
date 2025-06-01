use std::path::PathBuf;

use jsonwebtoken;
use pem::Pem;
use rsa::{pkcs8::EncodePrivateKey, rand_core::OsRng, RsaPrivateKey};

static RSA_KEY_SIZE: usize = 3072;

#[derive(Clone)]
pub struct Secrets {
	pub jwt_enc: jsonwebtoken::EncodingKey,
	pub jwt_dec: jsonwebtoken::DecodingKey,
}

impl Secrets {
	fn create_jwt_secret() -> String {
		let private_key =
			RsaPrivateKey::new(&mut OsRng, RSA_KEY_SIZE).expect("Arguments should be sufficient");

		let encoded_key = private_key
			.to_pkcs8_der()
			.expect("Encoding failed")
			.to_bytes();

		let pem = Pem::new("PRIVATE KEY", encoded_key.to_vec());

		pem::encode(&pem)
	}

	pub fn new(base_dir: &PathBuf) -> Secrets {
		let secrets_dir = base_dir.join("secrets/");
		if !secrets_dir.exists() {
			std::fs::create_dir_all(&secrets_dir).expect("Read/write should be available");
		}

		let jwt_secret_path = secrets_dir.join("jwt_secret.key");
		let jwt_secret_str = if jwt_secret_path.exists() {
			std::fs::read_to_string(&jwt_secret_path).expect("File should exist")
		} else {
			let new_secret = Secrets::create_jwt_secret();
			std::fs::write(&jwt_secret_path, &new_secret).expect("Failed to write secret");
			new_secret
		};

		let jwt_secret = pem::parse(jwt_secret_str).unwrap();
		let jwt_enc = jsonwebtoken::EncodingKey::from_rsa_pem(jwt_secret.contents())
			.expect("Key format should be valid");
		let jwt_dec = jsonwebtoken::DecodingKey::from_rsa_pem(jwt_secret.contents())
			.expect("Key format should be valid");

		Secrets { jwt_enc, jwt_dec }
	}
}
