use std::string::String;

struct Ball {
        pub color: String,
    pub ratio: f32
}
impl Ball {
    pub fn new (color: String, ratio: f32) -> Ball{
        return Ball {color, ratio};
    }
}

static mut red_ball : Ball= Ball::new("red".to_string(),1.0);
static mut green_ball : Ball = Ball::new("green".to_string(),3.0);
static mut blue_ball : Ball  = Ball::new("blue".to_string(),-65.0);

/*
struct BallFactory  {}
impl BallFactory {
    pub fn new () -> BallFactory {
        return BallFactory{};
    }
    pub fn create(&self,color: String) -> mut Ball{
        unsafe {
        match color.as_ref() {
            "red" => return red_ball,
            "green" => return green_ball,
            _ => return blue_ball,
        }
        }
    }
}

*/

fn main () {
    let factory = BallFactory::new();
    let red_ball_one = factory.create("red".to_string()); 
    println!("{}",red_ball_one.color);

}
