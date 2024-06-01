use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str, settings: &crate::settings::Settings) -> Self {
        match PgPoolOptions::new()
            .max_connections(settings.storage.pool_max_open)
            .min_connections(settings.storage.pool_max_idle)
            .idle_timeout(std::time::Duration::from_secs(
                settings.storage.pool_timeout_seconds,
            ))
            .connect(db_url)
            .await
        {
            Ok(pool) => Store { connection: pool },
            Err(e) => {
                panic!("Couldn't establish DB connection! Error: {}", e)
            }
        }
    }
}
