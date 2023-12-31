// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//         assert!(larger.can_hold(&smaller));
//     }
//
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };
//
//         assert!(!smaller.can_hold(&larger));
//     }
// }

////////////////////////////////////////////////////////////////

// pub fn add_two(a: i32) -> i32 {
//     a + 3
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_adds_two() {
//         assert_ne!(4, add_two(2));
//     }
// }

////////////////////////////////////////////////////////////////

// Custom Failure Messages

// pub fn greeting(name: &str) -> String {
//     format!("Hello!")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, valid was {}",
//             result
//         );
//     }
// }

////////////////////////////////////////////////////////////////

// Checking for Panics with should_panic

// pub struct Guess {
//     value: i32,
// }
//
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }
//
//         Guess { value }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(-200);
//     }
// }

////////////////////////////////////////////////////////////////////////

// Using Result<T, E> in Tests

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

////////////////////////////////////////////////////////////////

// Showing Function Output

// cargo test -- --show-output
// cargo test --help
// cargo test -- --help -> for bin

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }
//
//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

////////////////////////////////////////////////////////////////

// Running a Subset of Tests by Name

// cargo test add
// cargo test one
// cargo test one_hundred
// cargo test tests::

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn add_two_and_two() {
//         assert_eq!(4, add_two(2));
//     }
//
//     #[test]
//     fn add_three_and_two() {
//         assert_eq!(5, add_two(3));
//     }
//
//     #[test]
//     fn one_hundred() {
//         assert_eq!(102, add_two(100));
//     }
// }

////////////////////////////////////////////////////////////////

// Ignoring Tests

// cargo test -- --ignored
// cargo test -- --include-ignored

// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }
//
// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }

////////////////////////////////////////////////////////////////

// Test Organization
// cargo test --test integration_test

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
