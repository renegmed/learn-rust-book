// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         // let result = 2 + 2;
//         assert_eq!(2 + 2, 4);
//     }
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        // self.width < other.width && self.height > other.height  // this a bug
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // String::from("Hello!") + " " + name
    format!("Hello! {}", name)
    //String::from("Hello!")   // fail
}

// --------------------------------------

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {        
        if value < 1 {   
            panic !("Guess value must be greater than or equal to 1, got {}.", value); 
                  
        } else if value > 100 {
            panic !("Guess value must be less than or equal to 100, got {}.", value); 
        } 
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    /*
        user super::*;  

        Because the tests module is an inner module, we need 
        to bring the code under test in the outer module into 
        the scope of the inner module. We use a glob here so 
        anything we define in the outer module is available to 
        this tests module
    */
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannor_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };
        // assert!(!smaller.can_hold(&larger));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }
    /*
        All the primitive types and most of the standard library 
        types implement these traits. For structs and enums that 
        you define, you’ll need to implement PartialEq to assert 
        that values of those types are equal or not equal. 
        
        You’ll need to implement Debug to print the values when 
        the assertion fails. Because both traits are derivable 
        traits.
    */

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", 
            result
        );

    }
    
    // ---------------
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200.")]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")] // this is valid
    fn greater_than_100() {
        Guess::new(200);
    }

    // ----- using Result<T, E> in Tests 
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
        // if 2 + 2 == 5 {  // this is a bug
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to four"))
        }
    }
    /*
        Writing tests so they return a Result<T, E> enables you to use the question mark 
        operator in the body of tests, which can be a convenient way to write tests that 
        should fail if any operation within them returns an Err variant.

        You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
        Instead, you should return an Err value directly when the test should fail.
    */
}