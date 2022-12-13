extern crate support;

enum Instr {
    Nop,
    Add(i64)
}

fn cycle_time(instr: &Instr) -> i64 {
    match instr {
        Instr::Nop => 1,
        Instr::Add(_) => 2
    }
}

fn main() {
    let ops = 
        support::test_data!()
        .lines()
        .map(
            |line| 
            if line == "noop" { Instr::Nop } 
            else { 
                Instr::Add(
                    line.split(" ")
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()) 
                }
        );

    let mut res = 0;
    let mut cycles = 1;
    let mut reg = 1;
    let mut targets = vec![220, 180, 140, 100, 60, 20];
    for op in ops {
        if targets.len() == 0 {
            break
        }
        cycles += cycle_time(&op);
        let target = targets[targets.len()-1];
        if cycles > target {
            res += reg * target;
            targets.pop();
        }
        match op {
            Instr::Add(arg) => reg += arg,
            _ => ()
        }
        if cycles == target {
            res += reg * target;
            targets.pop();
        }
    };

    println!("{}", res);
}
