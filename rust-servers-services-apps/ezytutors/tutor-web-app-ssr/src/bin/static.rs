use std::env;

use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    // println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    println!("Listening on: {}, open browser and visit have a try!", addr);
    HttpServer::new(|| {
        // Register the actix_files service with the web application.
        // show_files_listing() allows subdirectory listings to be shown to users.
        App::new().service(fs::Files::new("/static", "./static").show_files_listing())
        // the route /static indicates that resource requests starting with the /static route
        // have to be served from the ./static subfolder in the project root folder.
    })
    .bind(addr)?
    .run()
    .await
}
