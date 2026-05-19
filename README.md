# Module 10 - Asynchronous Programming

This repository contains my work for Module 10: Asynchronous Programming.

## Tutorial 1: Timer

### Experiment 1.1: Original timer from the book

The first experiment implements the simple executor, spawner, task, and timer future from the Rust async book executor chapter. I changed the printed signature from the example text to `Rafi's Komputer`.

How to run:

```bash
cargo run -p timer
```

Expected output:

```text
Rafi's Komputer: howdy!
Rafi's Komputer: done!
```

### Experiment 1.2: Understanding how it works.

I added `println!("Rafi's Komputer: hey hey");` right after `spawner.spawn(...)`. The line is outside the async block, so it runs immediately while the async task has only been queued. When the executor starts, it polls the spawned task, prints `howdy!`, reaches `TimerFuture.await`, and returns `Poll::Pending`. After the timer thread sleeps for two seconds and wakes the task, the executor polls it again and the task prints `done!`.

Captured output:

```text
Rafi's Komputer: hey hey
Rafi's Komputer: howdy!
Rafi's Komputer: done!
```

### Experiment 1.3: Multiple Spawn and removing drop

I added three spawned async tasks with different timer durations. `spawn` packages a future as a task and sends it to the executor queue. The `spawner` owns the sending side of that queue, while the `executor` owns the receiving side and polls queued tasks. `drop(spawner)` closes the original sender after all tasks have been submitted, so the executor can stop once every cloned task sender is also gone. If `drop(spawner)` is removed, the executor keeps waiting for more work even after all printed tasks are finished, because the channel is still open.

Captured output:

```text
Rafi's Komputer: hey hey
Task 1: Rafi's Komputer says howdy!
Task 2: Rafi's Komputer says howdy!
Task 3: Rafi's Komputer says howdy!
Task 2: Rafi's Komputer says done!
Task 1: Rafi's Komputer says done!
Task 3: Rafi's Komputer says done!
```

## Tutorial 2: Broadcast Chat

### Experiment 2.1: Original code, and how it run

The broadcast chat follows the Comprehensive Rust two-binary structure. The server accepts websocket connections, receives text messages from any client, and broadcasts each message to all connected clients. The client concurrently reads terminal input and websocket messages with `tokio::select!`.

Run the server:

```bash
cargo run -p broadcast-chat --bin server
```

Run three clients in separate terminals:

```bash
cargo run -p broadcast-chat --bin client
```

When a client types a message, the server receives it through that client's websocket stream, sends it through a Tokio broadcast channel, and every subscribed client receives and prints the text.

### Experiment 2.2: Modifying port

I changed the websocket port to `8080`. The server side is defined in `broadcast-chat/src/bin/server.rs` by `TcpListener::bind("127.0.0.1:8080")`, and the client side is defined in `broadcast-chat/src/bin/client.rs` by `Uri::from_static("ws://127.0.0.1:8080")`. Both sides need to be changed because websocket communication needs one process listening on the same address that the other process connects to.

### Experiment 2.3: Small changes, add IP and Port

I changed the server so it formats each received message as `{addr}: {text}` before sending it through the broadcast channel. The `addr` value comes from `listener.accept()`, so it contains the sender's IP address and temporary client port. This makes the message flow easier to observe because all clients can see which connection produced each message, even though the clients do not have usernames yet.

Example client output:

```text
127.0.0.1:54321: hello from client one
127.0.0.1:54322: hello from client two
```

## Tutorial 3: WebChat using Yew

### Experiment 3.1: Original code

I added the original YewChat client from the tutorial's `websockets-part2` branch into `webchat-yew/`, and the original Node websocket server into `simple-websocket-server/`. The Yew client follows the blog structure: login route, chat route, websocket service, and event bus service. The original server uses Node/TypeScript and listens on port `8080`.

Run the original server:

```bash
cd simple-websocket-server
npm i
npm start
```

Run the original Yew client:

```bash
cd webchat-yew
npm i
npm start
```

### Experiment 3.2: Be Creative!

I customized the Yew web client by changing the login screen into a branded `Rafi's WebChat` entry view, adding a more polished chat header, changing the sidebar into an online-crew panel, and adding an empty-room state for the chat area. I kept the websocket flow unchanged, so the creative change is focused on the client experience instead of the networking logic.
