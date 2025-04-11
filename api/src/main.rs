use actix_web::{get, post, App, HttpServer, Responder, HttpResponse, web};
use serde::{Deserialize, Serialize};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[derive(Deserialize)]
struct SimulateParams {
    mars_mass: f64,
    duration_days: u32,
}

#[derive(Serialize)]
struct SimulationResult {
    message: String,
    mars_mass: f64,
    simulated_days: u32,
}

#[post("/simulate")]
async fn simulate(params: web::Json<SimulateParams>) -> impl Responder {
    let result = SimulationResult {
        message: "Simulation exécutée (placeholder)".to_string(),
        mars_mass: params.mars_mass,
        simulated_days: params.duration_days,
    };

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
