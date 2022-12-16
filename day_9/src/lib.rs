use std::fs;

/*

...... ...... ...... ......
...... ...... ...... ......
...... ...... ...... ......
...... ....H. ....H. ......
....H. ...... ....A. ....H.
..BA.. ..BA.. ..B... ..BA..


.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..... -> ...H. -> ..TH.
.TH..    .T...    .....
.....    .....    .....
 */

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn dist(&self, b: &Pos) -> f64 {
        let x = b.0 - self.0;
        let y = b.1 - self.1;

        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }

    pub fn equals(&self, b: &Pos) -> bool {
        self.0 == b.0 && self.1 == b.1
    }
    pub fn add(&mut self, b: &Pos) -> &Pos {
        self.0 += b.0;
        self.1 += b.1;
        self
    }
}

pub fn next_step(prev_h: Pos, curr_h: Pos, prev_t: Pos) -> Pos {
    let dist = prev_t.dist(&curr_h);
    if dist >= 1.5 {
        if (prev_h.dist(&curr_h) > 1.0) {
            if (dist == 2.0) {
                if prev_t.0 == curr_h.0 {
                    return Pos(prev_t.0, prev_t.1 + curr_h.1 - prev_h.1);
                } else {
                    return Pos(prev_t.0 + curr_h.0 - prev_h.0, prev_t.1);
                }
            }
            return Pos(
                prev_t.0 + curr_h.0 - prev_h.0,
                prev_t.1 + curr_h.1 - prev_h.1,
            );
        }
        return prev_h;
    }
    prev_t
}

pub fn first() {
    let contents = fs::read_to_string("day_9/input.txt").unwrap();

    let mut map: Vec<Vec<i32>> = vec![];
    let size = 500;
    let middle = size / 2;

    for _ in 0..size {
        map.push(vec![0; size]);
    }

    let mut has_passed = map.clone();

    let mut current_head = Pos(middle as i32, middle as i32);
    let mut previous_head = Pos(middle as i32, middle as i32);
    let mut current_tail = Pos(middle as i32, middle as i32);

    let mut visited = 0;

    for command in contents.lines() {
        let (direction, steps) = command.split_once(' ').unwrap();
        let steps: i32 = steps.parse().unwrap();
        let direction = match direction {
            "U" => Pos(0, -1),
            "D" => Pos(0, 1),
            "L" => Pos(-1, 0),
            "R" => Pos(1, 0),
            _ => unreachable!("Nope"),
        };

        for _ in 0..steps {
            previous_head = current_head;
            current_head.add(&direction);
            current_tail = next_step(previous_head, current_head, current_tail);

            if has_passed[current_tail.1 as usize][current_tail.0 as usize] == 0 {
                has_passed[current_tail.1 as usize][current_tail.0 as usize] = 1;
                visited += 1;
            }
        }
    }

    println!("{visited}")
}

pub fn second() {
    let contents = fs::read_to_string("day_9/input.txt").unwrap();

    let mut map: Vec<Vec<i32>> = vec![];
    let size = 500;
    let middle = size / 2;

    for _ in 0..size {
        map.push(vec![0; size]);
    }

    let mut has_passed = map.clone();

    let mut current_head = Pos(middle as i32, middle as i32);
    let mut previous_head = Pos(middle as i32, middle as i32);
    let mut previous_tails = vec![Pos(middle as i32, middle as i32); 9];
    let mut current_tails = vec![Pos(middle as i32, middle as i32); 9];

    let mut visited = 0;

    for command in contents.lines() {
        println!("== {command} ==");

        let (direction, steps) = command.split_once(' ').unwrap();
        let steps: i32 = steps.parse().unwrap();
        let direction = match direction {
            "U" => Pos(0, -1),
            "D" => Pos(0, 1),
            "L" => Pos(-1, 0),
            "R" => Pos(1, 0),
            _ => unreachable!("Nope"),
        };

        for _ in 0..steps {
            previous_head = current_head;
            current_head.add(&direction);

            previous_tails[0] = current_tails[0];
            current_tails[0] = next_step(previous_head, current_head, current_tails[0]);

            let mut i = 1;
            for _ in 1..9 {
                previous_tails[i] = current_tails[i];
                current_tails[i] = next_step(
                    previous_tails[i - 1],
                    current_tails[i - 1],
                    current_tails[i],
                );
                i += 1;
            }

            if has_passed[current_tails[8].1 as usize][current_tails[8].0 as usize] == 0 {
                has_passed[current_tails[8].1 as usize][current_tails[8].0 as usize] = 1;
                visited += 1;
            }

            /*
            let left = 0 as i32;
            let mut pros = Pos(0, 0);
            for _ in 0..size {
                for _ in 0..size {
                    let mut print = false;
                    if current_head.equals(&pros) {
                        print!("H");
                        print = true;
                    }
                    let mut i = 1;
                    for tail in &current_tails {
                        if !print && tail.equals(&pros) {
                            print!("{i}");
                            print = true;
                        }
                        i += 1;
                    }

                    if !print {
                        print!(".");
                    }
                    pros.0 += 1;
                }
                print!("\n");
                pros.0 = left;
                pros.1 += 1;
            }
            print!("\n\n");*/
        }
    }

    let left = 0 as i32;
    let mut pros = Pos(0, 0);
    for _ in 0..size {
        for _ in 0..size {
            if has_passed[pros.1 as usize][pros.0 as usize] > 0 {
                print!("#");
            } else {
                print!(".");
            }
            pros.0 += 1;
        }
        print!("\n");
        pros.0 = left;
        pros.1 += 1;
    }

    println!("{visited}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_axis() {
        //Moves in same axis
        assert_eq!(next_step(Pos(2, 1), Pos(3, 1), Pos(1, 1)), Pos(2, 1));
        assert_eq!(next_step(Pos(1, 2), Pos(1, 3), Pos(1, 1)), Pos(1, 2));
        assert_eq!(next_step(Pos(2, 0), Pos(1, 0), Pos(3, 0)), Pos(2, 0));
    }

    #[test]
    fn stays_in_place() {
        // Stays in place
        assert_eq!(next_step(Pos(0, 0), Pos(0, 1), Pos(0, 0)), Pos(0, 0));
        assert_eq!(next_step(Pos(0, 1), Pos(0, 0), Pos(0, 0)), Pos(0, 0));
        assert_eq!(next_step(Pos(1, 5), Pos(1, 4), Pos(0, 5)), Pos(0, 5));
        assert_eq!(next_step(Pos(1, 0), Pos(1, 1), Pos(2, 0)), Pos(2, 0));
        assert_eq!(next_step(Pos(1, 1), Pos(2, 1), Pos(2, 0)), Pos(2, 0));
        assert_eq!(next_step(Pos(2, 1), Pos(3, 1), Pos(2, 0)), Pos(2, 0));
    }

    #[test]
    fn diagonals() {
        // Diagonals
        assert_eq!(next_step(Pos(2, 2), Pos(2, 1), Pos(1, 3)), Pos(2, 2));
        assert_eq!(next_step(Pos(2, 2), Pos(3, 2), Pos(1, 3)), Pos(2, 2));
        assert_eq!(next_step(Pos(3, 1), Pos(4, 1), Pos(2, 0)), Pos(3, 1));
    }

    /*
    ..... -> ...H. -> ..TH.
    .TH..    .T...    .....
    .....    .....    .....
    */
    #[test]
    fn more_diagonals() {
        // Diagonals
        assert_eq!(next_step(Pos(2, 1), Pos(3, 0), Pos(1, 1)), Pos(2, 0));
    }
}
