use mongodb::bson::{doc};
use chrono::{DateTime, Utc};
use anyhow::Result;
use std::sync::Arc;

use crate::bo::celest_item::CelestItem;
use crate::dal::{cache_dao::CacheDAO, db_dao::MongoDBClient};

/// Enum pour spécifier le backend de persistance
#[derive(Debug, Clone, Copy)]
pub enum PersistenceTarget {
    Cache,
    Mongo,
    Postgres,
    All,
}

/// Interface pour lire et écrire des objets célestes
pub trait CelestItemRepository {
    async fn save(&self, item: &CelestItem, target: PersistenceTarget) -> Result<()>;
    async fn save_many(&self, items: &[CelestItem], target: PersistenceTarget) -> Result<()>;
    async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Vec<CelestItem>>;
}

/// Implémentation concrète de l’interface de repository
pub struct CelestItemRepositoryImpl {
  cache: Arc<CacheDAO>,
  mongo: Option<Arc<MongoDBClient>>,
  collection_name: Option<String>,
}

impl CelestItemRepositoryImpl {
  pub fn new(cache: Arc<CacheDAO>, mongo: Option<Arc<MongoDBClient>>, collection_name: Option<String>) -> Self {
      Self { cache, mongo, collection_name }
  }
}

impl CelestItemRepository for CelestItemRepositoryImpl {
  async fn save(&self, item: &CelestItem, target: PersistenceTarget) -> Result<()> {
    self.save_many(std::slice::from_ref(item), target).await
  }

  async fn save_many(&self, items: &[CelestItem], target: PersistenceTarget) -> Result<()> {
    if items.is_empty() {
      return Ok(());
    }

    match target {
      PersistenceTarget::Cache | PersistenceTarget::All => {
        println!("Datetime : {:?}", items[0].timestamp);
        if let Some(date) = items[0].timestamp {
            self.cache.save_to_cache(date, items);
        }
      }
      _ => {}
    }

    if matches!(target, PersistenceTarget::Mongo | PersistenceTarget::All) {
      if let Some(mongo) = &self.mongo {
        let collection_name: &str = &self.collection_name
          .as_deref()
          .unwrap_or("celestia");
        for item in items {
          mongo.insert_one(collection_name, item).await?; // collection name à discuter
        }
      }
    }

    // PostgreSQL : à implémenter plus tard

    Ok(())
  }

  async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Vec<CelestItem>> {
    let date_utc_string = date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let collection_name: &str = &self.collection_name
      .as_deref()
      .unwrap_or("celestia");

    // Load from DB first
    if let Some(mongo) = &self.mongo {
      let results = mongo.find_all::<CelestItem>(
        collection_name,
        Some(doc! {
          "timestamp": date_utc_string,
        }),
      ).await?;
      if !results.is_empty() {
        return Ok(results);
      }
    }

    // Else load from Cache
    if let Some(data) = self.cache.load_from_cache(date) {
      return Ok(data);
    }

    Ok(vec![])
  }
}