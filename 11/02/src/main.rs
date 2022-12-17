extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "monkeys.pest"]
struct MonkeyParser;

#[derive(Default, Debug)]
struct Monkey {
    index: usize,
    operation: String,
    operand: String,
    divisor: u64,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug)]
struct MonkeyItems {
    items: Vec<u64>,
}

#[derive(Debug)]
struct MonkeyBusiness {
    inspected: u64,
}

// 2713310158

fn main() {
    let rounds = 10000;
    let input = fs::read_to_string("input.txt").expect("Missing input file");
    let (monkeys, mut monkey_items): (Vec<Monkey>, Vec<MonkeyItems>) =
        parse_monkeys(input).into_iter().unzip();

    let mut monkey_business: Vec<MonkeyBusiness> =
        (0..monkeys.len()).map(|_| MonkeyBusiness { inspected: 0 }).collect();

    let modulo = (&monkeys).into_iter().fold(1, |a, m| a * m.divisor);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];

            let items = monkey_items[i].items.clone();
            monkey_items[i].items.clear();
            
            for item in items {
                monkey_business[i].inspected += 1;
                let worry = update_worry(&monkey.operation, &monkey.operand, item) % modulo;
                let target = choose_monkey(worry, monkey.divisor, monkey.true_target, monkey.false_target);

                monkey_items[target].items.push(worry);
            }
        }
    }

    monkey_business.sort_by(|m, _m| _m.inspected.cmp(&m.inspected));
    monkey_business.truncate(2);
    
    println!("{:?}", monkey_business.into_iter().fold(1, |a, m| a * m.inspected));
}

fn update_worry(operation: &String, operand: &String, current: u64) -> u64 {
    let num_operand: u64 = match operand.as_str() {
        "old" => current,
        _ => operand.parse().unwrap()
    };

    match operation.as_str() {
        "+" => current + num_operand,
        "*" => current * num_operand,
        _ => unreachable!()
    }
}

fn choose_monkey(item: u64, divisor: u64, true_target: usize, false_target: usize) -> usize {
    if item % divisor == 0 {
        true_target
    } else {
        false_target
    }
}

fn parse_monkeys(input: String) -> Vec<(Monkey, MonkeyItems)> {
    let monkeys_raw = MonkeyParser::parse(Rule::monkeys, input.as_str())
        .unwrap_or_else(|e| panic!("{:?}", e));

    let mut monkeys = vec![];
    
    for monkey_raw in monkeys_raw {
        let mut monkey = Monkey { ..Default::default() };
        let mut monkey_items = MonkeyItems { items: vec![] };

        for inner_monkey in monkey_raw.into_inner() {
            match inner_monkey.as_rule() {
                Rule::monkey_index => {
                    let index: usize = inner_monkey.into_inner()
                        .map(|r| r.as_str()).next().unwrap().parse().unwrap();

                    monkey.index = index;
                },
                
                Rule::starting_items => {
                    let items: Vec<u64> = inner_monkey.into_inner()
                        .flat_map(|r| r.into_inner()
                            .map(|_r| _r.as_str().parse().unwrap())
                            .collect::<Vec<u64>>())
                        .collect();

                    monkey_items = MonkeyItems { items: items }
                },
                
                Rule::operation => {
                    let mut text = inner_monkey.into_inner()
                        .map(|r| r.as_str());
                    let operation = text.next().unwrap().to_string();
                    let operand = text.next().unwrap().to_string();

                    monkey.operation = operation;
                    monkey.operand = operand;
                },

                Rule::test => {
                    let divisor: u64 = inner_monkey.into_inner()
                        .map(|r| r.as_str()).next().unwrap().parse().unwrap();
                
                    monkey.divisor = divisor;
                },
                
                Rule::if_true => {
                    let target: usize = inner_monkey.into_inner()
                        .map(|r| r.as_str()).next().unwrap().parse().unwrap();

                    monkey.true_target = target;
                },
                
                Rule::if_false =>  {
                    let target: usize = inner_monkey.into_inner()
                        .map(|r| r.as_str()).next().unwrap().parse().unwrap();

                    monkey.false_target = target;
                },

                _ => unreachable!()
            }
        }

        monkeys.push((monkey, monkey_items));
    }

    return monkeys;
}
