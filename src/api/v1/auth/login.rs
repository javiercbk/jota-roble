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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let hashed_pass = hash_password("password", "salt");
        let hash_str = hex::encode(hashed_pass);
        assert_eq!("fa6a2185b3e0a9a85ef41ffb67ef3c1fb6f74980f8ebf970e4e72e353ed9537d593083c201dfd6e43e1c8a7aac2bc8dbb119c7dfb7d4b8f131111395bd70e97f", hash_str)
    }
}
