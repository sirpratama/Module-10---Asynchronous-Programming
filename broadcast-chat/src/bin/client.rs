// Copyright 2024 Google LLC
// SPDX-License-Identifier: Apache-2.0

use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
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
                    println!("{text}");
                }
            }
            else => break,
        }
    }

    Ok(())
}
