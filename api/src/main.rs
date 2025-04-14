use std::env;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use dal::dao_factory::DAOFactory;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::time::Instant;
use std::sync::Arc;

mod bo;
mod bll;
mod dal;
mod utils;

use bll::simulator::Simulator;
use utils::logger_factory::LoggerFactory;

#[get("/")]
async fn home() -> impl Responder {
  HttpResponse::Ok().body("ok")
}

#[derive(Deserialize)]
struct SimulateParams {
  date: String,
}

#[post("/simulate")]
async fn simulate(simulator: web::Data<Simulator>, params: web::Json<SimulateParams>) -> impl Responder {
  let start = Instant::now();
  tracing::info!("📡 Requête reçue avec date = {}", params.date);
  tracing::info!("🔭 Nombre de planètes : {}", simulator.celestItems.len());

  let target_date = match DateTime::parse_from_rfc3339(&params.date) {
    Ok(parsed) => parsed.with_timezone(&Utc),
    Err(e) => {
      return HttpResponse::BadRequest().body(format!("Date invalide : {e}"));
    }
  };

  let result = simulator.load_or_compute(target_date).await;
  let nb_items = result.len();
  
  /// convert result to JSON
  let result = match serde_json::to_string(&result) {
    Ok(json) => json,
    Err(e) => {
      return HttpResponse::InternalServerError().body(format!("Erreur de sérialisation : {e}"));
    }
  };
  
  tracing::info!("✅ Simulation terminée avec {} objets celestes in {} ms", nb_items, start.elapsed().as_millis());
  HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Chargement des variables d'environnement
  dotenvy::from_filename(".env.local").ok();
  let port = env::var("RUST_SERVER_PORT").ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(8080);
  let address = env::var("RUST_SERVER_ADDRESS").ok().and_then(|s| s.parse::<String>().ok()).unwrap_or_else(|| "localhost".to_string());
  let allowerd_origins = env::var("RUST_ALLOWED_ORIGINS").ok().and_then(|s| s.parse::<String>().ok()).unwrap_or_else(|| "http://localhost:3000".to_string());
  let log_level = env::var("RUST_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
  let log_output = env::var("RUST_LOG_OUTPUT").unwrap_or_else(|_| "stdout".to_string());

  // Initialisation du logger
  LoggerFactory::init_from_env(log_level, log_output);

  // Initialisation DAOFactory + connexion Mongo
  let mut daoFactory = DAOFactory::new();
  if let (Ok(uri), Ok(db_name), Ok(collection_name)) = (env::var("MONGO_URI"), env::var("MONGO_DB_NAME"), env::var("MONGO_COLLECTION_NAME")) {
    daoFactory.connect(&uri, &db_name, &collection_name).await;
  }
  let daoFactory = Arc::new(daoFactory);

  const PLANETS_PATH: &str = "data/celest_items.json";
  let simulator = web::Data::new(Simulator::new(daoFactory, PLANETS_PATH));  

  println!("🚀 Serveur lancé sur http://{}:{}", address, port);
  HttpServer::new(move || {
    App::new()
      .app_data(simulator.clone())
      .wrap(
        Cors::default()
            .allowed_origin(&allowerd_origins)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials(),
      )
      .service(home)
      .service(simulate)
  })
  .bind((address, port))?
  .run()
  .await
}
