use actix_web::web;
use crate::handlers::health_check;


pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}