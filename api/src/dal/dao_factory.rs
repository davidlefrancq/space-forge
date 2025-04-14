use crate::dal::cache_dao::CacheDAO;
use crate::dal::celest_item_dao::CelestItemDAO;
use crate::dal::db_dao::MongoDBClient;
use crate::dal::celest_item_repository::{CelestItemRepositoryImpl, CelestItemRepository};
use std::sync::Arc;

pub struct DAOFactory {
  cacheDAO: Arc<CacheDAO>,
  celestItemDAO: Arc<CelestItemDAO>,
  mongoClient: Option<Arc<MongoDBClient>>,
  mongoClientIsConnected: bool,
  celestItemRepository: Option<Arc<CelestItemRepositoryImpl>>,
}

impl DAOFactory {
  pub fn new() -> Self {
    DAOFactory {
      cacheDAO: Arc::new(CacheDAO::new()),
      celestItemDAO: Arc::new(CelestItemDAO::new()),
      mongoClient: None,
      mongoClientIsConnected: false,
      celestItemRepository: None,
    }
  }

  pub async fn connect(&mut self, uri: &str, db_name: &str, collection_name: &str) {
    let client = MongoDBClient::new(uri, db_name).await.expect("Mongo init failed.");

    // Création de l'index unique {name, timestamp}
    client.ensure_indexes(collection_name).await.expect("Création d'index échouée");

    self.mongoClientIsConnected = true;
    self.mongoClient = Some(Arc::new(client));
    self.celestItemRepository = Some(Arc::new(CelestItemRepositoryImpl::new(
      Arc::clone(&self.cacheDAO),
      self.mongoClient.clone(),
      Some(collection_name.to_string()),
    )));
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

  pub fn celestItemRepository(&self) -> Arc<CelestItemRepositoryImpl> {
    Arc::clone(self.celestItemRepository.as_ref().expect("Repository not initialized"))
  }
}
