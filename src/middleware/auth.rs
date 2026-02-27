use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use crate::utils::jwt::Claims;
use crate::models::api_response::ApiResponse;

pub struct AuthenticatedUser {
    pub id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error; // ← Était manquant dans ton fichier original

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET doit être définie"); // ← Plus de fallback dangereux

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
                    return ready(Ok(AuthenticatedUser { id: data.claims.sub }));
                }
            }
        }

        // ErrorUnauthorized convertit ton HttpResponse en actix_web::Error
        // ce qui est requis par le type Error = Error ci-dessus
        ready(Err(ErrorUnauthorized(
            serde_json::to_string(
                &ApiResponse::<()>::errors("Accès non autorisé : jeton invalide ou manquant", None)
            )
            .unwrap_or_default(),
        )))
    }
}