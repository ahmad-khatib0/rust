use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

use routes::*;
use state::AppState;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[path = "../state.rs"]
mod state;

#[path = "../models.rs"]
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // initialize the application state
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    // Define the web application.
    let app = move || {
        App::new()
            // Register the application state with the web application.
            .app_data(shared_data.clone())
            // Configure routes for the web application.
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
