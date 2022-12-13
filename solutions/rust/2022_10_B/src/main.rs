extern crate support;

#[derive(Copy, Clone)]
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
    let mut ops = 
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

    let mut reg = 1;
    let mut op_end = 1;
    let mut cur_op = None;
    for cycle in 1..241 {
        // If we finished executing the operation
        if cycle == op_end && cur_op.is_some() {
            match cur_op.unwrap() {
                Instr::Nop => (),
                Instr::Add(arg) => reg += arg
            };
            cur_op = None;
        }
        
        // If we have nothing to execute we need to fetch an instruction
        if cur_op.is_none() {
            cur_op = ops.next();
            if cur_op.is_none() {
                return;
            } else {
                op_end = cycle + cycle_time(&cur_op.unwrap())
            }
        }

        if (reg - cycle % 40 + 1).abs() < 2 {
            print!("#");
        } else {
            print!(".");
        }

        // Print :)
        if cycle % 40 == 0 {
            print!("\n")
        }
    };

    // The solution is RZHFGJCB
}
