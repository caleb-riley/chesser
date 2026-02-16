use bevy::prelude::*;
use chesser_api::{
    network::{NetworkCommand, NetworkMessage},
    transfer::{BoardDto, PieceConfigDto},
};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;

use crate::Interface;

#[derive(Resource)]
pub struct NetworkClient {
    pub incoming_rx: UnboundedReceiver<NetworkMessage>,
    pub outgoing_tx: UnboundedSender<NetworkCommand>,
}

async fn fetch_board() -> Result<BoardDto, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3000/game/board")
        .send()
        .await?;

    let response = response.error_for_status()?;

    let board = response.json::<BoardDto>().await?;

    Ok(board)
}

async fn fetch_piece_configs() -> Result<HashMap<String, PieceConfigDto>, reqwest::Error> {
    let client = reqwest::Client::new();

    let piece_configs = client
        .get("http://127.0.0.1:3000/game/piece-configs")
        .send()
        .await?
        .error_for_status()?
        .json::<HashMap<_, _>>()
        .await?;

    println!("{}", piece_configs.len());

    Ok(piece_configs)
}

pub fn handle_network_messages(mut interface: ResMut<Interface>, mut net: ResMut<NetworkClient>) {
    while let Ok(msg) = net.incoming_rx.try_recv() {
        match msg {
            NetworkMessage::BoardLoaded(dto) => {
                interface.board = Some(dto);
                println!("Board loaded");
            }
            NetworkMessage::PieceConfigsLoaded(dto) => {
                interface.pieces = dto;
                println!("Piece configs loaded");
            }
            NetworkMessage::SocketMessage(message) => {
                println!("Socket said: {message}");
            }
            NetworkMessage::HintsReceived(moves) => {
                interface.hints = moves.into_iter().map(|m| (m.destination, m)).collect();
            }
            NetworkMessage::MovePerformed => {
                println!("someone performed a move");
            }
            NetworkMessage::ExperiencedError(err) => {
                eprintln!("Uh oh: {err}");
            }
        }
    }
}

pub async fn network_task(
    incoming_tx: UnboundedSender<NetworkMessage>,
    mut outgoing_rx: UnboundedReceiver<NetworkCommand>,
) {
    if let Ok(board) = fetch_board().await {
        incoming_tx.send(NetworkMessage::BoardLoaded(board)).ok();
    } else {
        eprintln!("Failed to fetch board over HTTP");
    }

    if let Ok(configs) = fetch_piece_configs().await {
        incoming_tx
            .send(NetworkMessage::PieceConfigsLoaded(configs))
            .ok();
    } else {
        eprintln!("Failed to fetch piece configs over HTTP");
    }

    let (ws_stream, _) = tokio_tungstenite::connect_async("ws://127.0.0.1:3000/ws")
        .await
        .expect("Failed to connect to websocket");

    let (mut ws_write, mut ws_read) = ws_stream.split();

    loop {
        tokio::select! {
            Some(cmd) = outgoing_rx.recv() => {
                match cmd {
                    NetworkCommand::RequestHints(pos) => {
                        let text = serde_json::to_string(&NetworkCommand::RequestHints(pos)).unwrap();

                        ws_write.send(Message::Text(text.into())).await.ok();
                    }
                    NetworkCommand::SendMove(mv) => {
                        let text = serde_json::to_string(&NetworkCommand::SendMove(mv)).unwrap();

                        ws_write.send(Message::Text(text.into())).await.ok();
                    }
                }
            }

            Some(msg) = ws_read.next() => {
                match msg {
                    Ok(msg) => {
                        if let Ok(text) = msg.to_text() {
                            match serde_json::from_str::<NetworkMessage>(text) {
                                Ok(network_msg) => {
                                    incoming_tx.send(network_msg).ok();
                                }
                                Err(e) => {
                                    eprintln!("Failed to deserialize NetworkMessage: {}", e);

                                    incoming_tx
                                        .send(NetworkMessage::SocketMessage(text.to_string()))
                                        .ok();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("WebSocket read error: {}", e);
                        break;
                    }
                }
            }
        }
    }
}

pub fn start_networking(mut commands: Commands, runtime: Res<TokioRuntime>) {
    let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel();
    let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::unbounded_channel();

    commands.insert_resource(NetworkClient {
        incoming_rx,
        outgoing_tx: outgoing_tx.clone(),
    });

    runtime.0.spawn(async move {
        network_task(incoming_tx, outgoing_rx).await;
    });
}

#[derive(Resource)]
pub struct TokioRuntime(pub tokio::runtime::Runtime);

impl Default for TokioRuntime {
    fn default() -> Self {
        Self(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
    }
}
