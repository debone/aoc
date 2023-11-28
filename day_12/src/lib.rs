use std::fs;

pub fn first() {
    let contents = fs::read_to_string("day_12/test.txt").unwrap();

    let mut map: Vec<Vec<i32>> = vec![];

    let mut x = 0;
    let mut y = 0;

    let mut start_x = 0;
    let mut start_y = 0;

    for line in contents.lines() {
        let mut curr: Vec<i32> = vec![];

        for ch in line.chars() {
            if ch == 'S' {
                start_x = x;
                start_y = y;
                curr.push(0);
            } else if ch == 'E' {
                curr.push(99);
            } else {
                curr.push(ch as i32 - 38);
            }
            x += 1;
        }

        y += 1;
        x = 0;
        map.push(curr);
    }

    println!("Start point at: {}, {}", start_x, start_y);
    for line in &map {
        for point in line {
            if *point == 0 {
                print!("S");
            } else if *point == 99 {
                print!("E");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub fn second() {
    let contents = fs::read_to_string("day_12/input.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
