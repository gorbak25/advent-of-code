extern crate support;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    println!("{}",
        support::test_data!()
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0, |acc, v| {
                let s: HashSet<char> = 
                    v
                    .map(|x| HashSet::from_iter(x.chars()))
                    .reduce(|l, r| HashSet::from_iter(l.intersection(&r).map(|x| *x)))
                    .unwrap();
                let r: u64 = s.into_iter().next().unwrap() as u64;
                acc + match r {
                    x if x >= 'a' as u64 && x <= 'z' as u64 => x - 'a' as u64 + 1,
                    x if x >= 'A' as u64 && x <= 'Z' as u64 => x - 'A' as u64 + 27,
                    _ => panic!("Impossible!")
                }
            }
        )
    )
}
