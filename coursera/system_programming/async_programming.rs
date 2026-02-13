// popular lib
// tokio - for high performance app with complex concurrency
//  async-std  - for simple app
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            // handle the connection
        });
    }
}
