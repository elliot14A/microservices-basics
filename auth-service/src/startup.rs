use std::sync::Arc;

use log::info;

use crate::{context, server};

pub struct Application {
    httpaddr: std::net::SocketAddr,
    httpserver: server::HttpServer,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL NOT SET");
        let auth_ctx = Arc::new(context::AuthContext::<sqlx::Postgres>::new(&db_url).await?);
        let (httpserver, httpaddr) = server::build_http_server(auth_ctx).await?;
        Ok(Self {
            httpaddr,
            httpserver,
        })
    }

    pub async fn run_untill_stopped(self) -> anyhow::Result<()> {
        info!("🚀 Listening on {} for HTTP traffic...", self.httpaddr);
        Ok(self.httpserver.await?)
    }
}
