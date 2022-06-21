use crate::errors::*;
use crate::AppData;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};


fn hash_password(password: &str, salt: &str) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(password);
    hasher.update(salt);
    return hasher.finalize().to_vec();
}


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

#[derive(Clone, Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn perform_login(payload: LoginRequest, app_data: AppData) -> ServiceResult<UserAuthentication> {
    log::info!("email => '{}', password => '{}'", payload.email, payload.password);
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
            return Err(ServiceError::NotFound);
        },
        Err(sqlx::Error::RowNotFound) => Err(ServiceError::Forbidden),
        Err(err) => {
            log::error!("error performing login, {}", err);
            Err(ServiceError::InternalServerError)
        },
    }
}


