extern crate support;

fn main() {
    println!("{}",
        support::test_data!()
        .split("\n")
        .filter(|&x| !x.is_empty())
        .fold(0, |acc, x| {
                let wins = [2, 0, 1];
                let mut s = x.split(' ');
                // What opponent plays
                let o = s.next().unwrap().chars().next().unwrap() as u64 - b'A' as u64;
                // How we respond
                let r = s.next().unwrap().chars().next().unwrap() as u64 - b'X' as u64;
                acc 
                + r + 1 
                + match (r, o) {
                    // Draw
                    (x, y) if x == y => 3,
                    // Wins
                    (x, y) if wins[x as usize] == y => 6,
                    // Loss
                    _ => 0
                } 
            }
        )
    )
        
}
