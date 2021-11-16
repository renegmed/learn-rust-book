fn main() {
    let mut s = String::new();  // empty String

    let data = "initial contents"; // type string slice &str
    let s = data.to_string();   // convert to type String

    // the method alos works on a literal directly
    let s = "initial contents".to_string();  // create a String 

    let s = String::from("initial contents");  // create a String from &str

    // strings are UTF-8 encoded 
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    

    // Appending to a String with push_str and push 

    let mut s = String::from("foo"); // convert &str to String 
    s.push_str("bar"); // take a slice &str, would result to String foobar 

    let mut s1 = String::from("foo");  // from %str to String
    let s2 = "bar"; // this is string slice &str
    s1.push_str(s2); // push &str to String s1 
    println!("s2 is {}", s2); 
    // Why it is working?If the push_str method took ownership of s2, 
    // we wouldn’t be able to print. Because s2 is slice &str, referenced string

    let mut s = String::from("lo");  // slice &str converted to String
    s.push('l'); // push a char to String
    println!("String: {}", s);

    // Concatenation with the + Operator or the format! Macro 

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::from("$%^$");
    let s4 = s1 + &s2 + &s3; // note s1 has been moved here and can no longer be used
    println!("{}", s4); // + signature -  fn add(self, s: &str) -> String {


    let s11 = String::from("tic");
    let s22 = String::from("tac");
    let s33 = String::from("toe");    
    let s44 = format!("{}-{}-{}", s11, s22, s33);
    println!("{}", s44);

    // Indexing into Strings 

    // let s6 = String::from("hello");
    // let h = s6[0];
    //      ^^^^^ `String` cannot be indexed by `{integer}`

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    //           ^^^^^^^^ string indices are ranges of `usize`        
    println!("length {}", hello.len());


}
