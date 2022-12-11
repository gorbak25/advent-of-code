extern crate support;
#[macro_use] extern crate scan_fmt;
use itertools::Itertools;

// Try to make a one liner
fn main() {
    println!("{}",
    support::test_data!()
    .split("\n\n")
    .tuples::<(_,_)>()
    .take(1)
    .map(
        |(boxes, instr)|
        instr.lines()
        .map(|x| scan_fmt!(x, "move {} from {} to {}", usize, usize, usize).unwrap())
        .fold(
            boxes.lines().map(
                |x|
                x.chars()
                .collect::<Vec<char>>()
                .into_iter()
                .chunks(4)
                .into_iter()
                .map(
                    |c| 
                    c.into_iter()
                    .collect::<String>()
                    .trim()
                    .chars()
                    .nth(1)
                    .to_owned()
                ).collect::<Vec<_>>()
            ).collect::<Vec<Vec<_>>>()
            .split_last().unwrap().1.to_owned()
            // Time to transpose it
            .into_iter()
            .fold([vec![]], |[mut acc], el| {acc.push(el); [acc]})
            .map(|iters| (0..iters[0].len()).map(|i| iters.iter().map(|x| x[i]).collect::<Vec<Option<char>>>() ).collect::<Vec<Vec<Option<char>>>>() )
            .into_iter()
            .nth(0)
            .unwrap()
            .into_iter()
            .map(|mut x| {x.retain(|c| c.is_some()); x.into_iter().rev().map(|x| x.unwrap()).collect::<Vec<char>>()})
            .collect::<Vec<Vec<char>>>(),
            |acc, (amount, from, to)| {
                (0..amount).fold(acc, |mut acc, _| {
                    let v = acc[from-1].pop().unwrap();
                    acc[to-1].push(v);
                    acc})
            }
        )
    )
    .into_iter()
    .nth(0)
    .unwrap()
    .into_iter()
    .map(|x| x.last().unwrap().clone())
    .collect::<String>())
}
