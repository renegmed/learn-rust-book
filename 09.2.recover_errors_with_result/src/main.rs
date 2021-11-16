use std::fs::{self,File};
//use std::io::Result;
use std::io::ErrorKind;
use std::io::{self, Read};


fn main() {
    let f: Result<_,_> = File::open("hello.txt");
    println!("{:?}", f);

    // --- Matching on different errors

    let f = match f {
        Ok(file) => file, 
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
           
        }, 
    };

    let e = File::open("my_file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("my_file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
             panic!("Problem opening the file: {:?}", error);
        }  
    });

    // --- Shortcuts for PAnic on Error: unwrap and expect 
    // let f2 = File::open("my_file2.txt").unwrap();
    /*
        If the Result value is the Ok variant, unwrap will return the value inside 
        the Ok. If the Result is the Err variant, unwrap will call the panic! macro 
        for us. 
    */

    // let f3 = File::open("my_file3.txt").expect("Failed to open my_file3.txt");
    /*
        Using expect instead of unwrap and providing good error messages can convey your 
        intent and make tracking down the source of a panic easier.
    */

    // --- propagating errors 
   

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("file.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e), 
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        
        }
    }

    // --- shortcut for propagating errors: the ? operator 

    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut f = File::open("file3.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // --- Chaining method calls after the ? operator

    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("file3.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    // Speaking of different ways to write this function, above code shows that 
    // there’s a way to make this even shorter
    // use std::fs;
    // NOT WORKING
    // fn read_username_from_file4() -> Result<String, io::Error> {
    //     //                           ^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found `()`
    //     fs::read_to_string("file.txt");  
        
    // }

    // --- The ? Operator can be used in functions that return Result
    // NOT WORKING
    // let f = File::open("file.txt")?;
    //                            ^ cannot use the `?` operator in a function that returns `()`
    /*
        This error points out that we’re only allowed to use the ? operator in a function 
        that returns Result or Option or another type that implements std::ops::Try. 
        
        When you’re writing code in a function that doesn’t return one of these types, and 
        you want to use ? when you call other functions that return Result<T, E>, you have 
        two choices to fix this problem. 
        
        One technique is to change the return type of your function to be Result<T, E> if you 
        have no restrictions preventing that. 
        
        The other technique is to use a match or one of the Result<T, E> methods to handle 
        the Result<T, E> in whatever way is appropriate.
    */

    /*

    The main function is special, and there are restrictions on what its 
    return type must be. One valid return type for main is (), and conveniently, 
    another valid return type is Result<T, E>, as shown here:

    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }

    You can read Box<dyn Error> to mean “any kind of error.” Using ? in a main 
    function with this return type is allowed.
  */
}
