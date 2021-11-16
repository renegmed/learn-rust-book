use std::io;

fn main() {
    let s: String = String::from("hello world!");  // convert &str to String
    let st: &str = &s[..]; //"Hello, world!";  // slice allows us to use the same function on both String values and &str values

    loop {
        println!("Please choose:");
        println!("1=arg &String  2=arg &str  3=exit:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line"); 

        let num: u32 = match choice.trim().parse() { // extract num from io::Result<usize>
            Ok(num) => num, 
            Err(_) => continue,
        };
       
        match num {
            1 => { 
                let word = first_word(&s);  // borrow string &String as argument
                println!("{:?}", word);
            },
            2 => {                 
                let word = first_word2(st); // string literal, &str as argument
                println!("{:?}", word);
            },
            3 => break,
            _ => {
                println!("Invalid choice. Retry"); 
            }        
        }         
    };  // loop
}
 

fn first_word(s: &String) -> &str {  // return slice of a string
    let bstr = s.as_bytes();
    for (i, &c) in bstr.iter().enumerate() {
        if c == b' ' {
            return &s[..i]
        } 
    } 

    return &s[..]
}


fn first_word2(s: &str) -> &str {  // return slice of a string
    let bstr = s.as_bytes();
    for (i, &c) in bstr.iter().enumerate() {
        if c == b' ' {
            return &s[..i]
        } 
    } 

    return &s[..]
}
