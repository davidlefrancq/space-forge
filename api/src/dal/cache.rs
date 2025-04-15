use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use std::path::Path;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use serde_json;

use crate::bo::celest_item::CelestItem;

const CACHE_DIR: &str = "data/cache";

/// JSON Files Persistence
pub struct CachePersistor;

impl CachePersistor {
  pub fn new() -> Self {
    CachePersistor
  }

  fn cache_path(date: DateTime<Utc>) -> String {
    format!("{}/{}.json", CACHE_DIR, date.format("%Y-%m-%d"))
  }

  pub async fn save(&self, date: DateTime<Utc>, items: &[CelestItem]) -> Result<()> {
    create_dir_all(CACHE_DIR).context("Impossible de créer le dossier cache")?;
    let path = Self::cache_path(date);
    let json = serde_json::to_string_pretty(items)?;
    let mut file = File::create(&path).context("Erreur lors de la création du fichier de cache")?;
    file.write_all(json.as_bytes())?;
    println!("💾 Cache sauvegardé : {}", path);
    Ok(())
  }

  pub async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Option<Vec<CelestItem>>> {
    let path = Self::cache_path(date);
    if !Path::new(&path).exists() {
        return Ok(None);
    }

    let file = File::open(&path).context("Erreur lors de l'ouverture du fichier de cache")?;
    let reader = BufReader::new(file);
    let items: Vec<CelestItem> = serde_json::from_reader(reader)
        .context("Erreur de désérialisation depuis le cache")?;
    println!("♻️ Données chargées depuis cache : {}", path);
    Ok(Some(items))
  }
}
