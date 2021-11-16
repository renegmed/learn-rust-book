use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // constructing map using iterators --
    // Creating a hash map from a list of teams and a list of scores
    let teams = vec![String::from("Blue"), String::from("Red"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Blue: {} Red: {}", scores2["Blue"], scores2["Red"]);

    /*
        The type annotation HashMap<_, _> is needed here because it’s possible to collect 
        into many different data structures and Rust doesn’t know which you want unless you 
        specify. 
        
        For the parameters for the key and value types, however, we use underscores, and 
        Rust can infer the types that the hash map contains based on the types of the data 
        in the vectors. 
        
        The key type will be String (teams) and the value type will be i32 (initial_scores)
    */

    // ------ Hash Maps and Ownership 

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and 
    // see what compiler error you get!
    println!("{}", map["Favorite color"]);
    // println!("{}", field_name);
    //             ^^^^^^^^^^ value borrowed here after move

    // -- Accessing Values in a Hash Map 
    // let mut scores = HashMap::new();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    /*

        Here, score will have the value that’s associated with the Blue team, and the result 
        will be Some(&10). The result is wrapped in Some because get returns an Option<&V>; 
        if there’s no value for that key in the hash map, get will return None. 

    */

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 20);
    scores3.insert(String::from("Yellow"), 35);
    for (key, value) in &scores3 {
        println!("key {}: value: {}", key, value);
    }

    // ---- Updating HashMap 

    println!("{:?}", scores);

    // --- only inserting a value if the key has no value 
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);
    
    println!("Blue scores4 {}", scores4["Blue"]);

    scores4.entry(String::from("Yellow")).or_insert(50);
    scores4.entry(String::from("Blue")).or_insert(50);
    println!("scores4 {:#?}", scores4);
    println!("Blue scores4 {}", scores4["Blue"]);
    println!("Yellow scores4 {}", scores4["Yellow"]);
    /*
        The or_insert method on Entry is defined to return a mutable reference to the 
        value for the corresponding Entry key if that key exists, and if not, inserts 
        the parameter as the new value for this key and returns a mutable reference 
        to the new value. 
    */
    
    // ----- Updating a value based on the old value 

    let text = "hello world wonderful world hello again";  // &str 
    let mut map = HashMap::new();
   
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;

        // map.entry(word).or_insert(1);
        // count +=1;
    }
    /*
        the or_insert method actually returns a mutable reference (&mut V) 
        to the value for this key. Here we store that mutable reference in 
        the count variable, so in order to assign to that value, we must 
        first dereference count using the asterisk (*)
    */
    println!("text map {:?}", map);
    //println!("count: {}", count);



}
