#[macro_use]
extern crate text_io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
//    test1();
    test2();
}
fn double_width(rectangle: &mut Rectangle) {
    rectangle.width = 2*rectangle.width;
}

fn area_from_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn test2() {
    let mut rectangle = Rectangle { width: 0, height: 0 };
    println!("Please write rectangle width and height (followed by space)");
    scan!("{} {}", rectangle.width, rectangle.height);

    double_width(&mut rectangle);
    println!("doubled with of rectangle. Now it's {:#?}", rectangle);

    println!("Rect square area is {}", area_from_rectangle(&rectangle));
}

fn test1() {
    let rect2: (u32, u32) = (10, 10);
    println!("Rect1 square area is {}", area_from_tuple(rect2));
}