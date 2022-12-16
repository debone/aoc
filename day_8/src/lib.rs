use std::fs;

pub fn find_peeks(path: &[i32]) -> usize {
    let mut last_peek = path[0];
    let mut peeks = 0;

    for tree in path.iter() {
        if *tree > last_peek {
            last_peek = *tree;
            peeks += 1;
        }
    }

    peeks
}

pub fn first() {
    let contents = fs::read_to_string("day_8/input.txt").unwrap();

    let mut map: Vec<Vec<i32>> = vec![];
    let mut visible_trees = 0;
    let mut i = 0;

    for line in contents.lines() {
        let x: Vec<i32> = line
            .chars()
            .map(|ch| ch.to_string().parse().unwrap())
            .collect();

        map.push(x);

        if i == 0 || i == contents.lines().count() - 1 {
            visible_trees += line.chars().count();
        } else {
            /*let middle = line.chars().count() / 2;
            let (first, second) = z.split_at(middle);
            let mut second = Vec::from(second);
            second.reverse();
            visible_trees += find_peeks(first);
            visible_trees += find_peeks(&second);*/
            visible_trees += 2
        }

        i += 1;
    }

    let mut counted: Vec<Vec<i32>> = map.clone();

    let mut j = 0;
    for line in &map {
        if j > 0 && j < map.len() - 1 {
            let mut k = 0;
            let mut l = map.len() - 1;

            let middle = line.len() / 2;

            let mut last_peek_x = map[k][j];
            let mut last_peek_y = map[j][k];
            for _ in 0..map.len() - 1 {
                let this_peek_x = map[k][j];
                let has_counted = counted[k][j];
                if this_peek_x > last_peek_x {
                    last_peek_x = this_peek_x;
                    if has_counted >= 0 {
                        counted[k][j] = -1;
                        visible_trees += 1;
                    }
                }

                let this_peek_y = map[j][k];
                let has_counted = counted[j][k];
                if this_peek_y > last_peek_y {
                    last_peek_y = this_peek_y;
                    if has_counted >= 0 {
                        counted[j][k] = -1;
                        visible_trees += 1;
                    }
                }
                k += 1;
            }

            let mut last_peek_x = map[l][j];
            let mut last_peek_y = map[j][l];
            for _ in 0..map.len() - 1 {
                let this_peek_x = map[l][j];
                let has_counted = counted[l][j];
                if this_peek_x > last_peek_x {
                    last_peek_x = this_peek_x;
                    if has_counted >= 0 {
                        counted[l][j] = -1;
                        visible_trees += 1;
                    }
                }

                let this_peek_y = map[j][l];
                let has_counted = counted[j][l];
                if this_peek_y > last_peek_y {
                    last_peek_y = this_peek_y;
                    if has_counted >= 0 {
                        counted[j][l] = -1;
                        visible_trees += 1;
                    }
                }
                l -= 1;
            }
        }
        j += 1;
    }

    println!("{visible_trees}");
}

pub fn second() {
    let contents = fs::read_to_string("day_8/input.txt").unwrap();

    let mut map: Vec<Vec<i32>> = vec![];

    for line in contents.lines() {
        let x: Vec<i32> = line
            .chars()
            .map(|ch| ch.to_string().parse().unwrap())
            .collect();

        map.push(x);
    }

    let mut max_visible_tree: i32 = 0;
    let mut max_visible_tree_value = 0;
    let mut max_visible_tree_x = 0;
    let mut max_visible_tree_y = 0;

    let mut x = 0;
    let mut y = 0;

    let mut pointer_x = 0;
    let mut pointer_y = 0;

    let mut trees_score = 0;
    let mut trees_viewed_n = 0;
    let mut trees_viewed_s = 0;
    let mut trees_viewed_w = 0;
    let mut trees_viewed_e = 0;
    let mut current_tree = 0;
    for line in &map {
        for _ in line {
            // Go north
            pointer_x = x;
            pointer_y = y;

            current_tree = map[y][x];

            loop {
                if pointer_y > 0 {
                    pointer_y -= 1;
                    if map[pointer_y][pointer_x] < current_tree {
                        trees_viewed_n += 1;
                    } else {
                        trees_viewed_n += 1;
                        break;
                    }
                } else {
                    break;
                }
            }

            // Go south
            pointer_x = x;
            pointer_y = y;

            loop {
                if pointer_y < map.len() - 1 {
                    pointer_y += 1;
                    if map[pointer_y][pointer_x] < current_tree {
                        trees_viewed_s += 1;
                    } else {
                        trees_viewed_s += 1;
                        break;
                    }
                } else {
                    break;
                }
            }

            // Go west
            pointer_x = x;
            pointer_y = y;

            loop {
                if pointer_x > 0 {
                    pointer_x -= 1;
                    if map[pointer_y][pointer_x] < current_tree {
                        trees_viewed_w += 1;
                    } else {
                        trees_viewed_w += 1;
                        break;
                    }
                } else {
                    break;
                }
            }

            // Go east
            pointer_x = x;
            pointer_y = y;

            loop {
                if pointer_x < map.len() - 1 {
                    pointer_x += 1;
                    if map[pointer_y][pointer_x] < current_tree {
                        trees_viewed_e += 1;
                    } else {
                        trees_viewed_e += 1;
                        break;
                    }
                } else {
                    break;
                }
            }

            trees_score = trees_viewed_n * trees_viewed_s * trees_viewed_w * trees_viewed_e;

            if trees_score > max_visible_tree {
                max_visible_tree = trees_score;
                max_visible_tree_value = current_tree;
                max_visible_tree_x = x;
                max_visible_tree_y = y;
            }

            x += 1;
            trees_viewed_n = 0;
            trees_viewed_s = 0;
            trees_viewed_w = 0;
            trees_viewed_e = 0;
        }
        y += 1;
        x = 0;
    }

    println!("{max_visible_tree_value} with {max_visible_tree} at {max_visible_tree_x} {max_visible_tree_y}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
