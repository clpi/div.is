use jsonwebtoken::{encode, decode};
use argonautica::{input::{SecretKey, Salt}, Hasher, Verifier};

// TODO implement salt?

pub async fn hash_password(pwd: String) -> Result<String, argonautica::Error> {
    let secret_key = get_secret_key().await?;

    let mut hasher = Hasher::default();
    let mut verifier = Verifier::default();
    let hash = hasher
        .with_password(pwd)
        .with_secret_key(&secret_key)
        .hash()?;
    //hash_oon_blocking
    Ok(hash)
}

pub async fn verify_password(pwd: String) -> String {
    "".to_string()
}

pub async fn get_jwt_secret() -> Result<String, std::io::Error> {
    Ok(dotenv::var("JWT_SECRET")
        .expect("JWT SECRET NOT SET"))
}

pub async fn get_secret_key() -> Result<SecretKey<'static>, argonautica::Error>  {
    let key = dotenv::var("SECRET_KEY")
        .expect("SECRET_KEY NOT SET");
    Ok(SecretKey::from_base64_encoded(&key)?)
}
