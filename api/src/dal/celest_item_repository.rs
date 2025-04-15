use std::env;
use std::{fs::File, io::BufReader, path::Path};
use chrono::{DateTime, Utc};
use anyhow::{Context, Result};
use std::sync::Arc;
use async_trait::async_trait;

use crate::bo::celest_item::CelestItem;
use crate::dal::cache::CachePersistor;
use crate::dal::mongo::MongoPersistor;

/// Enum pour spécifier la cible de persistance
#[derive(Debug, Clone, Copy)]
pub enum PersistenceTarget {
  Cache,
  Mongo,
  All,
}

/// Interface pour lire et écrire des objets célestes
#[async_trait]
pub trait CelestItemRepositoryTrait: Send + Sync {
  async fn save(&self, item: &CelestItem, target: PersistenceTarget) -> Result<()>;
  async fn save_many(&self, items: &[CelestItem], target: PersistenceTarget) -> Result<()>;
  async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Vec<CelestItem>>;
  async fn load_celest_items(&self, file_path: &str) -> Result<Vec<CelestItem>> {
    // Default implementation to load CelestItem from a file
    let file = File::open(file_path)
      .context(format!("Open CelestItem file from path {} as failed", file_path))?;
    let reader = BufReader::new(file);
    let items: Vec<CelestItem> = serde_json::from_reader(reader)
      .context(format!("JSON parse from CelestItem file from path {} as failed.", file_path))?;
    Ok(items)
  }
}

/// Implémentation concrète du Repository
pub struct CelestItemRepository {
  cache: Arc<CachePersistor>,
  mongo: Option<Arc<MongoPersistor>>,
}

impl CelestItemRepository {
  pub async fn new() -> Self {
    let cache = Arc::new(CachePersistor::new());
    let mut mongo = None;

    let mongo_uri = env::var("MONGO_URI").ok();
    let mongo_db = env::var("MONGO_DB_NAME").ok();
    let mongo_collection = env::var("MONGO_COLLECTION_NAME").ok();

    mongo = if let (Some(uri), Some(db), Some(coll)) = (mongo_uri, mongo_db, mongo_collection) {
      let mongo_persistor = MongoPersistor::new(&uri, &db, &coll).await;
      Some(Arc::new(mongo_persistor))
    } else {
      tracing::warn!("MongoDB config not found: skipping Mongo persistence.");
      println!("⚠️  Wrong MongoDB config: skipping connection.");
      None
    };

    if !mongo.is_none() {
      tracing::info!("MongoDB connection is successful.");
    }

    CelestItemRepository { cache, mongo }
  }
}

#[async_trait]
impl CelestItemRepositoryTrait for CelestItemRepository {
  async fn save(&self, item: &CelestItem, target: PersistenceTarget) -> Result<()> {
    self.save_many(std::slice::from_ref(item), target).await
  }

  async fn save_many(&self, items: &[CelestItem], target: PersistenceTarget) -> Result<()> {
      if items.is_empty() {
        return Ok(());
      }

      // Persistence with files
      match target {
        PersistenceTarget::Cache | PersistenceTarget::All => {
          if let Some(date) = items[0].timestamp {
            self.cache.save(date, items).await?;
          } else {
            tracing::warn!("Tried to save to cache with missing timestamp");
          }
        }
        _ => {}
      }

      // Persistence with MongoDB
      if matches!(target, PersistenceTarget::Mongo | PersistenceTarget::All) {
        if let Some(mongo) = &self.mongo {
          mongo.save_many(items).await?;
        }
      }

      Ok(())
  }

  async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Vec<CelestItem>> {
    // Find in MongoDB
    tracing::info!("Searching in MongoDB for date: {}", date);
    if let Some(mongo) = &self.mongo {
      let results = mongo.find_by_date(date).await?;
      if !results.is_empty() {
        return Ok(results);
      }
    }
    tracing::info!("No results found in MongoDB for date: {}", date);

    // Find in cache files
    // let results = self.cache.find_by_date(date).await?;
    // Ok(results.unwrap_or_default())
    return Ok(vec![]);
  }
}