extern crate support;

fn main() {
    println!("{}",
        support::test_data!()
        .split("\n\n")
        .map(|x| 
            x.split('\n')
            .map(|x| x.parse::<u64>().unwrap())
            .into_iter()
            .sum()
        )
        .collect::<Vec<u64>>()
        .into_iter()
        .reduce(u64::max)
        .unwrap()
    )
}
