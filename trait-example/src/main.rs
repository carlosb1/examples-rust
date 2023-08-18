use std::rc::Rc;

trait Feature {
    fn exec(&self);
}

struct MyFeature {}

impl Feature for MyFeature {
    fn exec(&self) {
        println!("My polymorph")
    }
}

struct Example {
    feature: Rc<dyn Feature>,
}

impl Example {
    pub fn hello(&self) {
        self.feature.exec()
    }
}

fn main() {
    println!("Hello, world!");
    let example = Example {
        feature: Rc::new(MyFeature {}),
    };
    example.hello();
}
