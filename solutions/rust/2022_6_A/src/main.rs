extern crate support;
use std::collections::HashSet;

// Try to make a one liner
fn main() {
    println!("{}",
    support::test_data!()
    .trim()
    .as_bytes()
    .windows(4)
    .enumerate()
    .find(|(_, x)| HashSet::<&u8>::from_iter(x.into_iter()).len() == 4)
    .unwrap().0 + 4
    );
    ()
}
