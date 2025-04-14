use mongodb::{
  IndexModel,
  bson::{doc, Document},
  options::{ ClientOptions, IndexOptions },
  Client, Collection, Database,
};

use futures::stream::TryStreamExt;

pub struct MongoDBClient {
    client: Client,
    database: Database,
}

impl MongoDBClient {
    pub async fn new(uri: &str, db_name: &str) -> mongodb::error::Result<Self> {
      let mut client_options = ClientOptions::parse(uri).await?;
      client_options.app_name = Some("celestia".to_string());
      let client = Client::with_options(client_options)?;
      let database = client.database(db_name);
      Ok(Self { client, database })
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T> where T: serde::Serialize + Unpin + Send + Sync, {
      self.database.collection::<T>(name)
    }
  
    pub async fn insert_one<T> (
      &self,
      collection_name: &str,
      document: &T,
    ) -> mongodb::error::Result<()> where T: serde::Serialize + Unpin + Send + Sync, {
      let collection = self.collection::<T>(collection_name);
      collection.insert_one(document).await?;
      Ok(())
    }

    pub async fn find_all<T>(
      &self,
      collection_name: &str,
      filter: Option<Document>,
    ) -> mongodb::error::Result<Vec<T>> where T: for<'de> serde::Deserialize<'de> + serde::Serialize + Unpin + Send + Sync, {
        let collection = self.collection::<T>(collection_name);
        let cursor = collection.find(filter.unwrap_or_else(|| doc! {})).await?;
        let results = cursor.try_collect().await?;
        Ok(results)
    }

    pub async fn ensure_indexes(&self, collection_name: &str) -> mongodb::error::Result<()> {
      let collection = self.collection::<Document>(collection_name);

      let index_model = IndexModel::builder()
        .keys(doc! { "name": 1, "timestamp": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();

      collection.create_index(index_model).await?;

      Ok(())
    }
}
