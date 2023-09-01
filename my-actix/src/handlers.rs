use actix_web::{web, HttpResponse};
use crate::state::AppState;

pub async fn health_check(
  state: web::Data<AppState>
) -> HttpResponse {
  let health_check_response = state.health_check_response.clone();
  let mut count = state.visit_count.lock().unwrap();
  *count += 1;

  let response = format!("{}: {}\r\n", health_check_response, count);

  HttpResponse::Ok()
    .body(response)
}

use crate::models::Course;
use chrono::Utc;
pub async fn new_course(
  state: web::Data<AppState>,
  new_course: web::Json<Course>
) -> HttpResponse {
  let mut courses = state.courses.lock().unwrap();
  let course_count  = courses.iter().filter(|c| c.teacher_id == new_course.teacher_id).count() as u32;
  let course = Course {
    teacher_id: new_course.teacher_id,
    id: Some(course_count + 1),
    name: new_course.name.clone(),
    time: Some(Utc::now().naive_utc()),
  };
  courses.push(course);

  println!("Course added: {:?}", courses);

  HttpResponse::Ok().body("Course added.")
}