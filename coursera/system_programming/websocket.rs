// server code
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    while let Ok(stream, _) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            // handle websocket connection
        })
    }
}

// client code
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await.unwrap();
    let (write, read) = ws_stream.split();
    // sending and receiving data
}
