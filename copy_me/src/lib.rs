use std::fs;

pub fn first() {
    let contents = fs::read_to_string("copy_me/input.txt").unwrap();
    for line in contents.lines() {}
}

pub fn second() {
    let contents = fs::read_to_string("copy_me/input.txt").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
