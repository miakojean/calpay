use actix_web::{HttpResponse, web, Responder};
use uuid::Uuid;
use validator::Validate;
use crate::models::api_response::ApiResponse;
use crate::models::user::{User, Entity as UserEntity, CreateUser, ActiveModel, AuthResponse};

use sea_orm::DatabaseConnection;
use sea_orm::{ActiveModelTrait, Set};

use crate::utils::password::hash_password;
use crate::utils::password::verify_password;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use crate::models::user::LoginRequest;
use crate::utils::jwt::create_jwt;

// Imports pour le logout
use chrono::Utc;
use crate::models::revoked_token::{
    ActiveModel as RevokedTokenActiveModel,
    Entity as RevokedTokenEntity,
    Column as RevokedTokenColumn,
};
use crate::middleware::auth::AuthenticatedUser;
use crate::utils::jwt::now_timestamp;

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user_json: web::Json<CreateUser>
) -> HttpResponse {
    if let Err(errors) = user_json.validate() {
        return HttpResponse::BadRequest().json(
            ApiResponse::<()>::validation_error(errors)
        );
    }

    let hashed_password = match hash_password(&user_json.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::errors("Erreur lors du hachage du mot de passe", None)),
    };

    let new_user = ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(user_json.username.clone()),
        email: Set(user_json.email.clone()),
        firstname: Set(user_json.firstname.clone()),
        lastname: Set(user_json.lastname.clone()),
        password_hash: Set(hashed_password),
        is_active: Set(user_json.is_active.clone()),
        ..Default::default()
    };

    match new_user.insert(db.get_ref()).await {
        Ok(user_model) => HttpResponse::Created().json(ApiResponse::success(user_model, "Nouvel utilisateur enrégistré avec succès")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::errors(&e.to_string(), None))
    }
}

pub async fn get_user(user_id: web::Path<Uuid>) -> HttpResponse {
    let mock_user = User {
        id: *user_id,
        username: String::from("mock_user"),
        email: String::from("mock_user@example.com"),
        firstname: String::from("Jean"),
        lastname: String::from("Dupont"),
        password_hash: String::from("hashed_password_placeholder"),
        is_active: bool::from(true),
        role: String::from("user")
    };

    HttpResponse::Ok().json(ApiResponse::success(mock_user, "Utilisateur récupéré avec succès"))
}

pub async fn login(
    db: web::Data<DatabaseConnection>,
    login_json: web::Json<LoginRequest>
) -> HttpResponse {
    if let Err(errors) = login_json.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::validation_error(errors));
    }

    let user_result = UserEntity::find()
        .filter(crate::models::user::Column::Email.eq(login_json.email.clone()))
        .one(db.get_ref())
        .await;

    match user_result {
        Ok(Some(user)) => {
            match verify_password(&login_json.password, &user.password_hash) {
                Ok(true) => {
                    match create_jwt(user.id, &user.email, &user.role) {
                        Ok(token) => {
                            let response = AuthResponse { token, user };
                            HttpResponse::Ok().json(ApiResponse::success(response, "Connexion réussie"))
                        },
                        Err(_) => HttpResponse::InternalServerError().json(
                            ApiResponse::<()>::errors("Échec de génération du jeton", None)
                        ),
                    }
                },
                _ => HttpResponse::Unauthorized().json(
                    ApiResponse::<()>::errors("Identifiants invalides", None)
                ),
            }
        },
        Ok(None) => HttpResponse::Unauthorized().json(
            ApiResponse::<()>::errors("Identifiants invalides", None)
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::errors(&e.to_string(), None)
        ),
    }
}

pub async fn logout(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser, // ← Vérifie le token ET fournit le jti automatiquement
) -> HttpResponse {
    // 1. Insertion du jti en blacklist
    let revoked = RevokedTokenActiveModel {
        jti: Set(user.jti.clone()),
        user_id: Set(user.id),
        exp: Set(user.exp),
        revoked_at: Set(Utc::now().into()),
    };

    match revoked.insert(db.get_ref()).await {
        Ok(_) => {
            // 2. Purge opportuniste des tokens expirés
            let now = now_timestamp();
            let _ = RevokedTokenEntity::delete_many()
                .filter(RevokedTokenColumn::Exp.lt(now))
                .exec(db.get_ref())
                .await;

            HttpResponse::Ok().json(ApiResponse::<()>::errors("Déconnexion réussie", None))
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::errors(&e.to_string(), None)
        ),
    }
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("API is up and running!", "Serveur en marche"))
}

pub async fn health_db_check(db: web::Data<DatabaseConnection>) -> impl Responder {
    match db.ping().await {
        Ok(_) => HttpResponse::Ok().body("La base de données répond parfaitement !"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur BDD : {}", e))
    }
}