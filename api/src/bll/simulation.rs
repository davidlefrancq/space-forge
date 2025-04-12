use crate::bo::planet::Planet;
use crate::dal::dao_factory::DAOFactory;
use chrono::{DateTime, TimeZone, Utc};
use std::f64::consts::TAU; // TAU = 2π

pub struct Simulation;

impl Simulation {
  const REFERENCE_DATE: &'static str = "2000-01-01T12:00:00Z";
  
  /// Charge les planètes depuis un fichier JSON
  pub fn load_planets(path: &str) -> Vec<Planet> {
    let daoFactory = DAOFactory::new();
    match daoFactory.planetDAO().load_from_file(path) {
      Ok(planets) => planets,
      Err(err) => {
        eprintln!("Erreur lors du chargement des planètes : {err}");
        vec![]
      }
    }
  }

  pub fn run(planets: &[Planet], target_date: DateTime<Utc>) -> Vec<Planet> {
    const G: f64 = 6.67430e-11; // constante gravitationnelle
    let reference_date = DateTime::parse_from_rfc3339(Self::REFERENCE_DATE)
      .expect("Date de référence invalide")
      .with_timezone(&Utc);
    let delta_seconds = target_date.signed_duration_since(reference_date).num_seconds() as f64;
    let dt = 3600.0; // pas de temps en secondes (1h)

    let steps = (delta_seconds / dt).abs() as usize;
    let sign = if delta_seconds >= 0.0 { 1.0 } else { -1.0 };

    let mut state: Vec<Planet> = planets.to_vec();

    for _ in 0..steps {
      // Accélérations initialisées à zéro
      let mut accelerations = vec![[0.0; 3]; state.len()];

      // Calcul des forces gravitationnelles
      for i in 0..state.len() {
        for j in 0..state.len() {
          if i == j {
            continue;
          }

          let dx = state[j].position[0] - state[i].position[0];
          let dy = state[j].position[1] - state[i].position[1];
          let dz = state[j].position[2] - state[i].position[2];

          let r_squared = dx*dx + dy*dy + dz*dz;
          let r = r_squared.sqrt();

          if r < 1e3 { continue; } // éviter la division par zéro ou collisions

          let force_mag = G * state[j].mass / r_squared;
          accelerations[i][0] += force_mag * dx / r;
          accelerations[i][1] += force_mag * dy / r;
          accelerations[i][2] += force_mag * dz / r;
        }
      }

      // Mise à jour des positions et vitesses (Euler)
      for (planet, acc) in state.iter_mut().zip(accelerations.iter()) {
        for k in 0..3 {
          planet.velocity[k] += sign * acc[k] * dt;
          planet.position[k] += sign * planet.velocity[k] * dt;
        }
      }
    }

    state
  }
}
