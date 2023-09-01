use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
  pub teacher_id: u32,
  pub id: Option<u32>,
  pub name: String,
  pub time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
  fn from(course: web::Json<Course>) -> Self {
    Course {
      teacher_id: course.teacher_id,
      id: course.id,
      name: course.name.clone(),
      time: course.time,
    }
  }
}