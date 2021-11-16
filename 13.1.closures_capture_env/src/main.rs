use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay hydrated!"
            );
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
   
}

//---------- using functional --------------

struct Catcher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Catcher<T>
where 
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher{
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    } 
}


fn generate_workout2(intensity: u32, random_number: u32) {
    let mut expensive_result = Catcher::new( |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!(
                "Take a break today! Remember to stay hydrated!"
            );
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
   
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout2(simulated_user_specified_value, 
      simulated_random_number);

    // -- capturing the environment with closures 

  
     
}

#[test]
fn call_with_different_values() {
    let mut c = Catcher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(1, v1);
    assert_eq!(2, v2);

}

#[test]
fn capture_environment() {

    let x = 4; 
    let equal_to_x = |z| z == x;

    // this will fail
    println!("can't use x here: {:?}", x);


    // this will fail    
    // fn equal_to_x2(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;
    
    assert!(equal_to_x(y));
}

#[test]
fn capture_environment_fail() {

    let x = vec![1, 2, 3]; 
    let equal_to_x = move |z| z == x;

    // this will fail
    // println!("can't use x here: {:?}", x); 
    //                                    ^ value borrowed here after move

    let y = vec![1, 2, 3];
    
    assert!(equal_to_x(y));
}
 
 