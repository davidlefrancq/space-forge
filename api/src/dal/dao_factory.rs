use crate::dal::cache_dao::CacheDAO;
use crate::dal::celest_item_dao::CelestItemDAO;
use crate::dal::db_dao::MongoDBClient;
use std::sync::Arc;

pub struct DAOFactory {
  cacheDAO: Arc<CacheDAO>,
  celestItemDAO: Arc<CelestItemDAO>,
  mongoClient: Option<Arc<MongoDBClient>>,
  mongoClientIsConnected: bool,
}

impl DAOFactory {
  pub fn new() -> Self {
    DAOFactory {
      cacheDAO: Arc::new(CacheDAO::new()),
      celestItemDAO: Arc::new(CelestItemDAO::new()),
      mongoClient: None,
      mongoClientIsConnected: false,
    }
  }

  pub async fn connect(&mut self, uri: &str, db_name: &str) {
    let client = MongoDBClient::new(uri, db_name).await.expect("Mongo init failed.");
    self.mongoClientIsConnected = true;
    self.mongoClient = Some(Arc::new(client));
  }

  pub fn celestItemDAO(&self) -> Arc<CelestItemDAO> {
    Arc::clone(&self.celestItemDAO)
  }

  pub fn cacheDAO(&self) -> Arc<CacheDAO> {
    Arc::clone(&self.cacheDAO)
  }

  pub fn mongoClient(&self) -> Option<Arc<MongoDBClient>> {
    if (self.mongoClient.is_none()) {
      panic!("MongoDB client is not connected.");
    }
    self.mongoClient.clone()
  }
  
}
