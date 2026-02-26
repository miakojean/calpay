use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, SaltString
    },
    Argon2
};

/// Hache un mot de passe en clair avec Argon2id (paramètres par défaut)
///
/// # Arguments
/// * `password` - Le mot de passe en clair (une chaîne de caractères)
///
/// # Retourne
/// * `Ok(String)` contenant le hash au format PHC string (prêt à être stocké)
/// * `Err(argon2::password_hash::Error)` en cas d'échec (peu probable avec les paramètres par défaut)
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