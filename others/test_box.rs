use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}


#[allow(dead_code)]
struct Rectangle{
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point {x: 0.0, y: 0.0})
}

fn main () {

    //Stack allocated variable
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point {x: 3.0, y: 4.0}
    };
    //Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::New(Rectangle {
        p1: origin(),
        p2: origin()
    });

    let boxed_point = Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupied {} bytes in the stack",
             mem::size_of_val(&point));

    println!("Rectangle occupied {} bytes in the stack",
             mem::size_of_val(&rectangle));


    println!("Boxed point occupied {} bytes in the stack",
             mem::size_of_val(&boxed_point));

    println!("Boxed rectangle occupied {} bytes in the stack",
             mem::size_of_val(&boxed_rectangle));

    println!("Boxed box occupied {} bytes in the stack",
             mem::size_of_val(&box_in_a_box));
    
    let unboxed_point: Point = *boxed_point;

    println!("Unboixed point occupie {} bytes in the stack", mem::size_of_val(&unboxed_point));
}
