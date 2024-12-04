/*fn main() {
    println!("Hello, world 2!");
}*/

struct Rectangle {
    height1: u32,
    width1: u32,
}

fn main() {
    let rect = Rectangle {
        height1: 30,
        width1: 50,
    };

    println!( "{}", 
        area(&rect)
    );

    println!("h: {0}, w: {1}", rect.height1, rect.width1);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width1*rectangle.height1
}