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
    let reference_date = DateTime::parse_from_rfc3339(Self::REFERENCE_DATE)
      .expect("Date de référence invalide")
      .with_timezone(&Utc);

    let delta = target_date.signed_duration_since(reference_date).num_days() as f64;

    // Transformation de la position pour chaque planète
    planets
      .iter()
      .map(|planet| {
        let radius = (planet.position[0].powi(2) + planet.position[1].powi(2)).sqrt();
        if radius == 0.0 {
          return planet.clone(); // Soleil ou corps statique
        }

        let speed = (planet.velocity[0].powi(2) + planet.velocity[1].powi(2)).sqrt();
        let period = TAU * radius / speed; // en secondes

        // Position angulaire
        let angle = TAU * (delta * 86400.0) / period;

        let new_x = radius * angle.cos();
        let new_y = radius * angle.sin();

        let mut new_planet = planet.clone();
        new_planet.position[0] = new_x;
        new_planet.position[1] = new_y;
        new_planet
      })
      .collect()
  }
}
