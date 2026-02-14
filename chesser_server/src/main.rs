use std::{net::SocketAddr, sync::Arc, thread};

use axum::http::{Method, header::CONTENT_TYPE};
use chesser_core::game::Game;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    normalize_path::NormalizePathLayer,
};
use uuid::Uuid;

mod api;
mod transfer;

use crate::api::{Clients, app_router, broadcast};

#[derive(Clone)]
struct AppState {
    _pool: Arc<SqlitePool>,
    game: Arc<Game>,
    clients: Clients,
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
        let clients: Clients = Arc::default();
        let clients2 = Arc::clone(&clients);

        thread::spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                broadcast(&clients2, &Uuid::new_v4().to_string());
            }
        });

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

        let router = app_router()
            .with_state(AppState {
                _pool: Arc::new(pool),
                game: Arc::new(Game::default()),
                clients,
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
    let database_url = "sqlite://./database/data.db";

    let http_server = HttpServer::new(socket_addr, database_url);
    http_server.start().await;

    println!("Server started on {}", socket_addr);
}
