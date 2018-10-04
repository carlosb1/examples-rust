extern crate mosquitto_client as mosq;
use mosq::Mosquitto;
use std::{thread, time};

fn main() {
    let m = Mosquitto::new("test1");
    m.connect("localhost", 1883).expect("can t connect");
    m.subscribe("bilbo/#", 1).expect("can't susbscribe to bonzo");

    let mt = m.clone();
    thread::spawn(move || {
        let timeout = time::Duration::from_millis(500);
        for _ in 0..5 {
            mt.publish("/bilbo/baggins", "hello dolly".as_bytes(), 1, false).unwrap();
            thread::sleep(timeout);
        }
        mt.disconnect().unwrap();
    });
    let mut mc = m.callbacks(0);
    mc.on_message(|data, msg| {
        println!("bilbo {:?}", msg);
        *data +=1
    });
    m.loop_until_disconnect(200).expect("broken loop");
    println!("received {} message", mc.data);

}
