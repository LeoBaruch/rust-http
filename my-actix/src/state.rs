use std::sync::Mutex;

pub struct AppState {
    pub visit_count: Mutex<u32>,
    pub health_check_response: String,
}