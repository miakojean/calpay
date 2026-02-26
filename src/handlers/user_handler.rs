use actix_web::{HttpResponse, web, Responder};
use uuid::Uuid;
use validator::Validate;
// Note : j'ai simplifié l'import pour plus de clarté
use crate::models::api_response::ApiResponse;
use crate::models::user::{User, CreateUser, ActiveModel};

use sea_orm::{DatabaseConnection};
use sea_orm::{ActiveModelTrait, Set};

// ... tes autres imports

pub async fn create_user(
    db: web::Data<DatabaseConnection>, // 1. AJOUT de la connexion ici !
    user_json: web::Json<CreateUser>   // 2. Renommé pour la clarté
) -> HttpResponse {
    
    // Validation
    if let Err(errors) = user_json.validate() {
        return HttpResponse::BadRequest().json(
            ApiResponse::<()>::validation_error(errors)
        );
    }

    // Sécurité (Hachage)
    let hashed_password = format!("sha256_fake_hash_{}", user_json.password); 

    // 3. Création de l'ActiveModel
    // On utilise user_json (et non user_data)
    let new_user = ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(user_json.username.clone()),
        email: Set(user_json.email.clone()),
        firstname: Set(user_json.firstname.clone()),
        lastname: Set(user_json.lastname.clone()),
        password_hash: Set(hashed_password),
        ..Default::default()
    };

    // 4. Insertion réelle en base
    match new_user.insert(db.get_ref()).await {
        Ok(user_model) => HttpResponse::Created().json(ApiResponse::success(user_model)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&e.to_string()))
    }
}

pub async fn get_user(user_id: web::Path<Uuid>) -> HttpResponse {
    // On crée un mock qui respecte la structure User complète
    let mock_user = User {
        id: *user_id,
        username: String::from("mock_user"),
        email: String::from("mock_user@example.com"),
        firstname: String::from("Jean"),
        lastname: String::from("Dupont"),
        // Même pour un mock, ce champ est requis par le compilateur
        password_hash: String::from("hashed_password_placeholder"), 
    };

    // On renvoie une réponse structurée via ton modèle ApiResponse
    HttpResponse::Ok().json(ApiResponse::success(mock_user))
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("API is up and running!"))
}

pub async fn health_db_check(db: web::Data<DatabaseConnection>) -> impl Responder {
    match db.ping().await {
        Ok(_)=> HttpResponse::Ok().body("la base de données repond parfaitement !"),
        Err(e)=> HttpResponse::InternalServerError().body(format!("Erreur BDD:{}", e))
    }
}