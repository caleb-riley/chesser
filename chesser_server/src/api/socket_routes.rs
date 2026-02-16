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
use chesser_api::{
    network::{NetworkCommand, NetworkMessage},
    transfer::MoveDto,
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

pub fn _broadcast_filter(
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

    ws.on_upgrade(move |socket| handle_socket(state, socket, user_id))
}

async fn handle_socket(state: AppState, socket: WebSocket, user_id: String) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    state.clients.lock().unwrap().insert(user_id.clone(), tx);

    let (sender, mut receiver) = socket.split();

    let sender = Arc::new(tokio::sync::Mutex::new(sender));

    let send_task = {
        let sender = sender.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let mut sender = sender.lock().await;

                if sender
                    .send(axum::extract::ws::Message::Text(
                        msg.to_text().unwrap().to_string().into(),
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        })
    };

    let clients = Arc::clone(&state.clients);

    let recv_task = {
        let sender = sender.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if let axum::extract::ws::Message::Text(text) = msg {
                    match serde_json::from_str::<NetworkCommand>(&text) {
                        Ok(NetworkCommand::SendMove(mv)) => {
                            let mut game = state.game.lock().await;
                            game.board.perform_move(&mv.into());

                            broadcast(
                                &clients,
                                serde_json::to_string(&NetworkMessage::BoardLoaded(
                                    (&game.board).into(),
                                ))
                                .unwrap()
                                .as_str(),
                            );
                        }
                        Ok(NetworkCommand::RequestHints(pos)) => {
                            let game = state.game.lock().await;

                            let piece = game.board.get_piece(pos.into()).unwrap();
                            let hints = game.get_available_moves(&piece.kind, pos.into());
                            let hints_dto = hints.iter().map(MoveDto::from).collect();

                            let mut sender = sender.lock().await;

                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&NetworkMessage::HintsReceived(
                                        hints_dto,
                                    ))
                                    .unwrap()
                                    .into(),
                                ))
                                .await;
                        }
                        Err(e) => {
                            let mut sender = sender.lock().await;

                            let _ = sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&NetworkMessage::ExperiencedError(
                                        format!("{}", e),
                                    ))
                                    .unwrap()
                                    .into(),
                                ))
                                .await;
                        }
                    }
                }
            }
        })
    };

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    state.clients.lock().unwrap().remove(&user_id);
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(ws_handler))
}
