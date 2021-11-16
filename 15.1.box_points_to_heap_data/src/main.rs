use List::{Cons, Nil};

// enum List {
// // ^^^^^^^^^ recursive type has infinite size    
//     Cons(i32, List),
//     Nil,
// }

enum List { 
    Cons(i32, Box<List>),  // Box<List> - pointer to List
    Nil,
}
fn main() {
    // --- Using a Box<T> to Store data on the heap 
    let b = Box::new(5);
    println!("b = {}", b);
    // Box is allocated in the heap with pointer b resides in the memory stack 
   
    // --- cons list - Boxes enable recursive types 
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))) )));
}
