/*fn main() {
    println!("Hello, world 2!");
}*/

#[derive(Debug)]
struct Rectangle {
    height1: u32,
    width1: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        height1: dbg!(30*scale),
        width1: 50,
    };

    println!( "{}", 
        area(&rect)
    );

    println!("rect1 is {rect:?}");

    dbg!(&rect);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width1*rectangle.height1
}