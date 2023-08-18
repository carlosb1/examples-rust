use clap::Parser;
use rand::Rng;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
//use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::task;

#[derive(clap::ValueEnum, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Server,
    Client,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    mode: Mode,
}

struct Report {
    total_numbers: u64,
}

type SharedReport = Arc<Mutex<Report>>;
type Stop = Arc<Mutex<bool>>;

async fn bg_report_task(_report: &SharedReport, stop: &Stop) {
    let stop_1 = stop.clone();
    let join = task::spawn(async move {
        println!("starting background task");
        while *stop_1.lock().unwrap() {
            thread::sleep(time::Duration::from_secs(10));
            println!("Checking reports...");
        }
    });
    let _ = join.await;
}

pub struct TCPServer {
    host: String,
    port: u32,
    max_clients: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.mode == Mode::Server {
        // set up shared domain modules
        let report: SharedReport = Arc::new(Mutex::new(Report { total_numbers: 0 }));
        let stop: Stop = Arc::new(Mutex::new(false));

        //TCPServer
        let tcp_server = TCPServer::new("127.0.0.1".to_string(), 8080, 5);

        let rt = Runtime::new()?;
        rt.block_on(async {
            bg_report_task(&report, &stop).await;
            let _ = tcp_server.run(&report, &stop).await;
        });
    } else if args.mode == Mode::Client {
        println!("I am a client");
    } else {
        println!("It was not specified any mode");
    }
    Ok(())
}
