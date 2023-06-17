use crate::args::ARGS;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use num_traits::cast::ToPrimitive;
use sqlx::types::BigDecimal;
use sqlx::{query, Pool, Postgres};

lazy_static! {
    pub static ref POOL: AsyncOnce<Pool<Postgres>> = AsyncOnce::new(async {
        sqlx::PgPool::connect(ARGS.auth_db.as_ref().unwrap())
            .await
            .expect("Could not connect to the auth posgres database, make sure its valid")
    });
}

pub async fn create_passwords() -> anyhow::Result<()> {
    query!(
        r#"
CREATE TABLE IF NOT EXISTS passwords
(
    id          BIGSERIAL PRIMARY KEY,
    username    TEXT    NOT NULL,
    password    NUMERIC NOT NULL
);
           "#
    )
    .execute(POOL.get().await)
    .await?;

    Ok(())
}

pub async fn get_password(username: String) -> Option<u64> {
    query!(
        r#"SELECT * FROM passwords WHERE username = $1"#,
        username.to_lowercase()
    )
    .fetch_one(POOL.get().await)
    .await
    .ok()?
    .password
    .to_u64()
}

pub async fn set_password(username: String, password: u64) -> anyhow::Result<()> {
    query!(
        r#"INSERT INTO passwords (username, password) VALUES ( $1 , $2 ) RETURNING id"#,
        username.to_lowercase(),
        BigDecimal::from(password)
    )
    .fetch_one(POOL.get().await)
    .await?;

    Ok(())
}
