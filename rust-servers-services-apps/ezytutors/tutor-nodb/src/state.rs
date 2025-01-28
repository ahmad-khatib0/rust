use std::sync::Mutex;

use super::models::Course;

pub struct AppState {
    pub health_check_response: String, // shared immutable state
    pub visit_count: Mutex<u32>,       // shared mutable state
    pub courses: Mutex<Vec<Course>>,
}
