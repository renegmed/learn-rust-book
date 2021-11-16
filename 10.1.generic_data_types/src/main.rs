/*

 this error states that the body of largest won’t work for all 
 possible types that T could be. Because we want to compare values 
 of type T in the body, we can only use types whose values can be 
 ordered.

*/

// fn largest<T>(list: &[T]) -> T {

//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             // ERROR: binary operation `>` cannot be applied to type `T`
//             largest = item;
//         }
//     }

//     largest
// }
// The FIX


/*
    ERROR:


|     for &item in list {
         -----    ^^^^
         ||
         |data moved here
         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
    FIX: <T: PartialOrd + Copy>
 
 */

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {             
            largest = item;
        }
    }
    largest
}

/*
   Another way we could implement largest is for the function to return a reference 
   to a T value in the slice. If we change the return type to &T instead of T, thereby 
   changing the body of the function to return a reference, we wouldn’t need the Clone 
   or Copy trait bounds and we could avoid heap allocations. 
   
   Try implementing these alternate solutions on your own! If you get stuck with errors 
   having to do with lifetimes, keep reading: the “Validating References with Lifetimes” 
   section coming up will explain, but lifetimes aren't required to solve these challenges.

*/

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > &largest {             
//             largest = item;
//         }
//     }
//     largest
// }


// -------------------------------------------
struct Point<T> {
    x: T, 
    y: T,
}

// --- generic of different types 
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn getX(&self) -> &T {
        &self.x
    }

    fn getY(&self) -> &U {
        &self.y
    }
}

// --- using concrete instead of generic

struct Point3<f32> {
    x: f32, 
    y: f32,
}
impl Point3<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

  // ----- generic on enums 
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// -- A method that uses different generic types from its struct’s definition
struct Point4<T, U> {
    x: T,
    y: U,
}

// returns mixed type
impl<T, U> Point4<T,U> {
    fn mixup<V,W>(self, other: Point4<V, W>) -> Point4<T,W> {
        Point4 {
            x: self.x,
            y: other.y
        }
    }
}
fn main() {
    let number_list = vec![34, 58, 25, 100, 65,120];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    println!("integer {}, {}", integer.x, integer.y);

    let float = Point { x: 1.0, y: 4.0 };
    println!("float {}, {}", float.x, float.y);

    //let wont_work = Point { x: 5, y: 4.0 };


    let p = Point2 { x: 5, y: 10.5 };
    println!("Point of diff types {}, {}", p.x, p.y);
    println!("Impl point {}, {}", p.getX(), p.getY());

    let p3 = Point3{ x: 3.0, y: 4.0};
    let d = p3.distance_from_origin();
    println!("Distance from origin {}", d);

    // --- mixed points 
    let p41 = Point4 { x: 5, y: 10.4 };
    let p42 = Point4 { x: "Hello", y: 'c'};

    let p43 = p41.mixup(p42);

    println!("mixed points p3.x = {}, p3.y ={}", p43.x, p43.y);

  

}
