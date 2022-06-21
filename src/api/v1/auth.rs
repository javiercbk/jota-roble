use crate::errors::*;
use crate::AppData;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserAuthentication {
    pub id: i64,
    first_name: String,
    last_name: String,
    #[serde(skip_serializing)]
    password: String,
    #[serde(skip_serializing)]
    salt: String,
}

pub mod runners {
    use super::*;
    use sha2::{Digest, Sha512};

    #[derive(Clone, Debug, Deserialize)]
    pub struct Login {
        pub email: String,
        pub password: String,
    }

    fn hash_password(password: &str, salt: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(password);
        hasher.update(salt);
        return hasher.finalize().to_vec();
    }

    pub async fn login_runner(
        payload: Login,
        app_data: AppData,
    ) -> ServiceResult<UserAuthentication> {
        let user_fut = sqlx::query_as!(
            UserAuthentication,
            r#"
    SELECT id, first_name, last_name, password, salt
    FROM users
    WHERE email = ?
            "#,
            payload.email,
        )
        .fetch_one(&app_data.db)
        .await;

        match user_fut {
            Ok(user_authentication) => {
                let password_hash = hash_password(&payload.password, &user_authentication.salt);
                let hash_str = hex::encode(password_hash);
                log::info!(
                    "password_hash = {}\npassword_usr  = {}",
                    hash_str,
                    user_authentication.password
                );
                if user_authentication.password == hash_str {
                    return Ok(user_authentication);
                }
                return Err(ServiceError::AccountNotFound);
            }
            Err(_) => Err(ServiceError::InternalServerError),
        }
    }
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

#[post("/api/v1/signin")]
async fn login(
    // id: Identity,
    payload: web::Json<runners::Login>,
    app_data: AppData,
) -> ServiceResult<impl Responder> {
    let user_authentication = runners::login_runner(payload.into_inner(), app_data).await?;
    // id.remember(userAuthentication.id.to_string());
    Ok(HttpResponse::Ok().json(json!({ "userAuthentication": user_authentication })))
}
