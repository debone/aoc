use std::fs;

/*
[N]     [Q]         [N]
[R]     [F] [Q]     [G] [M]
[J]     [Z] [T]     [R] [H] [J]
[T] [H] [G] [R]     [B] [N] [T]
[Z] [J] [J] [G] [F] [Z] [S] [M]
[B] [N] [N] [N] [Q] [W] [L] [Q] [S]
[D] [S] [R] [V] [T] [C] [C] [N] [G]
[F] [R] [C] [F] [L] [Q] [F] [D] [P]
 1   2   3   4   5   6   7   8   9
 */

fn getInitialInputStack() -> Vec<Vec<char>> {
    vec![
        vec!['F', 'D', 'B', 'Z', 'T', 'J', 'R', 'N'],
        vec!['R', 'S', 'N', 'J', 'H'],
        vec!['C', 'R', 'N', 'J', 'G', 'Z', 'F', 'Q'],
        vec!['F', 'V', 'N', 'G', 'R', 'T', 'Q'],
        vec!['L', 'T', 'Q', 'F'],
        vec!['Q', 'C', 'W', 'Z', 'B', 'R', 'G', 'N'],
        vec!['F', 'C', 'L', 'S', 'N', 'H', 'M'],
        vec!['D', 'N', 'Q', 'M', 'T', 'J'],
        vec!['P', 'G', 'S'],
    ]
}

/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
*/
fn getInitialTestStack() -> Vec<Vec<char>> {
    vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
}

pub fn first() {
    let contents = fs::read_to_string("day_5/input.txt").unwrap();
    let mut stack = getInitialInputStack();

    for line in contents.lines() {
        let cmd: Vec<&str> = line.split('-').collect();
        let (quantity, from, to): (usize, usize, usize) = (
            cmd[0].parse().unwrap(),
            cmd[1].parse().unwrap(),
            cmd[2].parse().unwrap(),
        );

        for _ in 0..quantity {
            let the_crate = stack[from - 1].pop().unwrap();
            stack[to - 1].push(the_crate);
        }
    }

    for i in 0..stack.len() {
        print!("{}", stack[i][stack[i].len() - 1]);
    }
}

pub fn second() {
    let contents = fs::read_to_string("day_5/input.txt").unwrap();
    let mut stack = getInitialInputStack();

    for line in contents.lines() {
        let cmd: Vec<&str> = line.split('-').collect();
        let (quantity, from, to): (usize, usize, usize) = (
            cmd[0].parse().unwrap(),
            cmd[1].parse().unwrap(),
            cmd[2].parse().unwrap(),
        );

        let mut storage = vec![];
        for _ in 0..quantity {
            storage.push(stack[from - 1].pop().unwrap());
        }
        for _ in 0..quantity {
            let the_crate = storage.pop().unwrap();
            stack[to - 1].push(the_crate);
        }
    }

    for i in 0..stack.len() {
        print!("{}", stack[i][stack[i].len() - 1]);
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
