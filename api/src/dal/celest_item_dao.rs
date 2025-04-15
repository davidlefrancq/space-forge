use chrono::{DateTime, Utc};
use anyhow::Result;
use std::sync::Arc;

use crate::bo::celest_item::CelestItem;
use crate::dal::celest_item_repository::{CelestItemRepository, CelestItemRepositoryTrait, PersistenceTarget};

pub struct CelestItemDAO {
  repository: Arc<dyn CelestItemRepositoryTrait>,
  persistence_target: PersistenceTarget,
}

impl CelestItemDAO {
  pub async fn new(persistence_target: PersistenceTarget) -> Self {
    let repository = Arc::new(CelestItemRepository::new().await);
    Self { repository, persistence_target }
  }

  /// Load CelestItem list from a file
  pub async fn load_celest_items(&self, file_path: &str) -> Result<Vec<CelestItem>> {
    self.repository.load_celest_items(file_path).await
  }

  /// Load CelestItem list for a given simulation date
  pub async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Vec<CelestItem>> {
    self.repository.find_by_date(date).await
  }

  /// Load CelestItem list for a given date range from cache
  pub async fn find_by_dates(&self, start: DateTime<Utc>, stop: DateTime<Utc>) -> Result<Vec<CelestItem>> {
    self.repository.find_by_dates(start, stop).await
  }

  /// Save simulation results
  pub async fn save_many(&self, items: &[CelestItem]) -> Result<()> {
    self.repository.save_many(items, self.persistence_target).await
  }
}
