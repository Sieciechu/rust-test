#[macro_use]
extern crate text_io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double_width(&mut self) {
        self.width = 2 * self.width;
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn create_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
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

//    rectangle.double_width();
//    println!("doubled with of rectangle. Now it's {:#?}", rectangle);

    println!("Rect square area is {}", rectangle.area());


    let rect2 = Rectangle { width: 5, height: 5};
    let rect3 = Rectangle { width: 5, height: 50};
    println!("Rectangle can hold rect2: {:?}", rectangle.can_hold(&rect2));
    println!("Rectangle can hold rect3: {:?}", rectangle.can_hold(&rect3));

    println!("Created square: {:?}", Rectangle::create_square(16));
}

fn test1() {
    let rect2: (u32, u32) = (10, 10);
    println!("Rect1 square area is {}", area_from_tuple(rect2));
}