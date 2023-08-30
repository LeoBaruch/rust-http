use actix_web::{web, HttpResponse};
use crate::state::AppState;

pub async fn health_check(
  state: web::Data<AppState>
) -> HttpResponse {
  let health_check_response = state.health_check_response.clone();
  let mut count = state.visit_count.lock().unwrap();
  *count += 1;

  let response = format!("{}: {}", health_check_response, count);

  HttpResponse::Ok()
    .body(response)
}