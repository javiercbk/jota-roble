use crate::errors::*;
use crate::AppData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, sqlx::FromRow, Deserialize, Serialize, PartialEq)]
pub struct Show {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
}

pub struct Api {
    app_data: AppData,
}

impl Api {
    pub fn new(app_data: AppData) -> Self {
        Self { app_data: app_data }
    }

    pub async fn create_show(&self, payload: &mut Show) -> Result<(), ServiceError> {
        let insert_result = sqlx::query!(r#"
        INSERT INTO shows (name) VALUES (?)
                "#,
            payload.name).execute(&self.app_data.db).await;
        match insert_result {
            Ok(result) => {
                payload.id = result.last_insert_rowid();
                return Ok(())
            },
            Err(sqlx::Error::Database(ref err)) if err.message() == "UNIQUE constraint failed: shows.name" => {
                Err(ServiceError::Conflict)
            }
            Err(err) => {
                log::error!("error inserting show, {}", err);
                Err(ServiceError::InternalServerError)
            }
        }
    }

    pub async fn get_show(&self, id: i64) -> Result<Show, ServiceError> {
        let get_result = sqlx::query_as!(Show, r#"SELECT * FROM shows WHERE id = ?"#, id).fetch_one(&self.app_data.db).await;
        match get_result {
            Err(err) => {
                log::error!("error retrieving show, {}", err);
                Err(ServiceError::InternalServerError)
            }
            Ok(show) => Ok(show),
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::Data;
    use crate::db::load_schema;
    use super::*;

    #[actix_web::test]
    async fn test_create_show() {
        let data = Data::new(":memory:").await;
        let app_data = actix_web::web::Data::new(data);
        let res = load_schema(&app_data.db).await;
        assert!(!res.is_err(), "schema should have been loaded");
        let mut show = Show { id: 0, name: String::from("a test show") };
        let shows_api = Api::new(app_data);
        let show_res = shows_api.create_show(&mut show).await;
        assert_eq!(show_res, Ok(()));
        let show_get_res = shows_api.get_show(show.id).await;
        assert!(!show_get_res.is_err(), "show should have been found");
        assert_eq!(show_get_res.unwrap(), show);
    }
}
