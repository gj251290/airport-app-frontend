use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Airline {
    pub id: Uuid,                // UUID como en el backend
    pub code: String,            // Código único de la aerolínea
    pub name: String,            // Nombre de la aerolínea
    pub country: Option<String>, // Puede ser null → Option<String>
    pub created_at: DateTime<Utc>, // Fecha de creación con zona horaria
}