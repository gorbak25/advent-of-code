extern crate support;
use std::collections::HashSet;
use std::iter;

fn main() {
    let unique_positions = 
        support::test_data!()
        .lines()
        .flat_map(|line| {
            let mut x = line.split(' ');
            let (dir, delta) = (x.next().unwrap(), x.next().unwrap().parse::<usize>().unwrap());
            let dir = match dir {
                "U" => (0 as i64, 1 as i64),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!("Impossible!")
            };
            iter::repeat(dir).take(delta)
        })
        .fold(((0, 0), (0, 0), HashSet::<(i64, i64)>::new()), 
            |(h1, t1, mut visited), delta| {
                let h2 = (h1.0 + delta.0, h1.1 + delta.1);
                let t2 = match (h2, t1) {
                    // No movement
                    ((x1, y1), (x2, y2)) if (x1 - x2).abs() <= 1 && (y1 - y2).abs() <= 1 => 
                        t1,
                    // Movement Horizontal
                    ((h2x, h2y), (t1x, _t1y)) if (h2x - t1x).abs() == 2 => 
                        (t1x + (h2x - t1x)/2, h2y),
                    // Movement Vertical
                    ((h2x, h2y), (_t1x, t1y)) if (h2y - t1y).abs() == 2 => 
                        (h2x, t1y + (h2y - t1y)/2),
                    _ => panic!("Impossible")
                };
                visited.insert(t2);
                (h2, t2, visited)
            }).2.len();

    println!("{}", unique_positions);
}
