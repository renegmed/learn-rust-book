use std::ops::Deref; 

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // ------ Using Box<T> like a reference
    let x = 5;
    let y = Box::new(x);
    // we set y to be an instance of a box pointing to the value 
    // in x rather than a reference pointing to the value of x. 
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // ----- defining our own smart pointer 
    let x = 5;
    //let y = MyBox::new(x);
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);  // behind the scene - *(y.deref())

    /*
        The reason the deref method returns a reference to a value and 
        that the plain dereference outside the parentheses in *(y.deref()) 
        is still necessary is due to the ownership system. 
        
        If the deref method returned the value directly instead of a reference 
        to the value, the value would be moved out of self. We don’t want to 
        take ownership of the inner value inside MyBox<T> in this case and 
        in most cases where we use the dereference operator.
    */

    // ---- impolicit deref coercions with functions and methods
    let m: MyBox<String> = MyBox::new(String::from("Rust"));
    hello(&m);
    /*
        Because we implemented the Deref trait on MyBox<T>, 
        Rust can turn &MyBox<String> into &String by calling deref. 
    */
    let m: &MyBox<String> = MyBox::new(String::from("Rust"));
    //let m = "Rust";    
    hello(&(*m)[..]); // this is the call if MyBox has not implemented the deref method.
}