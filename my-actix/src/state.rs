use std::sync::Mutex;
use crate::models::Course;

pub struct AppState {
    pub visit_count: Mutex<u32>,
    pub health_check_response: String,
    pub courses: Mutex<Vec<Course>>
}