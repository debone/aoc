use std::fs;

pub fn first() {
    let contents = fs::read_to_string("day_4/input.txt").unwrap();

    let mut overlaps = 0;

    for line in contents.lines() {
        let (left, right) = line.split_once(',').unwrap();

        let (l_start_range, l_end_range) = left.split_once('-').unwrap();
        let (r_start_range, r_end_range) = right.split_once('-').unwrap();

        let start_range: [i32; 2] = [
            l_start_range.parse().unwrap(),
            r_start_range.parse().unwrap(),
        ];
        let end_range: [i32; 2] = [l_end_range.parse().unwrap(), r_end_range.parse().unwrap()];

        let side = if end_range[1] - start_range[1] > end_range[0] - start_range[0] {
            1
        } else {
            0
        };
        let otherside = if side == 0 { 1 } else { 0 };

        if start_range[side] <= start_range[otherside] && end_range[side] >= end_range[otherside] {
            overlaps += 1;
        }
    }

    println!("{}", overlaps);
}

pub fn second() {
    let contents = fs::read_to_string("day_4/input.txt").unwrap();

    let mut overlaps = 0;

    for line in contents.lines() {
        let (left, right) = line.split_once(',').unwrap();

        let (l_start_range, l_end_range) = left.split_once('-').unwrap();
        let (r_start_range, r_end_range) = right.split_once('-').unwrap();

        let start_range: [i32; 2] = [
            l_start_range.parse().unwrap(),
            r_start_range.parse().unwrap(),
        ];
        let end_range: [i32; 2] = [l_end_range.parse().unwrap(), r_end_range.parse().unwrap()];

        let side = if end_range[1] - start_range[1] > end_range[0] - start_range[0] {
            0
        } else {
            1
        };
        let otherside = if side == 0 { 1 } else { 0 };

        if end_range[side] >= start_range[otherside] && end_range[side] <= end_range[otherside]
            || start_range[side] >= start_range[otherside]
                && start_range[side] <= end_range[otherside]
        {
            overlaps += 1;
        }
    }

    println!("{}", overlaps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
