use super::state::AppState;
use actix_web::{web, HttpResponse};

// is the type of the custom application

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    // Application state registered with the Actix web application is made available
    // to all handler functions as an extractor object of type web::Data<T>,
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let respone = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&respone)
}
