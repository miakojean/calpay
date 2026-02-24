use actix_web::{HttpResponse, web};
use uuid::Uuid;
use validator::Validate;
// Note : j'ai simplifié l'import pour plus de clarté
use crate::models::api_response::ApiResponse;
use crate::models::user::{User, CreateUser};

pub async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
    // 1. Validation (inclut maintenant la longueur du password)
    if let Err(errors) = user.validate() {
        return HttpResponse::BadRequest().json(
            ApiResponse::<()>::validation_error(errors)
        );
    }

    // 2. Sécurité : Hachage du mot de passe
    // Dans la vraie vie : let hashed_password = bcrypt::hash(&user.password, 10).unwrap();
    let hashed_password = format!("sha256_fake_hash_{}", user.password); 

    // 3. Création de l'objet User
    let new_user = User {
        id: Uuid::new_v4(),
        username: user.username.clone(),
        email: user.email.clone(),
        firstname: user.firstname.clone(),
        lastname: user.lastname.clone(),
        password_hash: hashed_password, // On stocke le hash, pas le mot de passe
    };

    // 4. Réponse
    // Grâce à #[serde(skip_serializing)], password_hash ne sera pas dans le JSON final
    HttpResponse::Created().json(ApiResponse::success(new_user))
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