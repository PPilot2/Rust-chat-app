# WebSocket Chat Application

A simple WebSocket-based chat application built with Rust using the `warp`, `tokio`, and `futures` libraries. This app allows clients to connect via WebSockets, broadcast messages to all connected clients, and serve static files.

## Features

- **WebSocket chat server**: Clients can send and receive messages in real-time.
- **Broadcast**: Messages sent by any client are broadcasted to all other connected clients.
- **Static file serving**: Static assets (e.g., HTML, CSS, JS) can be served via the `static` directory.
  
## Dependencies

- [warp](https://docs.rs/warp/) - Web framework for building the WebSocket server.
- [tokio](https://tokio.rs/) - Asynchronous runtime for Rust.
- [futures](https://docs.rs/futures/) - Async utilities for working with streams and sinks.

## Prerequisites

Make sure you have the following installed:
- Rust (via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/PPilot2/chat_app.git
   cd chat_app
