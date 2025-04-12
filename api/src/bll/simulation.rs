use crate::bo::planet::Planet;
use chrono::{DateTime, Utc};
use rayon::prelude::*; // 🚀 Parallélisation
use std::time::Instant;
use std::sync::Arc;
use std::fs::{File, create_dir_all};
use std::io::{BufReader, Write};
use serde_json;

pub struct Simulation;

impl Simulation {
  const REFERENCE_DATE: &'static str = "2000-01-01T12:00:00Z";

  /// Charge les planètes depuis un fichier JSON
  pub fn load_planets(path: &str) -> Vec<Planet> {
    use crate::dal::dao_factory::DAOFactory;
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
    let start = Instant::now();
    const G: f64 = 6.67430e-11;
    let reference_date = DateTime::parse_from_rfc3339(Self::REFERENCE_DATE)
      .expect("Date de référence invalide")
      .with_timezone(&Utc);
    let delta_seconds = target_date.signed_duration_since(reference_date).num_seconds() as f64;

    // Pas de temps initial (1h)
    let mut dt = 3600.0;
    let mut steps = (delta_seconds / dt).abs() as usize;
    let max_steps = 10_000;
    if steps > max_steps {
      dt = delta_seconds.abs() / max_steps as f64;
      steps = max_steps;
    }

    let sign = if delta_seconds >= 0.0 { 1.0 } else { -1.0 };
    let mut state: Vec<Planet> = planets.to_vec();

    for _ in 0..steps {
      let shared_state = Arc::new(state.clone());

      // 1. Calcul des accélérations en parallèle
      let accelerations: Vec<[f64; 3]> = (0..shared_state.len())
        .into_par_iter()
        .map(|i| {
          let mut acc = [0.0; 3];
          for (j, other) in shared_state.iter().enumerate() {
            if i == j {
              continue;
            }

            let self_p = &shared_state[i];
            let dx = other.position[0] - self_p.position[0];
            let dy = other.position[1] - self_p.position[1];
            let dz = other.position[2] - self_p.position[2];

            let r_squared = dx * dx + dy * dy + dz * dz;
            let r = r_squared.sqrt();

            if r < 1e3 {
              continue;
            }

            let force_mag = G * other.mass / r_squared;
            acc[0] += force_mag * dx / r;
            acc[1] += force_mag * dy / r;
            acc[2] += force_mag * dz / r;
          }
          acc
        })
        .collect();

      // 2. Mise à jour des vitesses et positions en parallèle
      state
        .par_iter_mut()
        .zip(accelerations.par_iter())
        .for_each(|(planet, acc)| {
          for k in 0..3 {
            planet.velocity[k] += sign * acc[k] * dt;
            planet.position[k] += sign * planet.velocity[k] * dt;
          }
        });
    }
    
    let duration = start.elapsed();
    tracing::info!(
      "⏱️ Simulation terminée en {} secondes",
      duration.as_secs_f64()
    );

    state
  }

  pub fn load_or_compute(planets: &[Planet], target_date: DateTime<Utc>) -> Vec<Planet> {
    let cache_dir = "data/cache";
    let cache_filename = format!("{}/{}.json", cache_dir, target_date.format("%Y-%m-%d"));

    // Crée le dossier de cache s'il n'existe pas
    let _ = create_dir_all(cache_dir);

    // Essaie de charger depuis cache
    if let Ok(file) = File::open(&cache_filename) {
      let reader = BufReader::new(file);
      if let Ok(cached) = serde_json::from_reader(reader) {
        println!("♻️ Résultat chargé depuis cache : {}", cache_filename);
        return cached;
      }
    }

    // Sinon, calcule et sauvegarde
    let result = Simulation::run(planets, target_date);

    if let Ok(json) = serde_json::to_string_pretty(&result) {
      if let Ok(mut file) = File::create(&cache_filename) {
        let _ = file.write_all(json.as_bytes());
        println!("💾 Résultat sauvegardé dans cache : {}", cache_filename);
      }
    }

    result
  }
}