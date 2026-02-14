use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

use crate::AppState;

type Client = UnboundedSender<Message>;
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

pub fn broadcast(clients: &Clients, message: &str) {
    for (_, client) in clients.lock().unwrap().iter() {
        let _ = client.send(Message::Text(message.to_string().into()));
    }
}

pub fn broadcast_filter(
    clients: &Clients,
    filter: impl Fn(&String, &Client) -> bool,
    message: &str,
) {
    for (user_id, client) in clients.lock().unwrap().iter() {
        if filter(user_id, client) {
            let _ = client.send(Message::Text(message.to_string().into()));
        }
    }
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    let user_id = Uuid::new_v4().to_string();

    ws.on_upgrade(move |socket| handle_socket(socket, user_id, state.clients))
}

async fn handle_socket(socket: WebSocket, user_id: String, clients: Clients) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    clients.lock().unwrap().insert(user_id.clone(), tx);

    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // TODO: handle pings
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    clients.lock().unwrap().remove(&user_id);
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(ws_handler))
}
