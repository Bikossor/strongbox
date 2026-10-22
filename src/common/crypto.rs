use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::Error};

fn get_argon2_config() -> Argon2<'static> {
    // Defaults to m=19456 (19 MiB), t=2 and p=1
    Argon2::default()
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let argon2 = get_argon2_config();
    let hash = argon2.hash_password(password.as_bytes())?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    get_argon2_config()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}
