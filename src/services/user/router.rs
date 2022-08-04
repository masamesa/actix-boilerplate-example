use actix_web::{web, get, HttpResponse};
use crate::services::user::models;

// This is the router for the user service. 
// Here we handle the inital registration of the routes in this file by creating a new "register_routes" function.
pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service({
            web::scope("/api") // Scope is the path that the routes will be auto registered to; E.g. <url>/api/v1/user/getuser)
            .service(get_user)
        });
}   


#[get("v1/user/getuser")] // We declare it in this way so when it comes time to upgrade to v2 we can easily change the route per endpoint.
async fn get_user(body: web::Json<models::UserRequest>) -> HttpResponse { // There's multiple web types and return types available. See the docs for more info.
    HttpResponse::Ok().body(format!("Hello user {}!", body.user_id))
}