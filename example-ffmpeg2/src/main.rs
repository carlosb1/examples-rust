extern crate ffmpeg_next as ffmpeg;
extern crate pretty_env_logger;

use std::env;
use std::process::Command;
use std::thread;
use std::time;

use futures::{FutureExt, StreamExt};
use http::Uri;
use warp::Filter;

use getopts::Options;
//ffmpeg -i rtsp://192.xxx.xxx:5554 -y c:a aac -b:a 160000 -ac 2 s 854x480 -c:v libx264 -b:v 800000 -hls_time 10 -hls_list_size 10 -start_number 1 playlist.m3u8

fn run(input_address: Uri, output_file: &str) -> Option<u32> {
    Command::new("ffmpeg")
        .args(&[
            "-i",
            format!("{}", input_address).as_str(),
            "-y",
            "c:a",
            "aac",
            "-b:a",
            "160000",
            "-ac",
            "2",
            "s",
            "854x480",
            "-c:v",
            "libx264",
            "-b:v",
            "800000",
            "-hls_time",
            "10",
            "-hls_list_size",
            "10",
            "-start_number",
            "1",
            output_file,
        ])
        .spawn()
        .ok()
        .map(|child| child.id())
}
fn run_http(input_address: Uri, output_address: Uri) -> Option<u32> {
    Command::new("ffmpeg")
        .args(&[
            "-i",
            format!("{}", input_address).as_str(),
            "-f",
            "mpeg1video",
            "-b",
            "800k",
            "-r",
            "30",
            format!("{}", output_address).as_str(),
        ])
        .spawn()
        .ok()
        .map(|child| child.id())
}
fn run_http_server() {
    println!("Trying to run http server");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        pretty_env_logger::init();

        let dispatch_websocket = |ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    } else {
                        let info = result.unwrap();
                        println!("{:?}", info)
                    }
                })
            })
        };

        let videos_endpoint = warp::path("videos").and(warp::ws()).map(dispatch_websocket);
        warp::serve(videos_endpoint)
            .run(([127, 0, 0, 1], 3030))
            .await;
    })
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optopt("i", "", "set input file name", "NAME");
    opts.optflag("h", "help", "print this help menu");

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

    if !matches.opt_str("o").is_some() || !matches.opt_str("i").is_some() {
        print_usage(&program, opts);
        return;
    }

    let input = matches.opt_str("i").expect("Expected input file");
    let output = matches.opt_str("o").expect("Expected output file");

    let pos_uri_input = input.parse::<Uri>();
    let pos_uri_output = output.parse::<Uri>();

    let uri_input = match pos_uri_input {
        Ok(v) => v,
        Err(_) => {
            print_usage(&program, opts);
            return;
        }
    };
    let uri_output = match pos_uri_output {
        Ok(v) => v,
        Err(_) => {
            print_usage(&program, opts);
            return;
        }
    };

    thread::spawn(move || run_http_server());
    thread::sleep(time::Duration::from_secs(5));
    if let Some(return_id) = run_http(uri_input, uri_output) {
        println!("Type of return value {}", return_id);
    } else {
        println!("Error t get id")
    }
    thread::sleep(time::Duration::from_secs(20));
}
