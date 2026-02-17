use std::{net::SocketAddr, sync::Arc};

use axum::http::{Method, header::CONTENT_TYPE};
use chesser_core::engine::game::Game;
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    normalize_path::NormalizePathLayer,
};

mod api;

use crate::api::{Clients, app_router};

#[derive(Clone)]
struct AppState {
    game: Arc<Mutex<Game>>,
    clients: Clients,
}

struct HttpServer {
    socket_addr: SocketAddr,
}

impl HttpServer {
    fn new(socket_addr: SocketAddr) -> Self {
        Self { socket_addr }
    }

    async fn start(&self) {
        let clients: Clients = Arc::default();

        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(|origin, _| {
                origin.to_str().is_ok_and(|o| {
                    o.starts_with("http://localhost") || o.starts_with("http://192.168.")
                })
            }))
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
            .allow_headers([CONTENT_TYPE])
            .allow_credentials(true);

        let router = app_router()
            .with_state(AppState {
                game: Arc::new(Mutex::new(Game::default())),
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
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let http_server = HttpServer::new(socket_addr);
    http_server.start().await;

    println!("Server started on {}", socket_addr);
}
