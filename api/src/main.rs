use std::env;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::Deserialize;
use chrono::{DateTime, Utc};

mod bo;
mod bll;
mod dal;
mod utils;

use bll::simulation::Simulation;
use utils::logger_factory::LoggerFactory;

const PLANETS_PATH: &str = "data/planets.json";

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().body("pong")
}

#[derive(Deserialize)]
struct SimulateParams {
  date: String,
}

#[post("/simulate")]
async fn simulate(params: web::Json<SimulateParams>) -> impl Responder {
  tracing::info!("📡 Requête reçue avec date = {}", params.date);

  let target_date = match DateTime::parse_from_rfc3339(&params.date) {
    Ok(parsed) => parsed.with_timezone(&Utc),
    Err(e) => {
      return HttpResponse::BadRequest().body(format!("Date invalide : {e}"));
    }
  };

  let planets = Simulation::load_planets(PLANETS_PATH);
  tracing::info!("🪐 {} planètes chargées", planets.len());

  let result = Simulation::run(&planets, target_date);
  tracing::info!("✅ Simulation terminée");

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
  
  // println!("🚀 Serveur lancé sur http://localhost:8080");
  println!("🚀 Serveur lancé sur http://{}:{}", address, port);

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
            .allowed_origin(&allowerd_origins)  // <-- ton origine Next.js
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials(),
      )
      .service(ping)
      .service(simulate)
  })
  .bind((address, port))?
  .run()
  .await
}
