extern crate pencil;
use std::collections::BTreeMap;
use pencil::{Pencil, Request, Response, PencilResult};

fn hello (_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello world!"))
}

fn hello_template(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    context.insert("name".to_string(), "template".to_string());
    return request.app.render_template("hello.html", &context);
}

fn main() {
    let mut app = Pencil::new("/web/hello");
    app.register_template("hello.html");
    app.get("/hello_template", "hello_template", hello_template)
    //app.get("/", "hello", hello);
    //app.run("127.0.0.1:5555");
}
