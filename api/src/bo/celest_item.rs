use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CelestItem {
    pub name: String,
    pub mass: f64,       // en kilogrammes
    pub radius: f64,     // en mètres
    pub position: [f64; 3], // en mètres
    pub velocity: [f64; 3], // en m/s
    pub timestamp: Option<DateTime<Utc>>, // date de la simulation
}