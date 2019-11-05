use std::rc::Rc;

#[derive(Copy, Clone)]
struct Pig {
    name: &'static str
}

#[derive(Copy, Clone)]
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
    animal: Rc<dyn Animal>,
}
impl Farm {
    fn hello_farm(&self) {
        self.animal.sound();
    }
}


struct Family {
   animal: Rc<dyn Animal> 
}


fn main () {
    let pig = Pig{name: "pig1"};
    pig.sound();
    let dog = Dog{name: "dog1"};
    dog.sound();
    let ref_pig = Rc::new(pig);
    let farm = Farm{animal: ref_pig.clone()};
    let farm2 = Farm{animal: ref_pig.clone()};
    let farm3 = Farm{animal: Rc::new(dog)};

    farm.hello_farm();
    farm2.hello_farm();
    farm3.hello_farm();


    // hello example


}
