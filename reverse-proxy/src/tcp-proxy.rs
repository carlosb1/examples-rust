extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use tokio::join;
use tokio::net::TcpListener;
use tokio::net::TcpSocket;
use tokio::net::TcpStream;

use std::io;

async fn forward(
    mut stream_origin: TcpStream,
    output_address: &str,
    output_port: &str,
) -> io::Result<()> {
    let address = format!("{}:{}", output_address, output_port)
        .parse()
        .unwrap();
    let socket_output = TcpSocket::new_v4()?;
    let mut stream_output = socket_output.connect(address).await?;
    let (mut stream_origin_recv, mut stream_origin_send) = stream_origin.split();
    let (mut stream_output_recv, mut stream_output_send) = stream_output.split();
    let (stream_origin_bytes_copied, stream_output_bytes_copied) = join!(
        tokio::io::copy(&mut stream_origin_recv, &mut stream_output_send),
        tokio::io::copy(&mut stream_output_recv, &mut stream_origin_send),
    );

    if let Ok(count) = stream_origin_bytes_copied {
        info!("Stream origin transfered bytes: {}", count);
    } else {
        error!("Stream origin error transfering bytes");
    }

    if let Ok(count) = stream_output_bytes_copied {
        info!("Stream output transfered bytes: {}", count);
    } else {
        error!("Stream output eError transfering bytes");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();
    let output_port: &str = "8000";
    let output_address: &str = "0.0.0.0";
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        forward(socket, output_address, output_port).await?;
    }
}
