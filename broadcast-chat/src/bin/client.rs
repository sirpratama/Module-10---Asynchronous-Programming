// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use serde::Deserialize;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MessageType {
    Users,
    Register,
    Message,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MessageType,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    from: String,
    message: String,
}

fn display_message(raw: &str) -> String {
    let Ok(message) = serde_json::from_str::<WebSocketMessage>(raw) else {
        return raw.to_string();
    };

    match message.message_type {
        MessageType::Users => format!(
            "users: {}",
            message.data_array.unwrap_or_default().join(", ")
        ),
        MessageType::Message => message
            .data
            .and_then(|data| serde_json::from_str::<ChatMessage>(&data).ok())
            .map(|data| format!("{}: {}", data.from, data.message))
            .unwrap_or_else(|| raw.to_string()),
        MessageType::Register => raw.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line? {
                    Some(text) => ws_stream.send(Message::text(text)).await?,
                    None => break,
                }
            }
            Some(msg) = ws_stream.next() => {
                let msg = msg?;
                if let Some(text) = msg.as_text() {
                    println!("{}", display_message(text));
                }
            }
            else => break,
        }
    }

    Ok(())
}
