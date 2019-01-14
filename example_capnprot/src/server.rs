use capnp_rpc::{RpcSystem, twoparty, rpc_twoparty_capnp};
use hello_capnp::{hello};

use gj::{EventLoop, Promise, TaskReaper, TaskSet};

struct HelloImpl;

impl hello::Server for HelloImpl {
    fn hello( &mut self,
              params: hello::HelloParaams,
              mut results: hello::HelloResults,
        )->Promise<(), ::capnp::Error>
    {
        println!("received a request for greetings!");
        let greeting: &str = "Hello ";
        let name: &str = pry!(pry!(params.get()).get_name());
        let response = format!("{}{}!", greeting, name);

        results.get().set_response(&response);
        Promise::ok(())
    }
}
pub fn accept_loop(
        listener: ::gjio::SocketListener,
        mut task_set: TaskSet<(), ::capnp::Error>,
        helloc: hello::Client,
    ) -> Promise<(), ::std::io::Error> {
    listener.accept().then(move |stream| {
        let mut network = twoparty::VatNetwork::new(
                stream.clone(),
                stream,
                rpc_twoparty_capnp::Side::Server,
                Default::default(),
            );
            let disconnect_promise = network.on_disconnect();
            let rpc_system  = RpcSystem::new(Box::new(network), Some(helloc.clone().client));
            task_set.add(disconnect_promise.attach(rpc_system));
            accept_loop(listener, task_set, helloc);
    })
}
struct Reaper;
impl TaskReaper<(), ::capnp::Error> for Reaper {
    fn task_failed(&mut self, error: ::capnp::Error) {
        println!("Task failed {}", error);
    }
}
