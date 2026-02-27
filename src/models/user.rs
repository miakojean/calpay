use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use sea_orm::entity::prelude::*; // Import vital pour les macros

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    #[validate(length(min = 8, message = "Le mot de passe doit faire au moins 8 caractères"))]
    pub password: String, 
}

#[derive(serde::Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User, // Ton modèle User sans le password_hash si possible
}

// --- CONFIGURATION SEA-ORM ---

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "users")] // Nom de la table dans SQLite
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(skip_serializing)] 
    pub password_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Alias pour garder tes habitudes de nommage
pub type User = Model;