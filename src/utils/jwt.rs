use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,      // Subject (User ID)
    pub exp: usize,     // Expiration time (Unix timestamp)
    pub iat: usize,     // Issued at (Unix timestamp)
    pub jti: String,    // JWT ID unique — utilisé pour la blacklist
    pub email: String,  // Email de l'utilisateur
    pub role: String,   // Rôle (ex: "user", "admin")
}

pub fn create_jwt(user_id: Uuid, email: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET doit être définie");

    let now = Utc::now();

    let expiration = now
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        iat: now.timestamp() as usize,
        exp: expiration as usize,
        jti: Uuid::new_v4().to_string(), // ← Identifiant unique par token
        email: email.to_string(),
        role: role.to_string(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

/// Retourne le timestamp Unix actuel — utile pour comparer avec `exp` lors de la purge
pub fn now_timestamp() -> i64 {
    Utc::now().timestamp()
}