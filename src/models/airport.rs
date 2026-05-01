use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Airport {
    pub id: Uuid,                 // UUID como en el backend
    pub code: String,             // Código único del aeropuerto
    pub name: String,             // Nombre del aeropuerto
    pub city: Option<String>,     // Puede ser null → Option<String>
    pub country: String,          // Obligatorio
    pub created_at: DateTime<Utc>, // Fecha de creación con zona horaria
}