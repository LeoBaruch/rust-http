use actix_web::{web, App, HttpServer};
use std::{io::Result, sync::Mutex};

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../router.rs"]
mod router;
#[path = "../state.rs"]
mod state;

use router::{course_routes, general_routes};
use state::AppState;

#[actix_web::main]
async fn main() -> Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(course_routes)
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
