pub mod user;

pub fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
    user::router::register_routes(cfg); // callig the register_routes function in the user module. 
}