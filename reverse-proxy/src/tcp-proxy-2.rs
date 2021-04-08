use tokio::net::TcpListener;

use std::io;

async fn process_socket<T>(socket: T, port: &str) {
    TcpStream::connect();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let output_port: &str = "";
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket, output_port).await;
    }
}
