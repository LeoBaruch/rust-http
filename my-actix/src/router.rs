use actix_web::web;
use crate::handlers::{health_check, new_course, get_course_with_id};


pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/courses")
      .route("", web::post().to(new_course))
      .route("/{id}", web::get().to(get_course_with_id))
  );
}