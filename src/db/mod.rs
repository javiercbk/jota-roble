use sqlx::sqlite::{SqliteQueryResult};

pub async fn load_schema(db: &sqlx::SqlitePool) -> Result<SqliteQueryResult, sqlx::Error>{
    sqlx::query_file!("./db/schema.sql").execute(db).await
}