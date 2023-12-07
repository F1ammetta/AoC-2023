pub fn p1() -> i64 {
    let input = std::fs::read_to_string("input5.txt").unwrap();
    let lines = input.lines();
    let lines2 = lines.clone().collect::<Vec<&str>>();

    let strseeds = lines.clone().collect::<Vec<&str>>()[0]
        .split(":")
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut seeds = strseeds
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut indices: Vec<i64> = vec![];

    for (idx, line) in lines.enumerate() {
        if line == "" {
            indices.push(idx as i64);
        }
    }

    let mut maps = vec![];

    for (i, index) in indices.iter().enumerate() {
        if i == indices.len() - 1 {
            maps.push(lines2[*index as usize + 2..lines2.len() as usize].to_vec());
        } else {
            maps.push(lines2[*index as usize + 2..indices[i + 1] as usize].to_vec());
        }
    }

    for map in maps {
        let mut temp = seeds.clone();
        for range in map {
            let range = range
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let outstart = range[0];
            let instart = range[1];
            let len = range[2];

            for (idx, seed) in seeds.iter_mut().enumerate() {
                if *seed >= instart && *seed < instart + len {
                    temp[idx] = *seed + outstart - instart;
                }
            }
        }
        seeds = temp;
    }

    *seeds.iter().min().unwrap()
}

pub fn p2() -> i64 {
    0
}
