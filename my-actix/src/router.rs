use crate::handlers::{get_course_details, get_course_with_id, health_check, new_course};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::post().to(new_course))
            .route("/{id}", web::get().to(get_course_with_id))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_course_details),
            ),
    );
}
