use actix::{Actor, Addr, Context, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am alive");
        System::current().stop();
    }
}

fn main() {
    let system = System::new("test");
    let addr = MyActor.start();
    system.run();
}
