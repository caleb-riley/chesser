use axum::Router;

use crate::AppState;

mod game_router;
mod socket_routes;

pub use socket_routes::Clients;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("/ws", socket_routes::router())
        .nest("/game", game_router::game_router())
}
