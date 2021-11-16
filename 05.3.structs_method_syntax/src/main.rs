#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }
    /*
    Associated function 

    Another useful feature of impl blocks is that we’re allowed to define 
    functions within impl blocks that don’t take self as a parameter. 
    
    These are called associated functions because they’re associated with 
    the struct. They’re still functions, not methods, because they don’t have 
    an instance of the struct to work with. You’ve already used the String::from 
    associated function.

    Associated functions are often used for constructors that will return a 
    new instance of the struct. 
    */

    fn square(s: u32) -> Rectangle {
        Rectangle {
            width: s, 
            height: s,
        }
    }

    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
        }
    }
}

fn main() {
    //let rect1 = Rectangle{width: 30, height: 50};
    let rect1 = Rectangle::new(30, 50);
    
    //println!("Rectangle {:#?}", rectangle);
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle { width: 40, height: 40 };
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(40);
    println!(
        "The area of the rectangle 4 is {} square pixels",
        rect4.area()
    );
}
