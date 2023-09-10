// use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub teacher_id: u32,
    pub id: Option<u32>,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}
