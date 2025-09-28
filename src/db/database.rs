
use sqlx::{Pool, Postgres};
use crate::config::Config;

pub async fn init_db() -> anyhow::Result<Pool<Postgres>> {
    let config = Config::from_env();
    let pool = Pool::<Postgres>::connect(&config.database_url).await?;
    Ok(pool)
}

