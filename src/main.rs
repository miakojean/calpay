mod handlers;
mod models;
mod routes;
mod utils;
mod middleware;

use actix_web::{web, App, HttpServer}; // Ajout de 'web' ici
use dotenv::dotenv;
use std::env;
use sea_orm::{Database, DatabaseConnection};

use routes::user_routes::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Charger les variables d'environnement (.env)
    dotenv().ok();

    let _ = env::var("JWT_SECRET")
    .expect("JWT_SECRET non définie dans le fichier .env");
    
    // 2. Récupérer l'URL de la base de données
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL non définie dans le fichier .env");
    
    // 3. Établir la connexion (Fail-fast : le programme crash si la BDD est inaccessible)
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Impossible de se connecter à la base de données");

    // 4. Configuration de l'hôte et du port
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Serveur lancé sur http://{}:{}", host, port);

    // 5. Lancement du serveur
    HttpServer::new(move || {
        App::new()
            // IMPORTANT : On injecte la connexion clonée dans l'état de l'application
            // SeaORM utilise des Arc en interne, donc .clone() est très peu coûteux.
            .app_data(web::Data::new(db.clone())) 
            .configure(init) // On passe la config aux routes
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}