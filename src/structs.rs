struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/*Associated functions -> associated with the type name not with the instance,
They are often used for constructors that will return a new instance of the struct */

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let square1 = Rectangle::square(30);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold square1 ? {}", rect1.can_hold(&square1));
}