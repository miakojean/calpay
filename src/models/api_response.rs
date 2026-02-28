use serde::Serialize;
use validator::ValidationErrors; // Import nécessaire pour la méthode utilitaire

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            status: "success".to_string(),
            message: None,
            data: Some(data),
            errors: None,
        }
    }

    pub fn errors(message: &str, errors: Option<serde_json::Value>) -> Self {
        Self {
            status: "error".to_string(),
            message: Some(message.to_string()),
            data: None,
            errors,
        }
    }

    // NOUVEAU : Méthode dédiée pour transformer les erreurs de validation automatiquement
    pub fn validation_error(errors: ValidationErrors) -> ApiResponse<()> {
        ApiResponse {
            status: "error".to_string(),
            message: Some("Erreur de validation des données".to_string()),
            data: None,
            // .field_errors() transforme les erreurs en un format Map lisible par le front
            errors: Some(serde_json::json!(errors.field_errors())),
        }
    }
}