use std::fs;

use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

#[derive(Debug)]
enum CommandsOrder {
    Monkey,
    Items,
    Operation,
    Test,
    Yes,
    No,
}

struct Monkey {
    items: Vec<BigUint>,
    operation: Box<dyn Fn(BigUint) -> BigUint>,
    test: BigUint,
    yes: usize,
    no: usize,
    inspected: u32,
}

pub fn first() {
    let contents = fs::read_to_string("day_11/test.txt").unwrap();

    let mut monkeys: Vec<Monkey> = vec![];

    let mut comm = CommandsOrder::Monkey;
    let mut curr_monkey = 0;
    let mut items: Vec<BigUint> = vec![];

    for line in contents.lines() {
        println!("{:?}", comm);
        match &comm {
            CommandsOrder::Monkey => {
                curr_monkey = line.parse().unwrap();
            }
            CommandsOrder::Items => {
                for item in line.split(',') {
                    items.push(item.parse().unwrap());
                }
            }
            CommandsOrder::Operation => {
                let (operation, value) = line.split_once(' ').unwrap();

                let operation = match operation {
                    "*" => match value {
                        "old" => {
                            let wololo: Box<dyn Fn(BigUint) -> BigUint> =
                                Box::new(move |i| i.clone() * i);
                            wololo
                        }
                        a => {
                            let value = BigUint::from(value.parse::<u32>().unwrap());
                            let wololo: Box<dyn Fn(BigUint) -> BigUint> =
                                Box::new(move |i| i * value.clone());
                            wololo
                        }
                    },
                    "+" => {
                        let value = BigUint::from(value.parse::<u32>().unwrap());
                        let wololo: Box<dyn Fn(BigUint) -> BigUint> =
                            Box::new(move |i| i + value.clone());
                        wololo
                    }
                    _ => unreachable!("wololo"),
                };

                monkeys.push(Monkey {
                    items: items.clone(),
                    operation,
                    test: Zero::zero(),
                    yes: 0,
                    no: 0,
                    inspected: 0,
                })
            }
            CommandsOrder::Test => {
                monkeys[curr_monkey].test = line.parse().unwrap();
            }
            CommandsOrder::Yes => {
                monkeys[curr_monkey].yes = line.parse().unwrap();
            }
            CommandsOrder::No => {
                monkeys[curr_monkey].no = line.parse().unwrap();
            }
        }

        comm = match comm {
            CommandsOrder::Monkey => CommandsOrder::Items,
            CommandsOrder::Items => CommandsOrder::Operation,
            CommandsOrder::Operation => CommandsOrder::Test,
            CommandsOrder::Test => CommandsOrder::Yes,
            CommandsOrder::Yes => CommandsOrder::No,
            CommandsOrder::No => {
                items = vec![];
                CommandsOrder::Monkey
            }
        }
    }

    let mut next_items: Vec<Vec<BigUint>> = vec![];

    for i in 0..monkeys.len() {
        next_items.push(monkeys[i].items.clone());
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let items = &monkey.items;

            for item in items {
                let item = item;
                let item = (monkey.operation)(item.clone());
                let item = &(item / BigUint::from(3 as u32));
                let worry = item % &monkey.test == BigUint::from(0 as u32);

                let next_monkey;
                if worry {
                    next_monkey = &mut next_items[monkey.yes];
                } else {
                    next_monkey = &mut next_items[monkey.no];
                }

                next_monkey.push(item.clone());
            }

            for h in 0..monkeys.len() {
                if h == i {
                    monkeys[h].inspected += monkeys[h].items.len() as u32;
                    monkeys[h].items = vec![];
                    next_items[h] = vec![];
                } else {
                    monkeys[h].items = next_items[h].clone();
                }
            }
        }
    }

    println!("!")
}

pub fn second() {
    let contents = fs::read_to_string("day_11/input.txt").unwrap();

    let mut monkeys: Vec<Monkey> = vec![];

    let mut comm = CommandsOrder::Monkey;
    let mut curr_monkey = 0;
    let mut items: Vec<BigUint> = vec![];

    for line in contents.lines() {
        println!("{:?}", comm);
        match &comm {
            CommandsOrder::Monkey => {
                curr_monkey = line.parse().unwrap();
            }
            CommandsOrder::Items => {
                for item in line.split(',') {
                    items.push(item.parse().unwrap());
                }
            }
            CommandsOrder::Operation => {
                let (operation, value) = line.split_once(' ').unwrap();

                let operation = match operation {
                    "*" => match value {
                        "old" => {
                            let wololo: Box<dyn Fn(BigUint) -> BigUint> =
                                Box::new(move |i| i.clone() * i);
                            wololo
                        }
                        a => {
                            let value = BigUint::from(value.parse::<u32>().unwrap());
                            let wololo: Box<dyn Fn(BigUint) -> BigUint> =
                                Box::new(move |i| i * value.clone());
                            wololo
                        }
                    },
                    "+" => {
                        let value = BigUint::from(value.parse::<u32>().unwrap());
                        let wololo: Box<dyn Fn(BigUint) -> BigUint> =
                            Box::new(move |i| i + value.clone());
                        wololo
                    }
                    _ => unreachable!("wololo"),
                };

                monkeys.push(Monkey {
                    items: items.clone(),
                    operation,
                    test: Zero::zero(),
                    yes: 0,
                    no: 0,
                    inspected: 0,
                })
            }
            CommandsOrder::Test => {
                monkeys[curr_monkey].test = line.parse().unwrap();
            }
            CommandsOrder::Yes => {
                monkeys[curr_monkey].yes = line.parse().unwrap();
            }
            CommandsOrder::No => {
                monkeys[curr_monkey].no = line.parse().unwrap();
            }
        }

        comm = match comm {
            CommandsOrder::Monkey => CommandsOrder::Items,
            CommandsOrder::Items => CommandsOrder::Operation,
            CommandsOrder::Operation => CommandsOrder::Test,
            CommandsOrder::Test => CommandsOrder::Yes,
            CommandsOrder::Yes => CommandsOrder::No,
            CommandsOrder::No => {
                items = vec![];
                CommandsOrder::Monkey
            }
        }
    }

    let mut next_items: Vec<Vec<BigUint>> = vec![];

    for i in 0..monkeys.len() {
        next_items.push(monkeys[i].items.clone());
    }

    for step in 0..10000 {
        println!("{step}");
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let items = &monkey.items;

            for item in items {
                let item = item;
                let item = (monkey.operation)(item.clone());
                let worry = item.clone() % BigUint::from(9699690 as u32);
                let worry_2 = item.clone() % &monkey.test;
                let next_monkey;
                if worry_2 == BigUint::from(0 as u32) {
                    next_monkey = &mut next_items[monkey.yes];
                } else {
                    next_monkey = &mut next_items[monkey.no];
                }
                next_monkey.push(worry);
            }

            for h in 0..monkeys.len() {
                if h == i {
                    monkeys[h].inspected += monkeys[h].items.len() as u32;
                    monkeys[h].items = vec![];
                    next_items[h] = vec![];
                } else {
                    monkeys[h].items = next_items[h].clone();
                }
            }
        }
    }

    println!("!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
