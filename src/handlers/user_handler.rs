use actix_web::{HttpResponse, web, Responder};
use serde::de::value::BoolDeserializer;
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
        Ok(user_model) => HttpResponse::Created().json(ApiResponse::success(user_model)),
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
        is_active: bool::from(true)
    };

    HttpResponse::Ok().json(ApiResponse::success(mock_user))
}

pub async fn login(
    db: web::Data<DatabaseConnection>,
    login_json: web::Json<LoginRequest>
) -> HttpResponse {
    // 1. Validation des champs
    if let Err(errors) = login_json.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::validation_error(errors));
    }

    // 2. Recherche de l'utilisateur par email
    let user_result = UserEntity::find()
        .filter(crate::models::user::Column::Email.eq(login_json.email.clone()))
        .one(db.get_ref())
        .await;

    match user_result {
        Ok(Some(user)) => {
            // 3. Vérification du mot de passe
            match verify_password(&login_json.password, &user.password_hash) {
                Ok(true) => {
                    // 4. Génération du JWT
                    match create_jwt(user.id) {
                        Ok(token) => {
                            // 5. Réponse finale : token + user (password_hash masqué par Serde)
                            let response = AuthResponse { token, user };
                            HttpResponse::Ok().json(ApiResponse::success(response))
                            //                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                            // Un seul niveau de wrapping — corrige le double ApiResponse
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

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("API is up and running!"))
}

pub async fn health_db_check(db: web::Data<DatabaseConnection>) -> impl Responder {
    match db.ping().await {
        Ok(_) => HttpResponse::Ok().body("La base de données répond parfaitement !"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur BDD : {}", e))
    }
}