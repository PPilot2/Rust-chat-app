use tokio::sync::broadcast;
use warp::{Filter, ws::{self, WebSocket}};
use futures::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    // Create a channel for broadcasting messages
    let (tx, _rx) = broadcast::channel::<String>(100);

    // Serve static files
    let static_files = warp::fs::dir("static");

    // WebSocket handler
    let chat = warp::path("chat")
        .and(warp::ws())
        .map(move |ws: ws::Ws| {
            let tx = tx.clone();
            ws.on_upgrade(move |socket| handle_socket(socket, tx))
        });

    // Start the server with both static file serving and WebSocket
    let routes = static_files.or(chat);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_socket(socket: WebSocket, tx: broadcast::Sender<String>) {
    let (mut tx_socket, mut rx_socket) = socket.split();

    // Broadcast received messages to all clients
    tokio::spawn(async move {
        while let Some(Ok(msg)) = rx_socket.next().await {
            if let ws::Message::Text(text) = msg {
                if let Err(_) = tx.send(text) {
                    break;
                }
            }
        }
    });

    // Send broadcast messages to the connected client
    let mut rx = tx.subscribe();
    while let Ok(msg) = rx.recv().await {
        if let Err(_) = tx_socket.send(ws::Message::text(msg)).await {
            break;
        }
    }
}
