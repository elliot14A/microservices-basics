#![allow(dead_code)]

use axum::async_trait;
use sqlx::{pool::PoolOptions, Pool};

use crate::error::ApiErrResp;

type Result<T> = std::result::Result<T, ApiErrResp>;

#[async_trait]
pub trait RawAuthContext: Send + Sync + 'static {
    async fn register() {}
    async fn authenticate() {}
    async fn authorize() {}
}

pub struct AuthContext<DB: sqlx::Database> {
    pool: Pool<DB>,
}

impl<DB: sqlx::Database> AuthContext<DB> {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool: Pool<DB> = PoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl<DB: sqlx::Database> RawAuthContext for AuthContext<DB> {
    async fn authenticate() {}
    async fn authorize() {}
    async fn register() {}
}