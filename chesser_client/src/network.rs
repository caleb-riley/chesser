use bevy::prelude::*;
use futures_util::{SinkExt, StreamExt};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio_tungstenite::connect_async;

#[derive(Resource)]
pub struct NetworkClient {
    pub _sender: mpsc::Sender<String>,
    pub receiver: Arc<Mutex<mpsc::Receiver<String>>>,
}

pub fn start_network(mut commands: Commands) {
    let (to_ws_tx, to_ws_rx) = mpsc::channel::<String>();
    let (from_ws_tx, from_ws_rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async move {
            let (ws_stream, _) = connect_async("ws://127.0.0.1:3000/connect")
                .await
                .expect("Failed to connect");

            let (mut write, mut read) = ws_stream.split();

            let writer = tokio::spawn(async move {
                while let Ok(msg) = to_ws_rx.recv() {
                    write.send(msg.into()).await.ok();
                }
            });

            let reader = tokio::spawn(async move {
                while let Some(Ok(msg)) = read.next().await {
                    if let Ok(text) = msg.to_text() {
                        from_ws_tx.send(text.to_string()).ok();
                    }
                }
            });

            tokio::join!(writer, reader);
        });
    });

    commands.insert_resource(NetworkClient {
        _sender: to_ws_tx,
        receiver: Arc::new(Mutex::new(from_ws_rx)),
    });
}
