extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};

fn hello (_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello world!"))
}

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.get("/", "hello", hello);
    app.run("127.0.0.1:5555");
}
