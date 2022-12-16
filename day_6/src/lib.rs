use std::fs;

pub fn first() {
    let contents = fs::read_to_string("day_6/input.txt").unwrap();

    for line in contents.lines() {
        let mut unique = 0;
        let mut marker = 0;
        let mut looped: Vec<char> = vec![];
        for ch in line.chars() {
            if !looped.contains(&ch) {
                unique += 1;
            }
            looped.push(ch);
            marker += 1;

            if unique == 4 {
                print!("");
                break;
            }

            if looped.len() == 4 {
                let killed = looped[0];
                looped = looped[1..4].to_vec();
                if (!looped.contains(&killed)) {
                    unique -= 1;
                }
            }
        }
        println!("Marker found at {marker}");
    }
}

pub fn second() {
    let contents = fs::read_to_string("day_6/input.txt").unwrap();

    for line in contents.lines() {
        let mut unique = 0;
        let mut marker = 0;
        let mut looped: Vec<char> = vec![];
        for ch in line.chars() {
            if !looped.contains(&ch) {
                unique += 1;
            }
            looped.push(ch);
            marker += 1;

            if unique == 14 {
                print!("");
                break;
            }

            if looped.len() == 14 {
                let killed = looped[0];
                looped = looped[1..14].to_vec();
                if !looped.contains(&killed) {
                    unique -= 1;
                }
            }
        }
        println!("Marker found at {marker}");
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
