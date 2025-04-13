use crate::dal::cache_dao::CacheDAO;
use crate::dal::celest_item_dao::CelestItemDAO;
use std::sync::Arc;

pub struct DAOFactory {
  cacheDAO: Arc<CacheDAO>,
  celestItemDAO: Arc<CelestItemDAO>
}

impl DAOFactory {
  pub fn new() -> Self {
    DAOFactory {
      cacheDAO: Arc::new(CacheDAO::new()),
      celestItemDAO: Arc::new(CelestItemDAO::new())
    }
  }

  pub fn celestItemDAO(&self) -> Arc<CelestItemDAO> {
    Arc::clone(&self.celestItemDAO)
  }

  pub fn cacheDAO(&self) -> Arc<CacheDAO> {
    Arc::clone(&self.cacheDAO)
  }
}
