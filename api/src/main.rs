use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use std::fs;

mod bo;
mod bll;
mod dal;

use bll::simulation::Simulation;
use bo::planet::Planet;

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().body("pong")
}

#[derive(Deserialize)]
struct SimulateParams {
  mars_mass: Option<f64>,     // à utiliser plus tard
  duration_days: u32
}

#[post("/simulate")]
async fn simulate(params: web::Json<SimulateParams>) -> impl Responder {
  let planets_path = "data/planets.json";

  let mut planets = Simulation::load_planets(planets_path);

  // TODO : appliquer la masse personnalisée de Mars si fournie
  if let Some(mass) = params.mars_mass {
    if let Some(mars) = planets.iter_mut().find(|p| p.name == "Mars") {
      mars.mass = mass;
    }
  }

  let result: Vec<Planet> = Simulation::run(&planets, params.duration_days);

  HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("🚀 Serveur lancé sur http://localhost:8080");

  HttpServer::new(|| {
    App::new()
      .service(ping)
      .service(simulate)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
