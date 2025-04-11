use crate::bo::planet::Planet;
use crate::dal::dao_factory::DAOFactory;

pub struct Simulation;

impl Simulation {
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

  /// Simule les données du système (placeholder)
  pub fn run(planets: &[Planet], duration_days: u32) -> Vec<Planet> {
    // TODO: intégrer des calculs dynamiques ici
    planets.to_vec() // pour l'instant, on renvoie les données brutes
  }
}
