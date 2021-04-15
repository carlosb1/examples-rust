extern crate pretty_env_logger;
#[macro_use]
extern crate log;
pub mod config;
pub mod tcpproxy;

use crate::config::load_config;
use crate::tcpproxy::Proxy;
use getopts::Options;
use std::env;

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
    let rule_config = load_config(config_file.as_str());
    let proxy = Proxy::new();
    proxy.run(&rule_config).await;
    Ok(())
}
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}
