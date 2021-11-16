fn main() {
    println!("Hello, world!");
}

// test_1
#[test]
fn iterator_demonstration(){ 
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();
    
    // let i:Option<_> = v1_iter.next();
    // assert_eq!(i, Some(&1));
    // let i:Option<_> = v1_iter.next();
    // assert_eq!(i, Some(&2));

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);

    /*
        Note that we needed to make v1_iter mutable: calling the next method on an 
        iterator changes internal state that the iterator uses to keep track of where 
        it is in the sequence. 
        
        In other words, this code consumes, or uses up, the iterator. Each call to next 
        eats up an item from the iterator. We didn’t need to make v1_iter mutable when 
        we used a for loop because the loop took ownership of v1_iter and made it mutable 
        behind the scenes.
    */
}

// test_2
#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    /*
        sum method, which takes ownership of the iterator and iterates through the 
        items by repeatedly calling next, thus consuming the iterator. As it iterates 
        through, it adds each item to a running total and returns the total when 
        iteration is complete. 

        We aren’t allowed to use v1_iter after the call to sum because sum takes 
        ownership of the iterator we call it on.
    */

    // let total2: i32 = v1_iter.sum();
    // //                ^^^^^^^ value used here after move
    // assert_eq!(total2, 6); 
}

// test_3
#[test]
fn produce_other_iterators() {
    let v1: Vec<i32> = vec![1,2,3];
    let i = v1.iter().map(|x| x + 1);

    let v2: Vec<_> =  i.collect();
    
    assert_eq!(v2, vec![2,3,4]);

    /*
        Calling the map method to create a new iterator and then calling the 
        collect method to consume the new iterator and create a vector
    */
}

// test_4
#[test]
fn closure_captures_environment() {

    #[derive(PartialEq,Debug)]
    struct Shoe{
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

   
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10); // The shoes_in_size function takes ownership of a vector of shoes

    // println!("{:?}", shoes);
    // //               ^^^^^ value borrowed here after move

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

// test_5
#[test]
fn create_own_iterator_trait() {
    struct Counter {
        count: u32,  // This field holds a u32 value that will keep 
                     // track of where we are in the process of iterating from 1 to 5
    }                
    
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1; 
                Some(self.count)
            } else {
                None
            }
        }
 
    }

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}
// test_6
// #[test]
// fn using_other_iterator_trait_methods() {
//     struct Counter {
//         count: i32,  // This field holds a u32 value that will keep 
//                      // track of where we are in the process of iterating from 1 to 5
//         sum: i32,
//     }
    
//     impl Counter {
//         fn new() -> Counter {
//             Counter { count: 0, sum: 0 }
//         } 
//     }
 
//     impl Iterator for Counter {
//         type Item = i32;

//         fn next(&mut self) -> Option<Self::Item> {
//             if self.count < 5 {
//                 self.count += 1; 
//                 self.sum += self.count;
//                 Some(self.count)
//             } else {
//                 None
//             }
//         }

//         fn sum(&mut self) -> i32 {
//             self.sum
//         }
 
//     }


//     let sum: i32 = Counter::new()
//         // .zip(Counter::new().skip(1))
//         // .map(|(a, b)| a * b)
//         // .filter(|x| x % 3 == 0)
//         .sum();
//     assert_eq!(18, sum);
// }