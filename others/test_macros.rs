
macro_rules! yo {
    ($name: expr) => {
        print!("hey {}", $name);
    };
}

fn main () {
    println!("hello world");
    yo!("carlos");
}


