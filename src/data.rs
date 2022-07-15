use std::{sync::Arc, str::FromStr, time::Duration};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
// use actix_identity::{CookieIdentityPolicy, IdentityService};

/// App data
pub struct Data {
    /// database pool
    pub db: sqlx::SqlitePool,
    // pub identityService: IdentityService<CookieIdentityPolicy>,
}

impl Data {

    pub async fn new(database_url: &str) -> Arc<Data> {
        // connect to SQLite DB

        let sqlite_connection_options = SqliteConnectOptions::from_str(database_url).expect("error creating database")
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::new(5, 0))
            .foreign_keys(true);
        let db = sqlx::SqlitePool::connect_with(sqlite_connection_options).await.expect("failed to create sqlite pool");
        // let policy = CookieIdentityPolicy::new(&[0; 32])
        //     .name("auth-cookie")
        //     .secure(false);
        // let identityService = IdentityService::new(policy);
        let data = Data {
            db,
            // identityService,
        };

        Arc::new(data)
    }
}