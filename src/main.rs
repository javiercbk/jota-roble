extern crate log;

use std::io;
use std::sync::Arc;

use actix_web::{middleware as actix_middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;
mod data;
mod errors;
mod routes;

pub use crate::data::Data;
pub use crate::routes::services;

pub type AppData = actix_web::web::Data<Arc<crate::data::Data>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").expect("Host not set");
    let port_str = env::var("PORT").expect("Port not set");
    let port = port_str.parse::<u16>().expect("port is not valid");
    log::info!("starting HTTP server at http://{}:{}", host, port_str);

    let data = Data::new().await;
    sqlx::migrate!("./migrations/").run(&data.db).await.unwrap();
    let data = actix_web::web::Data::new(data);

    // start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .add(("Permissions-Policy", "interest-cohort=()")),
            )
            .wrap(actix_middleware::Compress::default())
            .app_data(data.clone())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::TrailingSlash::Trim,
            ))
            .configure(routes::services)
    })
    .bind((host, port))?
    .run()
    .await
}
