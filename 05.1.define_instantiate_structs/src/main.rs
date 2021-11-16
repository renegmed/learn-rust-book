/*
You can also define structs that look similar to tuples, called tuple structs. 

Tuple structs have the added meaning the struct name provides but don’t have 
names associated with their fields; rather, they just have the types of the fields. 

Tuple structs are useful when you want to give the whole tuple a name and make the 
tuple be a different type than other tuples, and naming each field as in a regular 
struct would be verbose or redundant.

*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// struct User {
//     username: &str, 
//     email: &str, // ^ expected named lifetime parameter
//     sign_in_count: u64, 
//     active: bool,
// }

struct User {
    username: String, 
    email: String, 
    sign_in_count: u64, 
    active: bool,
}

fn main() {

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // let user1 = User{
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //println!("{}", user1);

    /*

    Using struct update syntax to set new email and username values for 
    a User instance but use the rest of the values from the fields of the 
    instance in the user1 variable

    */
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user2 email: {}, username: {}, user1.active: {}, user1.sign_in_count: {}", user2.email, user2.username, user2.active, user2.sign_in_count);

}
