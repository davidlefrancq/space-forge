use crate::bo::celest_item::CelestItem;
use crate::dal::cache_dao::CacheDAO;
use crate::dal::dao_factory::DAOFactory;
use crate::dal::celest_item_dao::CelestItemDAO;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use std::time::Instant;
use std::sync::Arc;

pub struct Simulator {
  daoFactory: Arc<DAOFactory>,
  cacheDao: Arc<CacheDAO>,
  celestItemDAO: Arc<CelestItemDAO>,
  pub celestItems: Vec<CelestItem>,
}

impl Simulator {
  const REFERENCE_DATE: &'static str = "2000-01-01T12:00:00Z";

  pub fn new(path: &str) -> Self {
    let daoFactory = Arc::new(DAOFactory::new());
    let cacheDao = daoFactory.cacheDAO();
    let celestItemDAO = daoFactory.celestItemDAO();
    let celestItems = celestItemDAO.load_from_file(path).unwrap_or_else(|err| {
      eprintln!("Erreur lors du chargement des planètes : {err}");
      vec![]
    });
    Simulator {
        daoFactory,
        cacheDao,
        celestItemDAO,
        celestItems,
    }
  }

  pub fn run(&self, celestItems: &[CelestItem], target_date: DateTime<Utc>) -> Vec<CelestItem> {
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
    let mut state: Vec<CelestItem> = celestItems.to_vec();

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
        .for_each(|(celestItem, acc)| {
          for k in 0..3 {
            celestItem.velocity[k] += sign * acc[k] * dt;
            celestItem.position[k] += sign * celestItem.velocity[k] * dt;
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

  pub fn load_or_compute(&self, target_date: DateTime<Utc>) -> Vec<CelestItem> {
    if let Some(cached) = self.cacheDao.load_from_cache(target_date) {
      return cached;
    }

    let result = self.run(&self.celestItems, target_date);
    self.cacheDao.save_to_cache(target_date, &result);

    result
  }
}