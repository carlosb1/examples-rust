extern crate collatz_multiplexed as collatz;

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;

pub fn main () {
    let addr = "127.0.0.1:9999".parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    core.run(
            collatz::Client::connect(&addr, &handle).and_then(
                |client| {
                    client.call("110".to_string())
                        .and_then( move |response| {
                                println!("We got back: {:?}", response);
                                Ok(())
                            })
                })
        ).unwrap();
}

