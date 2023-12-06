use std::collections::HashMap;
pub fn p1() -> u32 {
    let mut sum = 0;
    let input = std::fs::read_to_string("input3.txt").unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut curr_part = vec![];
    let mut is_in = false;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c.to_digit(10) {
                Some(x) => {
                    curr_part.push(x);
                    if check_adyacent(&matrix, i, j) {
                        is_in = true;
                    }
                }
                None => {
                    if is_in {
                        let mut partnum = 0;
                        for (idx, &x) in curr_part.iter().enumerate() {
                            partnum += x * 10u32.pow((curr_part.len() - idx - 1) as u32);
                        }
                        sum += partnum;
                        is_in = false;
                    }
                    curr_part.clear();
                }
            }
        }
    }

    sum
}

fn check_adyacent(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // check if there are any adyacent symbols other than . in the 8 directions
    let mut neighbors = Vec::new();

    for &di in &[i.checked_sub(1), Some(i), i.checked_add(1)] {
        for &dj in &[j.checked_sub(1), Some(j), j.checked_add(1)] {
            if let (Some(x), Some(y)) = (di, dj) {
                neighbors.push((x, y));
            }
        }
    }

    for (x, y) in neighbors {
        if x >= matrix.len() || y >= matrix[0].len() {
            continue;
        }
        if matrix[x][y] != '.' {
            if !matrix[x][y].is_digit(10) {
                return true;
            }
        }
    }
    false
}
fn check_for_gear(
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    gears: &mut HashMap<(usize, usize), Vec<u32>>,
    curr_gear: &mut (usize, usize),
    is_in: bool,
) -> bool {
    // check if there are any adyacent symbols other than . in the 8 directions
    let mut neighbors = Vec::new();

    for &di in &[i.checked_sub(1), Some(i), i.checked_add(1)] {
        for &dj in &[j.checked_sub(1), Some(j), j.checked_add(1)] {
            if let (Some(x), Some(y)) = (di, dj) {
                neighbors.push((x, y));
            }
        }
    }

    for (x, y) in neighbors {
        if x >= matrix.len() || y >= matrix[0].len() {
            continue;
        }
        if matrix[x][y] == '*' {
            if !gears.contains_key(&(x, y)) {
                gears.insert((x, y), Vec::new());
            }
            curr_gear.0 = x;
            curr_gear.1 = y;
            return true;
        }
    }
    if !is_in {
        curr_gear.0 = 0;
        curr_gear.1 = 0;
    }
    false
}
pub fn p2() -> u32 {
    let mut sum = 0;
    let input = std::fs::read_to_string("input3.txt").unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut curr_part = vec![];
    let mut curr_gear = (0, 0);
    let mut is_in = false;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c.to_digit(10) {
                Some(x) => {
                    curr_part.push(x);
                    if check_for_gear(&matrix, i, j, &mut gears, &mut curr_gear, is_in) {
                        is_in = true;
                    }
                }
                None => {
                    match c {
                        '*' => {
                            if !gears.contains_key(&(i, j)) {
                                gears.insert((i, j), vec![]);
                            }
                        }
                        _ => {}
                    }
                    if is_in {
                        let mut partnum = 0;
                        for (idx, &x) in curr_part.iter().enumerate() {
                            partnum += x * 10u32.pow((curr_part.len() - idx - 1) as u32);
                        }
                        if curr_gear != (0, 0) {
                            gears.get_mut(&curr_gear).unwrap().push(partnum);
                        }
                        is_in = false;
                    }
                    curr_part.clear();
                }
            }
        }
    }

    for gear in gears.values() {
        if gear.len() == 2 {
            let mut product = 1;
            for part in gear {
                product *= part;
            }
            sum += product;
        }
    }

    sum
}
