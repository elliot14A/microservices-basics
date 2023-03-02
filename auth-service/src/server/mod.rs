use std::net::TcpListener;
use std::sync::Arc;

use anyhow::Ok;
use axum::Extension;
use hyper::Method;

use crate::{context::RawAuthContext};
use crate::api;

mod layer;

pub type HttpServer =
    axum::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<axum::Router>>;

pub async fn build_http_server<H: RawAuthContext>(
    ctx: Arc<H>,
) -> anyhow::Result<(HttpServer, std::net::SocketAddr)> {
    let default_port = std::env::var("PORT").unwrap_or_else(|_| 8080.to_string());
    let default_host = "localhost";
    let default_http_addr = [default_host, &default_port].join(":");
    let routes = api::routes::<H>();
    let mut app = routes.layer(Extension(ctx));
    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(tower_http::cors::Any)
        .allow_credentials(false);

    app = app.layer(cors);
    if log::log_enabled!(log::Level::Info) {
        // only add logger layer if level >= INFO
        app = app.layer(layer::HttpLoggerLayer::new());
    }

    let listener = TcpListener::bind(default_http_addr)?;
    let addr = listener.local_addr().expect("failed to get address from the listener");

    let httpserver = axum::Server::from_tcp(listener).unwrap();
    let httpserver = httpserver.serve(app.into_make_service());
    Ok((httpserver, addr))
}
