use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;
use sea_orm::{EntityTrait, DatabaseConnection};
use actix_web::web::Data;

use crate::utils::jwt::Claims;
use crate::models::api_response::ApiResponse;
use crate::models::revoked_token::Entity as RevokedTokenEntity;

pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub jti: String,
    pub exp: i64, // ← Ajout
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;

    // LocalBoxFuture permet d'utiliser async/await contrairement à Ready
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let secret = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET doit être définie");

        // On extrait la connexion BDD depuis l'état de l'application
        let db = req.app_data::<Data<DatabaseConnection>>().cloned();

        // On extrait le token du header avant de rentrer dans le Future
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        Box::pin(async move {
            // 1. Token présent ?
            let token = match token {
                Some(t) => t,
                None => return Err(unauthorized("Token manquant")),
            };

            // 2. Token valide (signature + expiration) ?
            let claims = match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(data) => data.claims,
                Err(_) => return Err(unauthorized("Token invalide ou expiré")),
            };

            // 3. Token blacklisté ?
            let db = match db {
                Some(d) => d,
                None => return Err(unauthorized("Erreur interne")),
            };

            let is_revoked = RevokedTokenEntity::find_by_id(claims.jti.clone())
                .one(db.get_ref())
                .await
                .map_err(|_| unauthorized("Erreur interne"))?;

            if is_revoked.is_some() {
                return Err(unauthorized("Token révoqué"));
            }

            // 4. Tout est bon — on retourne l'utilisateur authentifié
           Ok(AuthenticatedUser {
                id: claims.sub,
                email: claims.email,
                role: claims.role,
                jti: claims.jti,
                exp: claims.exp as i64, // ← Ajout
            })
        })
    }
}

/// Helper pour éviter la répétition du format d'erreur
fn unauthorized(message: &str) -> Error {
    ErrorUnauthorized(
        serde_json::to_string(
            &ApiResponse::<()>::errors(message, None)
        )
        .unwrap_or_default(),
    )
}