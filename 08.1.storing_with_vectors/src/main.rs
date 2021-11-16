enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

impl SpreadsheetCell {
    fn text(&self) -> &str {
        "test"
    }
}

fn main() {
     {
         let mut v = vec![1,2,3,4];
         v.push(5);
     } // <- v goes out of scope and is freed here 

//      v.push(5);
// //   ^ not found in this scope
//      v.push(6); 

    let v1 = vec![1,2,3,4,5];
    let third: &i32 = &v1[2];
    println!("The third element is {}", third);

    match v1.get(2) {
        Some(x) => println!("The third element is {}", x),
        None => println!("There is no third element."),
    }

    println!("++++++ 2 ways to access ++++++++");
    let does_not_exist_none = v1.get(100);
    match does_not_exist_none {
        Some(x) => println!("The element is {}", x),
        None => println!("There is no element. The vector size is {}", v1.len()),
    }
    println!("++++++++++++++");

    // let _does_not_exist_panic = &v1[100];  // this would cause panic
    // println!("++++++++++++++");

    //  You cannot have mutable and immutable references in the same scope
    
    let first = &v1[0];
    println!("The first element is: {}", first);
    
    // v1.push(6); // this is immutable, cannot borrow as mutable 
    
    /*
        why should a reference to the first element care about what changes at 
        the end of the vector? 
        
        This error is due to the way vectors work: adding a new element onto 
        the end of the vector might require allocating new memory and copying 
        the old elements to the new space, if there isn’t enough room to put 
        all the elements next to each other where the vector currently is. 
        
        In that case, the reference to the first element would be pointing to 
        deallocated memory. The borrowing rules prevent programs from ending up 
        in that situation.

    */

    // Iterating over the values of a vector 
    let v2 = vec![100,32,57];
    for i in &v2 {
        println!("iterate {}", i);
    }

    let mut v3 = vec![100,32,57];
    for i in &mut v3 {
        *i += 50;
        println!("v3 *i {} {}", *i, i);
    }

    // Using an enum to store multiple types 
   
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //let item = row[0];
    match row.get(0) {
        // Some(x) => { println!("{}", x.text())}
        // None => { println!("none")}
        Text(x) => { println!("text{}", x)},
        Int(x) => { println!("int")},
        Float(x) => { println!("float")},
    }

     
    // for sc in row {
    //     match r.Int(3) {
    //         Some(x) => println!("row {}", x),
    //         None => println!("none"),
    //     }       
    // }

}
