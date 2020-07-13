use jsonwebtoken::{encode, decode};
use argonautica::{Hasher, Verifier, config::Variant, input::{SecretKey, Salt}};
use futures::Future;

// TODO implement salt?
//

pub async fn hash_pwd(pwd: String) -> String {
    let secret_key = get_secret_key().await.unwrap();
    let mut hasher = Hasher::default();
    hasher.configure_secret_key_clearing(false)
        .configure_threads(2)
        .configure_variant(Variant::Argon2id)
        .with_salt(Salt::random(16))
        .with_secret_key(&secret_key)
        .with_password(pwd)
        .hash_non_blocking()
        .wait().unwrap()
}

pub async fn verify_pwd(pwd: &String, db_pwd: &String) -> bool {
    let secret_key = get_secret_key().await.unwrap();
    let mut verifier = Verifier::default();
    verifier.with_secret_key(&secret_key)
        .with_hash(&db_pwd)
        .with_password(pwd)
        .verify_non_blocking()
        .wait().unwrap()
}

pub async fn get_jwt_secret() -> Result<String, std::io::Error> {
    Ok(dotenv::var("JWT_SECRET")
        .expect("JWT SECRET NOT SET"))
}

pub async fn get_secret_key() -> Result<SecretKey<'static>, std::io::Error>  {
    let key = dotenv::var("SECRET_KEY")
        .expect("SECRET_KEY NOT SET");
    Ok(SecretKey::from_base64_encoded(key).unwrap())
}
