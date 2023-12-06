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

pub fn p2() -> u32 {
    let mut sum = 0;

    sum
}
