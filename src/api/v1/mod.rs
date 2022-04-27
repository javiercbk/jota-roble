use actix_web::web::ServiceConfig;
pub mod auth;


pub fn services(cfg: &mut ServiceConfig) {
    auth::services(cfg);
}
