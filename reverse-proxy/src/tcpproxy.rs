use futures::future::join_all;
use std::io;
use tokio::join;
use tokio::net::TcpListener;
use tokio::net::TcpSocket;
use tokio::net::TcpStream;
use tokio::task;

use crate::config::RuleConfig;

pub struct Proxy {}

impl Proxy {
    /// Proxy Layer 3 implementation. It fowards tcp/ip packets from an origin service (address and
    /// port) and it is forwarded in a local port.
    ///
    /// Constructor initializes an proxy instance.
    pub fn new() -> Proxy {
        Proxy {}
    }
    /// Async. function to apply a forwarding port among services.
    ///
    /// # Arguments
    ///
    /// * `stream_origin` - Origin tcpstream (source tcp service that it will be forwarded)
    /// * `output_address` - Output address where it will be forwarded
    ///
    pub async fn forward(mut stream_origin: TcpStream, output_address: &str) -> io::Result<()> {
        let socket_output = TcpSocket::new_v4()?;
        let mut stream_output = socket_output
            .connect(output_address.parse().expect("Unable parse socket address"))
            .await?;
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
            error!("Stream output error transfering bytes");
        }
        Ok(())
    }

    /// Main async function that it throws each thread for any address to forward
    ///
    /// # Arguments
    ///
    /// * `rule_configs`: List of rules that speficy what will be forwarded.
    pub async fn run(&self, rule_configs: &Vec<RuleConfig>) {
        let mut joins = Vec::new();
        for rule in rule_configs {
            let input_host = rule.input.host.clone();
            let input_port = rule.input.port;
            let output_host = rule.output.host.clone();
            let output_port = rule.output.port;
            let rule_name = rule.name.clone();

            let join = task::spawn(async move {
                let output_address = format!("{}:{}", output_host, output_port);
                let input_address = format!("{}:{}", input_host, input_port);
                info!(
                    "SERVICE {} - input address {} to output address {}",
                    rule_name, input_address, output_address
                );
                let listener = TcpListener::bind(output_address.clone())
                    .await
                    .expect(format!("Error binding address {}", output_address.clone()).as_str());
                //TODO loop to wait for connection, is it needed this loop?
                loop {
                    let (socket, _) = listener.accept().await.unwrap();
                    Proxy::forward(socket, input_address.as_str())
                        .await
                        .unwrap();
                }
            });
            joins.push(join);
        }
        join_all(joins).await;
    }
}
