// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

type Users = Arc<Mutex<HashMap<SocketAddr, String>>>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum MessageType {
    Users,
    Register,
    Message,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MessageType,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    from: String,
    message: String,
    time: u128,
}

fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn users_message(users: Vec<String>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&WebSocketMessage {
        message_type: MessageType::Users,
        data_array: Some(users),
        data: None,
    })
}

fn chat_message(from: String, message: String) -> Result<String, serde_json::Error> {
    let data = serde_json::to_string(&ChatMessage {
        from,
        message,
        time: now_millis(),
    })?;

    serde_json::to_string(&WebSocketMessage {
        message_type: MessageType::Message,
        data_array: None,
        data: Some(data),
    })
}

async fn current_users_message(users: &Users) -> Result<String, serde_json::Error> {
    let users = users.lock().await;
    users_message(users.values().cloned().collect())
}

async fn handle_text(
    addr: SocketAddr,
    text: &str,
    bcast_tx: &Sender<String>,
    users: &Users,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Ok(message) = serde_json::from_str::<WebSocketMessage>(text) {
        match message.message_type {
            MessageType::Register => {
                let username = message.data.unwrap_or_else(|| addr.to_string());
                {
                    let mut users = users.lock().await;
                    users.insert(addr, username);
                }
                bcast_tx.send(current_users_message(users).await?)?;
            }
            MessageType::Message => {
                let sender = {
                    let users = users.lock().await;
                    users
                        .get(&addr)
                        .cloned()
                        .unwrap_or_else(|| addr.to_string())
                };
                let text = message.data.unwrap_or_default();
                bcast_tx.send(chat_message(sender, text)?)?;
            }
            MessageType::Users => {}
        }
    } else {
        let message = format!("{addr}: {text}");
        println!("{message}");
        bcast_tx.send(message)?;
    }

    Ok(())
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Users,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(msg) => {
                        let msg = msg?;
                        if let Some(text) = msg.as_text() {
                            handle_text(addr, text, &bcast_tx, &users).await?;
                        }
                    }
                    None => break,
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }

    if users.lock().await.remove(&addr).is_some() {
        bcast_tx.send(current_users_message(&users).await?)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx, users).await
        });
    }
}
