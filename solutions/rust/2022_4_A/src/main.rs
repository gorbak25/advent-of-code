extern crate support;
#[macro_use] extern crate scan_fmt;

fn main() {
    println!("{}",
        support::test_data!()
        .lines()
        .map(|x| 
            scan_fmt!(x, "{}-{},{}-{}", u64, u64, u64, u64).unwrap()
        )
        .filter(|(s1,e1,s2,e2)| (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1))
        .collect::<Vec<_>>()
        .len()
    )
}
