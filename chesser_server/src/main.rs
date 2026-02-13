use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    normalize_path::NormalizePathLayer,
};

#[derive(Clone)]
struct AppState {
    _pool: Arc<SqlitePool>,
}

struct HttpServer {
    socket_addr: SocketAddr,
    database_url: String,
}

impl HttpServer {
    fn new(socket_addr: SocketAddr, database_url: &str) -> Self {
        Self {
            socket_addr,
            database_url: database_url.into(),
        }
    }

    async fn start(&self) {
        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(|origin, _| {
                origin.to_str().is_ok_and(|o| {
                    o.starts_with("http://localhost") || o.starts_with("http://192.168.")
                })
            }))
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
            .allow_headers([CONTENT_TYPE])
            .allow_credentials(true);

        let pool = sqlx::sqlite::SqlitePool::connect(&self.database_url)
            .await
            .unwrap();

        let router = Router::new()
            .with_state(AppState {
                _pool: Arc::new(pool),
            })
            .layer(NormalizePathLayer::trim_trailing_slash())
            .layer(cors);

        let listener = TcpListener::bind(self.socket_addr).await.unwrap();

        axum::serve(listener, router).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let database_url = "sqlite://../database/data.db";

    let http_server = HttpServer::new(socket_addr, database_url);
    http_server.start().await;

    println!("Server started on {}", socket_addr);
}
