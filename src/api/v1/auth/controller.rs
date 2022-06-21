use crate::errors::{ServiceResult};
use crate::AppData;
use actix_web::{post, web, HttpResponse, Responder};

use super::login::{LoginRequest, perform_login};

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(login_controller);
}

#[post("/api/v1/login")]
async fn login_controller(
    // id: Identity,
    payload: web::Json<LoginRequest>,
    app_data: AppData,
) -> ServiceResult<impl Responder> {
    let user_authentication = perform_login(payload.into_inner(), app_data).await?;
    Ok(HttpResponse::Ok().json(user_authentication))
} 
