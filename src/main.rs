use actix_web::{App, HttpServer}; // Crate imports
use actix_web::middleware::Logger;
use env_logger::Env;

mod services; // Local import module for the services module.

// Main function containing factory for creating the REST server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(); // Setting up logging to console.
    dotenv::dotenv().ok(); // Loading .env file if it exists.

    // Initilizing with the env variables. If they don't exist, application panics.
    let server_host: String = dotenv::var("SERVER_IP").unwrap(); 
    let server_port: String = dotenv::var("SERVER_PORT").unwrap();

    println!("Starting server on {}:{}", server_host, &server_port); // Logging to console.

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(services::register_routes) // Register the routes in the services module
    })
    .bind(server_host + ":" + &server_port)?
    .run()
    .await
}
