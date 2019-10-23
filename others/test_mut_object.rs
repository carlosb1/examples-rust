struct Object1 {
    value: String,
    value2: i32,
}

impl Object1 {
    fn run(&self) {
        println!("hello world {}", self.value2); 
    }
    fn internal_change(& mut self, new_number: i32) {
        self.value2 = new_number;
    }
}


fn main () {
    // let object = Object1{value: String::new("carlos"), value2: 1};
    let mut object = Object1{value: String::from("carlos"), value2: 1};
    object.value2 = 2;
    object.run();
    object.internal_change(3);
    object.run();
}
