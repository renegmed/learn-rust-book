enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5, 
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// -------------
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
    NewJersey,
    NewYork,    
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin2::Nickel => 5, 
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// ----------------------------
enum Option2<T> {
    None, 
    Some(T),
}

fn plus_one(x: Option2<i32>) -> Option2<i32> {
    match x {
        Option2::None => Option2::None, 
        Option2::Some(i) => Option2::Some(i + 1),
    }
}

// ----------------------------
fn add_fancy_hat() {
    println!("add_fancy_hat()");
}

fn remove_fancy_hat() {
    println!("remove_fancy_hat()");
}

fn move_player(num_spaces: u8) {
    println!("move_player {}", num_spaces);
}

fn reroll() {
    println!("reroll()");
}

fn main() {
    let coin: u8 = value_in_cents(Coin::Penny);
    println!("{}", coin);

    let coin: u8 = value_in_cents(Coin::Quarter);
    println!("{}", coin);

    let coin: u8 = value_in_cents(Coin::Dime);
    println!("{}", coin);

    let coin: u8 = value_in_cents(Coin::Nickel);
    println!("{}", coin);

    // --------------------

    let coin: u8 = value_in_cents2(Coin2::Quarter(UsState::Arizona));
    println!("{}", coin);

    // --------------------
    let five = Option2::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option2::None);

    // --------------------
    let dice_roll = 9;  
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }

     // --------------------
     // Concise control flow with if let 
     let config_max = Some(3u8);
     match config_max {
         Some(max) => println!("The maximum is configured to be {}", max),
         _ => (),
     }

     let config_max = Some(3u8);
     if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
     }
     // --------------------
     let mut count = 0;
     let coin2 : Coin2 = Coin2::Quarter(UsState::Alaska);
     match coin2 {
         Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
         _ => count += 1,
     }

     // if let and else expression 
    //  let state2 = UsState::Alaska;
    //  if let Coin2::Quarter(state2) = coin2{
    //      println!("State quarter from {:?}!", state2);
    //  } else {
    //      count += 1;
    //      println!("count {}", count);
    //  }
}
