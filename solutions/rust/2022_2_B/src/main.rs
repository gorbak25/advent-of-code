extern crate support;

fn main() {
    println!("{}",
        support::test_data!()
        .split("\n")
        .fold(0, |acc, x| {
                let wins = [2, 0, 1];
                let mut s = x.split(' ');
                // What opponent plays
                let o = s.next().unwrap().chars().next().unwrap() as u64 - b'A' as u64;
                // What should be the outcome?
                let r = s.next().unwrap().chars().next().unwrap() as u64 - b'X' as u64;
                acc
                + r * 3
                + 1 + match (o, r) {
                    // Loss
                    (x, 0) => wins[x as usize],
                    // Draw
                    (x, 1) => x,
                    // Wins
                    (x, 2) => wins.iter().position(|&y| y == x).unwrap() as u64,
                    _ => panic!("Impossible")
                } 
            }
        )
    )
}
