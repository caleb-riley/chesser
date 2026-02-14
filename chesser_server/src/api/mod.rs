use axum::Router;

use crate::AppState;

mod socket_routes;

pub use socket_routes::{Clients, broadcast, broadcast_filter};

pub fn app_router() -> Router<AppState> {
    Router::new().nest("/connect", socket_routes::router())
}
