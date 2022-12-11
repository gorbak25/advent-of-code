extern crate support;
use std::collections::HashSet;

fn main() {
    println!("{}",
        support::test_data!()
        .split("\n")
        .fold(0, |acc, x| {
                let (c1, c2) = x.split_at(x.len()/2);
                let s1: HashSet<char> = HashSet::from_iter(c1.chars());
                let s2: HashSet<char> = HashSet::from_iter(c2.chars());
                let r: u64 = *s1.intersection(&s2).into_iter().next().unwrap() as u64;
                acc + match r {
                    x if x >= 'a' as u64 && x <= 'z' as u64 => x - 'a' as u64 + 1,
                    x if x >= 'A' as u64 && x <= 'Z' as u64 => x - 'A' as u64 + 27,
                    _ => panic!("Impossible!")
                }
            }
        )
    )
}
