/*
Notes on Lifetime elision

The compiler uses three rules to figure out what lifetimes references have 
when there aren’t explicit annotations. 

The first rule is that -  each parameter that is a reference gets its own lifetime parameter. 
  
  In other words, a function with one parameter gets one lifetime parameter: 
     fn foo<'a>(x: &'a i32); 
  
  A function with two parameters gets two separate lifetime parameters: 
     fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

The second rule is - if there is exactly one input lifetime parameter, that lifetime is 
  assigned to all output lifetime parameters: 
     fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is - if there are multiple input lifetime parameters, but one of them 
   is &self or &mut self because this is a method, the lifetime of self is assigned 
   to all output lifetime parameters. 
   
   This third rule makes methods much nicer to read and write because fewer symbols 
   are necessary.

*/
use std::fmt::Display;

struct ImportantExcerpt<'a> {  //  This annotation means an instance of ImportantExcerpt 
                               // can’t outlive the reference it holds in its part field.
    part: &'a str,
}

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);
    let string1 = String::from("abcd");  // String
    let string2 = "xyz"; // &str

    let result = longest(string1.as_str(), string2);
    //let result = longest(string2, string2);
    println!("The longest string is {}", result);

    // --- Using the longest function with references to String values 
    //     that have different concrete lifetimes
    
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("+++The longest string is '{}'", result);
    }
    /*
        In this example, string1 is valid until the end of the outer scope, 
        string2 is valid until the end of the inner scope, and result references 
        something that is valid until the end of the inner scope. 
    */

    // --------- Attempting to use result after string2 has gone out of scope
    /*
        example that shows that the lifetime of the reference in result must be 
        the smaller lifetime of the two arguments. We’ll move the declaration of 
        the result variable outside the inner scope but leave the assignment of 
        the value to the result variable inside the scope with string2. 
        
        Then we’ll move the println! that uses result outside the inner scope, after 
        the inner scope has ended.
    */
    let string1 = String::from("long string is long");
    let result;
    // {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        //                                 ^^^^^^^ borrowed value does not live long enough
    // }
    println!("The longest string is {}", result);
    /*
        the error shows that for result to be valid for the println! statement, string2 
        would need to be valid until the end of the outer scope. Rust knows this because 
        we annotated the lifetimes of the function parameters and return values using the 
        same lifetime parameter 'a.
    */

    // ------- lifetime annotations in Struct definitions 
    // A struct that holds a reference, so its definition needs a lifetime 
    // annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find an '.'");
    //{
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("part: {}", i.part);
    //}
    println!("first sentence: {}", first_sentence);
    /*
        function here creates an instance of the ImportantExcerpt struct that holds 
        a reference to the first sentence of the String owned by the variable novel. 
        
        The data in novel exists before the ImportantExcerpt instance is created. In 
        addition, novel doesn’t go out of scope until after the ImportantExcerpt goes 
        out of scope, so the reference in the ImportantExcerpt instance is valid.
    */
}

/*
    this function's return type contains a borrowed value, 
    but the signature does not say whether it is borrowed 
    from `x` or `y`

    The borrow checker can’t determine this either, because 
    it doesn’t know how the lifetimes of x and y relate to 
    the lifetime of the return value. 
    
    To fix this error, we’ll add generic lifetime parameters 
    that define the relationship between the references so 
    the borrow checker can perform its analysis.
*/
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
/*

    The longest function definition specifying that all the references 
    in the signature must have the same lifetime '

    The function signature now tells Rust that for some lifetime 'a, the 
    function takes two parameters, both of which are string slices that live 
    at least as long as lifetime 'a. 
    
    The function signature also tells Rust that the string slice returned 
    from the function will live at least as long as lifetime 'a. 
    
    In practice, it means that the lifetime of the reference returned by the 
    longest function is the same as the smaller of the lifetimes of the 
    references passed in. These constraints are what we want Rust to enforce. 
    
    Remember, when we specify the lifetime parameters in this function signature, 
    we’re not changing the lifetimes of any values passed in or returned. Rather, 
    we’re specifying that the borrow checker should reject any values that don’t 
    adhere to these constraints. 
    
    Note that the longest function doesn’t need to know exactly how long x and y 
    will live, only that some scope can be substituted for 'a that will satisfy 
    this signature.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generic type parameters, trait bounds, and lifetimes together
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where 
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
  //       ^^^^^^^^ 
  //       returns a value referencing data owned by the current function
  // |`result` is borrowed here
}
*/
/*
  Even though we’ve specified a lifetime parameter 'a for the return type, 
  this implementation will fail to compile because the return value lifetime 
  is not related to the lifetime of the parameters at all.

  The problem is that result goes out of scope and gets cleaned up at the end 
  of the longest function. 
  
  We’re also trying to return a reference to result from the function. There is 
  no way we can specify lifetime parameters that would change the dangling reference, 
  and Rust won’t let us create a dangling reference. 
  
  In this case, the best fix would be to return an owned data type rather than a 
  reference so the calling function is then responsible for cleaning up the value.
*/
