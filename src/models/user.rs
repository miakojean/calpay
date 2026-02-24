use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    // Validation : minimum 8 caractères pour la sécurité
    #[validate(length(min = 8, message = "Le mot de passe doit faire au moins 8 caractères"))]
    pub password: String, 
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    // Note : On ne renvoie JAMAIS le password dans la réponse API
    #[serde(skip_serializing)] 
    pub password_hash: String,
}