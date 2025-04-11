use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Context, Result};
use serde_json;

use crate::bo::planet::Planet;

pub struct PlanetDAO;

impl PlanetDAO {
  pub fn new() -> Self {
    PlanetDAO
  }

  pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Planet>> {
    let path_ref = path.as_ref();

    let file = File::open(path_ref)
      .context(format!("Échec d'ouverture du fichier {:?}", path_ref))?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
      .context("Erreur de désérialisation JSON vers Vec<Planet>")
  }
}
