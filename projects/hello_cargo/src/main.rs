#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct Rectangle {
    height1: u32,
    width1: u32,
}

impl Rectangle {

    fn square(size :u32) -> Self {
        Self {
            height1: size,
            width1: size,
        }
    }

    fn area2(&self) -> u32 {
        self.height1*self.width1
    }

    fn wide_rect(&self) -> bool {
        self.width1 > 100
    }

    fn width1(&self) -> u32 {
        self.width1
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height1 > other.height1 && self.width1 > other.width1
    }
}

fn route(ip:IPAddrKind) {
    println!("{:#?}",ip);
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        height1: dbg!(30*scale),
        width1: 50,
    };

    let rect2 = Rectangle {
        width1: 10,
        height1: 40,
    };
    let rect3 = Rectangle {
        width1: 60,
        height1: 45,
    };

    println!( "{}", 
        area(&rect)
    );

    println!("rect1 is {rect:?}");
    
    let sq = Rectangle::square(3);     
    println!("Printing a square height: {} and width: {}", sq.height1, sq.width1);

    dbg!(&rect);

    println!("Area2 of rectangle is {}", rect.area2());

    println!("Width1 of rectangle is {}", rect.width1);

    if rect.wide_rect() {
        println!("It's a wider rectangle of width {}", rect.width1);
    }
    else {
        println!("It's not a wider rectangle of width {}", rect.width1());
    }

    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));


    route(IPAddrKind::V4);
    route(IPAddrKind::V6);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width1*rectangle.height1
}