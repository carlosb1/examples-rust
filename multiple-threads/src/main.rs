use clap::Parser;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time};
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

async fn run_threads(report: &SharedReport, stop: &Stop) {
    // Copy structure to manage conc.//
    let report_1 = report.clone();
    let stop_1 = stop.clone();
    let join = task::spawn(async move {
        for x in 0..10 {
            if *stop_1.lock().unwrap() == true {
                break;
            }
            let mut rep = report_1.lock().unwrap();
            rep.total_numbers += 1;
            println!("First thread working {}", x);
            thread::sleep(time::Duration::from_secs(
                rand::thread_rng().gen_range(0..2),
            ));
        }
        *stop_1.lock().unwrap() = true;
    });

    // Copy structure to manage conc.//
    let report_2 = report.clone();
    let stop_2 = stop.clone();
    let join2 = task::spawn(async move {
        for x in 0..10 {
            if *stop_2.lock().unwrap() == true {
                break;
            }
            println!("Second thread working {}", x);
            thread::sleep(time::Duration::from_secs(
                rand::thread_rng().gen_range(0..2),
            ));
            let rep = report_2.lock().unwrap();
            println!("my repo info {:?}", rep.total_numbers);
        }
        *stop_2.lock().unwrap() = true;
    });

    let _ = join.await;
    let _ = join2.await;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.mode == Mode::Server {
        // set up shared domain modules
        let report: SharedReport = Arc::new(Mutex::new(Report { total_numbers: 0 }));
        let stop: Stop = Arc::new(Mutex::new(false));

        println!("I am server");
        let rt = Runtime::new()?;
        rt.block_on(async {
            bg_report_task(&report, &stop).await;
            run_threads(&report, &stop).await;
        });
    } else if args.mode == Mode::Client {
        println!("I am a client");
    } else {
        println!("It was not specified any mode");
    }
    Ok(())
}
