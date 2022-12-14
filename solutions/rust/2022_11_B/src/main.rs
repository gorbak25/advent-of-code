extern crate support;
use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{line_ending, digit1, space1, space0, char, one_of},
    combinator::{map_res, map},
    multi::separated_list1,
    sequence::{tuple, delimited, preceded, terminated},
    IResult,
  };

fn parse_number<'a, T: std::str::FromStr>(
    input: &'a str
)-> IResult<&'a str, T> {
    map_res(digit1, str::parse)(input)
}

fn parse_monkey_header<'a>(
    input: &'a str,
  ) -> IResult<&'a str, usize> {
    let (r, (_, _, x, _)) = tuple((
        tag("Monkey"), 
        space1, 
        parse_number,
        char(':'),
    ))(input)?;
    Ok((r, x))
  }

fn parse_monkey_items<'a>(
    input: &'a str,
  ) -> IResult<&'a str, Vec<u64>> {
    let (r, (_, _, _, x)) = tuple((
        space1,
        tag("Starting items:"), 
        space0, 
        separated_list1(delimited(space0, char(','), space0), parse_number),
    ))(input)?;
    Ok((r, x))
  }

  #[derive(Debug, Copy, Clone)]
  enum MonkeyOperation {
    Add(u64),
    Multiplication(u64),
    Pow2,
  }

  fn parse_monkey_operation<'a>(
    input: &'a str,
  ) -> IResult<&'a str, MonkeyOperation> {
    let (r, (_, _, op, arg)) = tuple((
        space1,
        tag("Operation: new = old"), 
        delimited(space0, one_of("+*"), space0), 
        alt((map(parse_number, |x| Some(x)), map(tag("old"), |_| None))),
    ))(input)?;
    match (op, arg) {
       ('+', Some(arg)) => Ok((r, MonkeyOperation::Add(arg))),
       ('*', Some(arg)) => Ok((r, MonkeyOperation::Multiplication(arg))),
       ('*', None) => Ok((r, MonkeyOperation::Pow2)),
       _ => panic!("Parser broken!")
    }
  }

  fn parse_monkey_test<'a>(
    input: &'a str,
  ) -> IResult<&'a str, u64> {
    let (r, (_, _, _, x)) = tuple((
        space1,
        tag("Test: divisible by"), 
        space1,
        parse_number,
    ))(input)?;
    Ok((r, x))
  }

  fn parse_monkey_condition<'a>(
    input: &'a str,
  ) -> IResult<&'a str, (bool, usize)> {
    let (r, (_, op, _, arg)) = tuple((
        preceded(space1, tag("If")), 
        delimited(space0, 
            alt((tag("true"), tag("false"))), 
            terminated(char(':'), space0)),
        tag("throw to monkey"),
        preceded(space1, parse_number),
    ))(input)?;
    match op {
       "true" => Ok((r, (true, arg))),
       "false" => Ok((r, (false, arg))),
       _ => panic!("Parser broken!")
    }
  }

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    op: MonkeyOperation,
    test: u64,
    throw_if_true: usize,
    throw_if_false: usize,
    inspections: usize
}

fn parse_monkey<'a>(
    input: &'a str,
  ) -> IResult<&'a str, Monkey> {
    let (r, (id, items, op, test, (t, throw_if_true), (f, throw_if_false))) = tuple((
        terminated(parse_monkey_header, line_ending),
        terminated(parse_monkey_items, line_ending),
        terminated(parse_monkey_operation, line_ending),
        terminated(parse_monkey_test, line_ending),
        terminated(parse_monkey_condition, line_ending),
        terminated(parse_monkey_condition, line_ending),
    ))(input)?;
    assert!(t == true && f == false);
    Ok((r, Monkey {
        id,
        items: VecDeque::from(items),
        op,
        test,
        throw_if_true,
        throw_if_false,
        inspections: 0,
    }))
  }


fn parse_monkeys<'a>(
    input: &'a str,
  ) -> IResult<&'a str, Vec<Monkey>> {
    separated_list1(line_ending, parse_monkey)(input)
  }

fn main() {
    let (r, mut monkeys) = parse_monkeys(support::test_data!()).unwrap();
    let gcd = monkeys.iter().map(|x| x.test).reduce(|x, y| x * y).unwrap();
    // All input should be consumed :)
    assert!(r == "");
    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            while let Some(worry) = monkeys[idx].items.pop_front() {
                let worry = match monkeys[idx].op {
                    MonkeyOperation::Add(arg) => (worry + arg) % gcd,
                    MonkeyOperation::Multiplication(arg) => (worry * arg) % gcd,
                    MonkeyOperation::Pow2 => (worry * worry) % gcd
                };
                let target = if worry % monkeys[idx].test == 0 { 
                    monkeys[idx].throw_if_true
                } else {
                    monkeys[idx].throw_if_false
                };
                monkeys[idx].inspections += 1;
                monkeys[target].items.push_back(worry);
            }
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections).reverse());
    println!("{}", monkeys[0].inspections * monkeys[1].inspections);
    //dbg!(monkeys);
}
