use std::sync::Arc;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use mongodb::{
  bson::{doc, DateTime as BsonDateTime, Document}, options::{ ClientOptions, FindOptions, IndexOptions }, Client, Collection, Database, IndexModel
};
use futures::TryStreamExt;

use crate::bo::celest_item::CelestItem;

pub struct MongoDBClient {
  client: Client,
  database: Database,
  indexed: bool,
}

impl MongoDBClient {
  pub async fn new(uri: &str, db_name: &str) -> mongodb::error::Result<Self> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("celestia".to_string());
    let client = Client::with_options(client_options)?;
    let database = client.database(db_name);
    let indexed = false;
    Ok(Self { client, database, indexed })
  }

  pub fn collection<T>(&self, name: &str) -> Collection<T> where T: serde::Serialize + Unpin + Send + Sync, {
    self.database.collection::<T>(name)
  }

  pub async fn insert_one<T> (
    &self,
    collection_name: &str,
    document: &T,
  ) -> mongodb::error::Result<()> where T: serde::Serialize + Unpin + Send + Sync, {
    let is_indexed = self.is_exists_indexes(collection_name).await;
    if !is_indexed {
      self.ensure_name_timestamp_indexes(collection_name).await.expect("Failed to ensure indexes");
      tracing::info!("🔍 Indexes created for collection {}", collection_name);
    }
    let collection = self.collection::<T>(collection_name);
    let result = collection.insert_one(document).await.expect("Failed to insert document into MongoDB");
    tracing::info!("🌐 Document inserted with id: {}", result.inserted_id);
    Ok(())
  }

  pub async fn find_all<T>(
    &self,
    collection_name: &str,
    filter: Option<Document>,
  ) -> mongodb::error::Result<Vec<T>> where T: for<'de> serde::Deserialize<'de> + serde::Serialize + Unpin + Send + Sync, {
    let collection = self.collection::<T>(collection_name);
    let cursor = collection.find(filter.unwrap_or_else(|| doc! {})).await.expect("Failed to find documents in MongoDB");
    let results = cursor.try_collect().await.expect("Failed to collect results from MongoDB");
    Ok(results)
  }

  async fn is_exists_indexes(&self, collection_name: &str) -> bool {
    if self.indexed {
      return true;
    }

    // Check MongoDB Index { name: 1, timestamp: 1 } exists
    // Get the collection as a BSON Document collection (for listing raw indexes)
    let collection = self.collection::<Document>(collection_name);

    // Try to call `list_indexes()` on this collection
    // This returns a MongoDB cursor if successful
    let cursor = match collection.list_indexes().await {
      Ok(c) => {
        // Successfully retrieved the index cursor
        c
      }
      Err(err) => {
        // Could not retrieve indexes — likely a Mongo error
        eprintln!("⚠️ Failed to call list_indexes(): {}", err);
        return false;
      }
    };

    // Try to collect all index definitions from the cursor into a vector
    let existing_indexes: Vec<IndexModel> = match cursor.try_collect().await {
      Ok(indexes) => {
        // Successfully gathered all index metadata
        indexes
      }
      Err(err) => {
        // Something went wrong while consuming the cursor
        eprintln!("⚠️ Failed to collect index list from cursor: {}", err);
        return false;
      }
    };

    // Iterate over all indexes and check if any of them matches the keys we want
    let mut already_exists = false;
    for index in existing_indexes.iter() {
      // Access the keys field of the index. It is a BSON Document.
      let keys: &Document = &index.keys;

      // Try to extract the "name" key as an integer
      let name_ok = match keys.get("name") {
        Some(bson_value) => bson_value.as_i32() == Some(1),
        None => false,
      };

      // Try to extract the "timestamp" key as an integer
      let timestamp_ok = match keys.get("timestamp") {
        Some(bson_value) => bson_value.as_i32() == Some(1),
        None => false,
      };

      // If both keys match, we found our index
      if name_ok && timestamp_ok {
        already_exists = true;
        break;
      }
    }
    return already_exists;
  }

  async fn ensure_name_timestamp_indexes(&self, collection_name: &str) -> mongodb::error::Result<()> {
    let collection = self.collection::<Document>(collection_name);

    let index_model = IndexModel::builder()
      .keys(doc! { "name": 1, "timestamp": 1 })
      .options(IndexOptions::builder().unique(true).build())
      .build();

    collection.create_index(index_model).await?;

    Ok(())
  }
}

/// Persister MongoDB pour CelestItem
pub struct MongoPersistor {
  client: Arc<MongoDBClient>,
  collection_name: String,
}

impl MongoPersistor {
  pub async fn new(uri: &str, db_name: &str, collection_name: &str) -> Self {
    let mongo_client = MongoDBClient::new(uri, db_name).await.expect("Failed to create MongoDB client.");
    MongoPersistor {
      client: Arc::new(mongo_client),
      collection_name: collection_name.to_string(),
    }
  }

  pub async fn save_many(&self, items: &[CelestItem]) -> Result<()> {
    let collection: Collection<CelestItem> = self.client.collection(&self.collection_name);

    for item in items {
      // Utilisation de `insert_one`, on peut ajouter upsert plus tard si besoin
      collection
        .insert_one(item)
        .await
        .context("Échec de l'insertion Mongo")?;
    }

    println!("🌐 Données enregistrées dans Mongo : {}", items.len());
    Ok(())
  }

  pub async fn find_by_date(&self, date: DateTime<Utc>) -> Result<Vec<CelestItem>> {
    let collection: Collection<CelestItem> = self.client.collection(&self.collection_name);

    let filter = doc! {
      "timestamp": date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    };

    let cursor = collection
      .find(filter)
      .await
      .context("Erreur lors de la requête Mongo")?;

    let results: Vec<CelestItem> = cursor
      .try_collect()
      .await
      .context("Erreur de lecture des résultats Mongo")?;

    Ok(results)
  }

  pub async fn find_by_dates(&self, start: DateTime<Utc>, stop: DateTime<Utc>) -> Result<Vec<CelestItem>> {
    let collection: Collection<CelestItem> = self.client.collection(&self.collection_name);

    let filter = doc! {
      "timestamp": {
        "$gte": start.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "$lte": stop.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
      }
    };

    let options = FindOptions::builder().limit(100).build();

    let cursor = collection
      .find(filter)
      .with_options(options)
      .await
      .context("Erreur lors de la requête Mongo")?;

    let results: Vec<CelestItem> = cursor
      .try_collect()
      .await
      .context("Erreur de lecture des résultats Mongo")?;

    Ok(results)
  }
}
