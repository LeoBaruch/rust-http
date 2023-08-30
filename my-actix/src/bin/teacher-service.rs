use std::{
  io::Result, 
  sync::Mutex
};
use actix_web::{web, App, HttpServer};

#[path = "../state.rs"]
mod state;
#[path = "../router.rs"]
mod router;
#[path = "../handlers.rs"]
mod handlers;

use state::AppState;
use router::{ general_routes};

#[actix_web::main] 
async fn main() -> Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}