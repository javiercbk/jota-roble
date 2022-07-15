use crate::errors::{ServiceResult};
use crate::AppData;
use actix_web::{get, post, web, HttpResponse, Responder};
use super::api::{Show, Api};

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(create_show_controller);
    cfg.service(get_show_controller);
}

#[post("/api/v1/shows")]
async fn create_show_controller(
    // id: Identity,
    payload: web::Json<Show>,
    app_data: AppData,
) -> ServiceResult<impl Responder> {
    let mut show = payload.into_inner();
    let shows_api = Api::new(app_data);
    shows_api.create_show(&mut show).await?;
    Ok(HttpResponse::Ok().json(show))
}

#[get("/api/v1/shows/{show_id}")]
async fn get_show_controller(
    path: web::Path<i64>,
    app_data: AppData,
) -> ServiceResult<impl Responder> {
    let shows_api = Api::new(app_data);
    let show_id = path.into_inner();
    let show_res = shows_api.get_show(show_id).await?;
    Ok(HttpResponse::Ok().json(show_res))
} 
