use actix_web::{dev::Payload, FromRequest, HttpRequest, HttpResponse};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use crate::utils::jwt::Claims;
use crate::models::api_response::ApiResponse;

pub struct AuthenticatedUser {
    pub id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Récupération du secret (Assure-toi que JWT_SECRET est dans ton .env)
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_temporaire".to_string());
        
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_str) = auth_header.and_then(|h| h.to_str().ok()) {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::default(),
                );

                if let Ok(data) = token_data {
                    // Si le token est valide, on retourne l'ID de l'utilisateur
                    return ready(Ok(AuthenticatedUser { id: data.claims.sub }));
                }
            }
        }

        // Si on arrive ici, l'auth a échoué. On utilise ton format ApiResponse standard.
        ready(Err(
            HttpResponse::Unauthorized().json(
                ApiResponse::<()>::errors("Accès non autorisé : jeton invalide ou manquant", None)
            )
        ))
    }
}