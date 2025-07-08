use std::path::PathBuf;

use jsonwebtoken;
use pem::Pem;
use rsa::{
	pkcs8::{EncodePrivateKey, EncodePublicKey},
	rand_core::OsRng,
	RsaPrivateKey, RsaPublicKey,
};

static RSA_KEY_SIZE: usize = 3072;

#[derive(Clone)]
pub struct Secrets {
	pub jwt_enc: jsonwebtoken::EncodingKey,
	pub jwt_dec: jsonwebtoken::DecodingKey,
}

impl Secrets {
	/// Create RSA private and public keys for JWT
	///
	/// Tuple contains:
	///  - Private key string
	///  - Public key string
	fn create_jwt_keys() -> (String, String) {
		let private_key =
			RsaPrivateKey::new(&mut OsRng, RSA_KEY_SIZE).expect("Arguments should be sufficient");
		let private_key_der = private_key
			.to_pkcs8_der()
			.expect("Encoding failed")
			.to_bytes();
		let private_pem = Pem::new("PRIVATE KEY", private_key_der.to_vec());

		let public_key = RsaPublicKey::from(&private_key);
		let public_key_der = public_key.to_public_key_der().expect("Encoding failed");
		let public_pem = Pem::new("PUBLIC KEY", public_key_der.to_vec());

		(pem::encode(&private_pem), pem::encode(&public_pem))
	}

	pub fn new(base_dir: &PathBuf) -> Secrets {
		let secrets_dir = base_dir.join("secrets/");
		if !secrets_dir.exists() {
			std::fs::create_dir_all(&secrets_dir).expect("Read/write should be available");
		}

		let private_key_path = secrets_dir.join("jwt_private.key");
		let public_key_path = secrets_dir.join("jwt_public.pem");

		let (private_key, public_key) = if private_key_path.exists() && public_key_path.exists() {
			let private_key =
				std::fs::read_to_string(&private_key_path).expect("File should exist");
			let public_key = std::fs::read_to_string(&public_key_path).expect("File should exist");

			(private_key, public_key)
		} else {
			let (private_key, public_key) = Self::create_jwt_keys();

			std::fs::write(&private_key_path, &private_key).expect("Should have write access");
			std::fs::write(&public_key_path, &public_key).expect("Should have write access");

			(private_key, public_key)
		};

		let jwt_enc = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())
			.expect("Key format should be valid");
		let jwt_dec = jsonwebtoken::DecodingKey::from_rsa_pem(public_key.as_bytes())
			.expect("Key format should be valid");

		Secrets { jwt_enc, jwt_dec }
	}
}
