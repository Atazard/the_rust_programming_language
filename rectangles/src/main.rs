#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        //why would you name the function the same as the field?
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn create_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn print_area(&self) {
        println!("The area is: {}", self.calculate_area())
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // println!("The area of the rectangle is {}", calculate_area(width1, height1));

    // let rect1 = (30, 50);
    // println!("The area of the rectangle is {}", calculate_area2(rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("The area of the rectangle is {}", calculate_area3(&rect1));

    println!("\nrect1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    println!("The area of the rectangle is {}", rect2.calculate_area());
    if rect2.width() {
        //rust knows that if it has () then it's the method and not the field
        println!("The rectangle has a width value higher than 0");
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::create_square(5);
    sq.print_area();
}

// fn calculate_area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn calculate_area2(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn calculate_area3(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
