use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Génère un sel cryptographiquement sûr
    let salt = SaltString::generate(&mut OsRng);
    
    // Crée une instance d'Argon2 avec les paramètres par défaut (Argon2id, coût mémoire 19 Mo, 2 itérations, parallélisme 1)
    let argon2 = Argon2::default();
    
    // Hache le mot de passe
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    
    // Retourne le hash sous forme de chaîne (PHC format)
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}