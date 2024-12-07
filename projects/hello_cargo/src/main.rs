/*fn main() {
    println!("Hello, world 2!");
}*/

#[derive(Debug)]
struct Rectangle {
    height1: u32,
    width1: u32,
}

impl Rectangle {
    fn area2(&self) -> u32 {
        self.height1*self.width1
    }

    fn wide_rect(&self) -> bool {
        self.width1 > 100
    }

    fn width1(&self) -> u32 {
        self.width1
    }
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

    println!("Area2 of rectangle is {}", rect.area2());

    println!("Width1 of rectangle is {}", rect.width1);

    if rect.wide_rect() {
        println!("It's a wider rectangle of width {}", rect.width1);
    }
    else {
        println!("It's not a wider rectangle of width {}", rect.width1());
    }

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width1*rectangle.height1
}