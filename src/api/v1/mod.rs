use actix_web::web::ServiceConfig;
pub mod auth;
pub mod show;


pub fn services(cfg: &mut ServiceConfig) {
    auth::controller::services(cfg);
    show::controller::services(cfg)
}
