use std::fs;

pub fn getPriority(ch: char) -> i32 {
    let value = ch as i32;
    if value <= 96 {
        value - 38
    } else {
        value - 96
    }
}

pub fn first() {
    let contents = fs::read_to_string("day_3/input.txt").unwrap();

    let mut total_priority = 0;

    for line in contents.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        for ch in left.chars() {
            if right.contains(ch) {
                total_priority += getPriority(ch);
                break;
            }
        }
    }

    println!("{}", total_priority)
}

pub fn second() {
    let contents = fs::read_to_string("day_3/input.txt").unwrap();

    let mut total_priority = 0;

    let mut lines = contents.lines();

    loop {
        let first = match lines.next() {
            None => {
                break;
            }
            Some(line) => line,
        };
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        for ch in first.chars() {
            if second.contains(ch) && third.contains(ch) {
                total_priority += getPriority(ch);
                break;
            }
        }
    }

    println!("{}", total_priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(getPriority('a'), 1);
        assert_eq!(getPriority('z'), 26);
        assert_eq!(getPriority('A'), 27);
        assert_eq!(getPriority('Z'), 52);
    }
}
