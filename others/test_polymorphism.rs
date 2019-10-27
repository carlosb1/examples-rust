use std::rc::Rc;


struct Pig {
    name: &'static str
}

struct Dog {
    name: &'static str
}

trait Animal {
    fn sound(&self);
}

impl Animal for Pig {
    fn sound(&self) {
        println!("this is a {} pig", self.name);
    }
}

impl Animal for Dog {
    fn sound(&self) {
        println!("this is a {} dog", self.name);
    }
}


struct Farm {
    animal: Rc<dyn Animal>
}

struct Family {
   animal: Rc<dyn Animal> 
}


fn main () {
    let pig = Pig{name: "pig1"};
    pig.sound();
    let dog = Dog{name: "dog1"};
    dog.sound();
    let farm = Farm{animal: Rc::new(pig)};
    let farm2 = Farm{animal: Rc::new(pig)};


    // hello example


}
