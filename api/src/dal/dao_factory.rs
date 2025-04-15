use std::sync::Arc;

use crate::dal::celest_item_dao::CelestItemDAO;
use crate::dal::celest_item_repository::{PersistenceTarget};

pub struct DAOFactory {
    celest_item_dao: Arc<CelestItemDAO>,
}

impl DAOFactory {
    /// Creates a new DAOFactory with cache and optional MongoDB persistence
    pub async fn new() -> Self {
      let target = PersistenceTarget::Mongo;
      let celest_item_dao = Arc::new(CelestItemDAO::new(target).await);
      Self { celest_item_dao }
    }

    pub fn celest_item_dao(&self) -> Arc<CelestItemDAO> {
      Arc::clone(&self.celest_item_dao)
    }
}
