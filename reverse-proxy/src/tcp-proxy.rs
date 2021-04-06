use futures::future::join_all;
use getopts::Options;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::join;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;

type BoxedError = Box<dyn std::error::Error + Sync + Send + 'static>;
static DEBUG: AtomicBool = AtomicBool::new(false);

async fn forward(
    bind_ip: &str,
    local_port: i32,
    remote_host: &str,
    remote_port: i32,
) -> Result<(), BoxedError> {
    // Listen on the specified IP and port
    println!("Calling forward port");
    let bind_addr = if bind_ip.contains(':') {
        format!("[{}]:{}", bind_ip, local_port)
    } else {
        format!("{}:{}", bind_ip, local_port)
    };
    println!("Binding address {}", bind_addr);
    let bind_sock = bind_addr
        .parse::<std::net::SocketAddr>()
        .expect("Failed to parse bind address");
    println!("Trying to binding...");
    let listener = TcpListener::bind(&bind_sock).await?;
    println!("Listening on {}", listener.local_addr().unwrap());

    // We have either been provided an IP address or a host name.
    // Instead of trying to check its format, just trying creating a SocketAddr from it.
    let parse_result = format!("{}:{}", remote_host, remote_port).parse::<std::net::SocketAddr>();
    let remote_addr = match parse_result {
        Ok(s) => s,
        Err(_) => {
            // It's a hostname; we're going to need to resolve it.
            // Create an async dns resolver

            use trust_dns_resolver::config::*;
            use trust_dns_resolver::TokioAsyncResolver;

            let resolver =
                TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
                    .await
                    .expect("Failed to create DNS resolver");

            let resolutions = resolver
                .lookup_ip(remote_host)
                .await
                .expect("Failed to resolve server IP address!");
            let remote_addr = resolutions
                .iter()
                .nth(1)
                .expect("Failed to resolve server IP address!");
            println!("Resolved {} to {}", remote_host, remote_addr);
            format!("{}:{}", remote_addr, remote_port).parse()?
        }
    };

    loop {
        let (mut client, client_addr) = listener.accept().await?;

        tokio::spawn(async move {
            println!("New connection from {}", client_addr);

            // Establish connection to upstream for each incoming client connection
            let mut remote = TcpStream::connect(&remote_addr).await?;
            let (mut client_recv, mut client_send) = client.split();
            let (mut remote_recv, mut remote_send) = remote.split();

            let (remote_bytes_copied, client_bytes_copied) = join!(
                tokio::io::copy(&mut remote_recv, &mut client_send),
                tokio::io::copy(&mut client_recv, &mut remote_send),
            );

            match remote_bytes_copied {
                Ok(count) => {
                    if DEBUG.load(Ordering::Relaxed) {
                        eprintln!(
                            "Transferred {} bytes from remote client {} to upstream server",
                            count, client_addr
                        );
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Error writing from remote client {} to upstream server!",
                        client_addr
                    );
                    eprintln!("{:?}", err);
                }
            };

            match client_bytes_copied {
                Ok(count) => {
                    if DEBUG.load(Ordering::Relaxed) {
                        eprintln!(
                            "Transferred {} bytes from upstream server to remote client {}",
                            count, client_addr
                        );
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Error writing bytes from upstream server to remote client {}",
                        client_addr
                    );
                    eprintln!("{:?}", err);
                }
            };

            let r: Result<(), BoxedError> = Ok(());
            r
        });
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "o",
        "",
        "set remote host with port for the forwarding type: REMOTE_HOST:NUMBER_PORT",
        "REMOTE_HOST_WITH_PORT",
    );
    opts.optopt(
        "i",
        "",
        "set local host with  port for the forwarding type: LOCAL_HOST:NUMBER_PORT",
        "LOCAL_HOST_WITH_PORT",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if !matches.opt_present("i") || !matches.opt_present("o") {
        print_usage(&program, opts);
        return;
    }
    let str_output = matches.opt_str("o").expect("Error getting output");
    let output: Vec<&str> = str_output.split(":").collect();
    let str_input = matches.opt_str("i").expect("Error getting input");
    let input: Vec<&str> = str_input.split(":").collect();

    if input.len() != 2 && output.len() != 2 {
        print_usage(&program, opts);
        return;
    }
    if input[1].parse::<i32>().is_err() || output[1].parse::<i32>().is_err() {
        print_usage(&program, opts);
        return;
    }
    let input_port = input[1].parse::<i32>().unwrap();
    let output_port = output[1].parse::<i32>().unwrap();
    let input_host = input[0];
    let output_host = output[0];
    //let mut futures: Vec<dyn Future<Output = Result<(), BoxedError>>> = Vec::new();
    //futures.push(forward(input_host, input_port, output_host, output_port));
    let futures = vec![forward(input_host, input_port, output_host, output_port)];
    let rt = Runtime::new().unwrap();
    rt.block_on(async { join_all(futures).await });
}
