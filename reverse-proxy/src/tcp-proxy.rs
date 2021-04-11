extern crate config;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use futures::future::join_all;

use getopts::Options;
use std::env;

use tokio::join;
use tokio::net::TcpListener;
use tokio::net::TcpSocket;
use tokio::net::TcpStream;
use tokio::task;

use std::collections::HashMap;

use std::io;

type HashAddressInfo = HashMap<String, HashMap<String, String>>;
type HashContainsYaml = HashMap<String, HashAddressInfo>;

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
        error!("Stream output eError transfering bytes");
    }

    Ok(())
}

pub struct Host {
    host: String,
    port: i32,
}

pub struct RuleConfig {
    name: String,
    input: Host,
    output: Host,
}

fn load_config(config_file: &str) -> Vec<RuleConfig> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(config_file))
        .unwrap();
    let mut doc = settings.try_into::<HashContainsYaml>().unwrap();

    let mut rules: Vec<RuleConfig> = Vec::new();
    for (name, value) in doc.iter_mut() {
        let address_input = value.clone().get_mut("input").map_or(
            Host {
                host: "".to_owned(),
                port: 0,
            },
            |v| Host {
                host: v["host"].clone(),
                port: v["port"].clone().parse::<i32>().unwrap(),
            },
        );
        let address_output = value.clone().get_mut("output").map_or(
            Host {
                host: "".to_owned(),
                port: 0,
            },
            |v| Host {
                host: v["host"].clone(),
                port: v["port"].clone().parse::<i32>().unwrap(),
            },
        );

        rules.push(RuleConfig {
            name: name.to_string(),
            input: address_input,
            output: address_output,
        });
    }

    return rules;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[tokio::main]
async fn main() -> io::Result<()> {
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
        return;
    }

    let config_file = matches.opt_str("f").unwrap();
    let rule_configs = load_config(config_file.as_str());

    pretty_env_logger::init();

    let mut joins = Vec::new();
    for rule in rule_configs {
        let input_host = rule.input.host;
        let input_port = rule.input.port;
        let output_host = rule.output.host;
        let output_port = rule.output.port;

        let join = task::spawn(async move {
            let output_address = format!("{}:{}", output_host, output_port);
            let input_address = format!("{}:{}", input_host, input_port);
            let listener = TcpListener::bind(output_address.clone())
                .await
                .expect(format!("Error binding address {}", output_address).as_str());
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
