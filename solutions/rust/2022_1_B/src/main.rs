extern crate support;
use itertools::sorted;

fn main() {
    println!("{}",
        sorted(
        support::test_data!()
        .split("\n\n")
        .map(|x| 
            x.split('\n')
            .map(|x| x.parse::<u64>().unwrap())
            .into_iter()
            .sum()
        )
        .collect::<Vec<u64>>()
        )
        .into_iter()
        .rev()
        .take(3)
        .sum::<u64>()
    )
}
