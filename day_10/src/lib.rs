use std::fs;

#[derive(Debug, PartialEq)]
enum Commands {
    Null,
    Noop(i32),
    Add(i32, i32),
}

pub fn first() {
    let contents = fs::read_to_string("day_10/input.txt").unwrap();

    let mut commands = contents.lines();

    let mut signal_sums = 0;

    let mut register_x = 1;

    let mut current_cycle = 1;
    let max_cycles = 221;

    let mut current_command = Commands::Null;

    for _ in 0..max_cycles {
        if current_command == Commands::Null {
            current_command = match commands.next().unwrap() {
                "noop" => Commands::Noop(0),
                d => {
                    let (_, size) = d.split_once(' ').unwrap();
                    Commands::Add(1, size.parse().unwrap())
                }
            }
        }

        if (current_cycle + 20) % 40 == 0 {
            println!("{}: {}", current_cycle, register_x * current_cycle);
            signal_sums += register_x * current_cycle;
        }

        current_command = match current_command {
            Commands::Noop(cycles) => Commands::Null,
            Commands::Add(cycles, amount) => match cycles {
                2 => Commands::Add(1, amount),
                1 => Commands::Add(0, amount),
                0 => {
                    register_x += amount;
                    Commands::Null
                }
                _ => unreachable!("wololo"),
            },
            _ => unreachable!("wololo"),
        };

        current_cycle += 1;
    }

    println!("signal sum {}", signal_sums);
}

pub fn second() {
    let contents = fs::read_to_string("day_10/input.txt").unwrap();

    let mut commands = contents.lines();

    let mut register_x = 1;

    let mut current_cycle = 1;
    let max_cycles = 240;

    let mut current_command = Commands::Null;

    for _ in 0..max_cycles {
        if current_command == Commands::Null {
            current_command = match commands.next().unwrap() {
                "noop" => Commands::Noop(0),
                d => {
                    let (_, size) = d.split_once(' ').unwrap();
                    Commands::Add(1, size.parse().unwrap())
                }
            }
        }

        if current_cycle % 40 == register_x
            || current_cycle % 40 == register_x + 2
            || current_cycle % 40 == register_x + 1
        {
            print!("#");
        } else {
            print!(".");
        }

        if (current_cycle) % 40 == 0 {
            print!("\n");
        }

        current_command = match current_command {
            Commands::Noop(cycles) => Commands::Null,
            Commands::Add(cycles, amount) => match cycles {
                2 => Commands::Add(1, amount),
                1 => Commands::Add(0, amount),
                0 => {
                    register_x += amount;
                    Commands::Null
                }
                _ => unreachable!("wololo"),
            },
            _ => unreachable!("wololo"),
        };

        current_cycle += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
