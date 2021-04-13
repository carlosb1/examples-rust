extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod config;

use std::env;
use std::io;

use futures::future::join_all;
use getopts::Options;
use tokio::join;
use tokio::net::TcpListener;
use tokio::net::TcpSocket;
use tokio::net::TcpStream;
use tokio::task;

use crate::config::load_config;
//TODO validate configuration
//TODO validate host before to use

async fn forward(mut stream_origin: TcpStream, output_address: &str) -> io::Result<()> {
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "f",
        "config.yml",
        "specify configuration file for tcp rules",
        "CONFIG_FILE_PROXY",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!(f.to_string())
        }
    };
    let program = args[0].clone();
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    if !matches.opt_present("f") {
        print_usage(&program, opts);
        return Ok(());
    }
    let config_file = matches.opt_str("f").unwrap();
    let rule_configs = load_config(config_file.as_str());

    let mut joins = Vec::new();
    for rule in rule_configs {
        let input_host = rule.input.host;
        let input_port = rule.input.port;
        let output_host = rule.output.host;
        let output_port = rule.output.port;
        let rule_name = rule.name;

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
                forward(socket, input_address.as_str()).await.unwrap();
            }
        });
        joins.push(join);
    }
    join_all(joins).await;
    Ok(())
}
