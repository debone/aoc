use std::error::Error;
use std::fs;

pub fn first() {
    let contents = fs::read_to_string("day_1/input.txt").unwrap();

    let mut max = 0;
    let mut curr = 0;

    for line in contents.lines() {
        match line {
            "" => {
                if curr > max {
                    max = curr;
                }
                curr = 0;
            }
            all => {
                curr += all.parse::<i32>().unwrap();
            }
        }
    }

    println!("{max}");
}

pub fn second() {
    let contents = fs::read_to_string("day_1/input.txt").unwrap();

    let mut elves = vec![];
    let mut curr = 0;

    for line in contents.lines() {
        match line {
            "" => {
                elves.push(curr);
                curr = 0;
            }
            all => {
                curr += all.parse::<i32>().unwrap();
            }
        }
    }
    // 264
    // 758
    // 197638
    elves.sort();
    println!(
        "{}",
        elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
