#[derive(Debug)]
struct Rectangle { 
    w: u32,
    h: u32,
}

// we can implement methods that are tied to a struct we created
// structs are allowed to have multiple impl blocks
impl Rectangle {
    
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.w > other.w) && (self.h > other.h)
    }

    // associated function
    // does not have &self as first parameter, not a method
    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}

fn main() {
    
    let rect = Rectangle {
        w: 30,
        h: 50,
    };
    
    let rect2 = Rectangle {
        w: 10,
        h: 40,
    };

    let rect3 = Rectangle {
        w: 60,
        h: 45,
    };
    
    let sq = Rectangle::square(4);

    println!("The area of the rectangle is {}.", area(&rect));
    println!("The area of the rectangle is {}.", rect.area());

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    println!("Square {:?}", sq);

    // {:?} tells println! we want to use an output format called Debug.
    // we have to explicitly opt in to make it available for our struct.
    // we use the outer attribute #[derive(Debug)] before the struct definition.
    // println!("Rectangle {:?}.", rect);
    
    // other way to output the data of a struct
    // println!("Rectangle {:#?}.", rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.w * rectangle.h
}