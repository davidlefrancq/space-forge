use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use chrono::{DateTime, Utc};

mod bo;
mod bll;
mod dal;

use bll::simulation::Simulation;

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
  let target_date = match DateTime::parse_from_rfc3339(&params.date) {
    Ok(parsed) => parsed.with_timezone(&Utc),
    Err(e) => {
      return HttpResponse::BadRequest().body(format!("Date invalide : {e}"));
    }
  };

  let planets_path = "data/planets.json";
  let planets = Simulation::load_planets(planets_path);
  let result = Simulation::run(&planets, target_date);

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
