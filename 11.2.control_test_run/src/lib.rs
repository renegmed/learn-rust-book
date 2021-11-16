
fn prints_and_returns_10(a: i32) -> i32 {
    println!("+++++++ I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- show output 
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_ne!(5, value);
    }

    // --- running a subset of tests by name 
    #[test]
    fn add_two_and_two() {  // $ cargo test add
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {   // $ cargo test one_hundred
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(152, add_two(150));
    }
}