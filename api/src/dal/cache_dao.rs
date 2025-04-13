use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use chrono::{DateTime, Utc};
use serde_json;
use crate::bo::celest_item::CelestItem;

const CACHE_DIR: &str = "data/cache";

pub struct CacheDAO;

impl CacheDAO {
  pub fn new() -> Self {
    CacheDAO
  }

  pub fn load_from_cache(&self, target_date: DateTime<Utc>) -> Option<Vec<CelestItem>> {
    let cache_filename = format!("{}/{}.json", CACHE_DIR, target_date.format("%Y-%m-%d"));

    if let Ok(file) = File::open(&cache_filename) {
      let reader = BufReader::new(file);
      if let Ok(cached) = serde_json::from_reader(reader) {
        println!("♻️ Résultat chargé depuis cache : {}", cache_filename);
        return Some(cached);
      }
    }
    None
  }
  
  pub fn save_to_cache(&self, target_date: DateTime<Utc>, data: &[CelestItem]) {
    let _ = create_dir_all(CACHE_DIR);
    let cache_filename = format!("{}/{}.json", CACHE_DIR, target_date.format("%Y-%m-%d"));

    if let Ok(json) = serde_json::to_string_pretty(data) {
      if let Ok(mut file) = File::create(&cache_filename) {
        let _ = file.write_all(json.as_bytes());
        println!("💾 Résultat sauvegardé dans cache : {}", cache_filename);
      }
    }
  }
}
