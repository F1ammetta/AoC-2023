pub fn p1() -> u32 {
    let colors = vec!["red", "green", "blue"];
    let numbers = vec![12, 13, 14];
    let mut sum = 0;
    let input = std::fs::read_to_string("input2.txt").unwrap();
    for line in input.lines() {
        let mut playable = true;
        let gaem = line.split(":").collect::<Vec<&str>>();
        let gameid = gaem[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let balls = gaem[1];
        let pulls = balls.split(";").collect::<Vec<&str>>();
        for pull in pulls {
            let subpulls = pull.split(",").collect::<Vec<&str>>();
            for subpull in subpulls {
                let split = subpull.split_whitespace().collect::<Vec<&str>>();
                let color = split[1];
                let number = split[0].parse::<u32>().unwrap();
                if numbers[colors.iter().position(|&x| x == color).unwrap()] < number {
                    // println!("{} is not playable", gameid);
                    playable = false;
                }
            }
        }
        if playable {
            sum += gameid;
        }
    }

    sum
}

pub fn p2() -> u32 {
    let colors = vec!["red", "green", "blue"];
    let mut sum = 0;
    let input = std::fs::read_to_string("input2.txt").unwrap();
    for line in input.lines() {
        let mut max = vec![0, 0, 0];
        let gaem = line.split(":").collect::<Vec<&str>>();
        let balls = gaem[1];
        let pulls = balls.split(";").collect::<Vec<&str>>();
        for pull in pulls {
            let subpulls = pull.split(",").collect::<Vec<&str>>();
            for subpull in subpulls {
                let split = subpull.split_whitespace().collect::<Vec<&str>>();
                let color = split[1];
                let number = split[0].parse::<u32>().unwrap();
                if max[colors.iter().position(|&x| x == color).unwrap()] < number {
                    max[colors.iter().position(|&x| x == color).unwrap()] = number;
                }
            }
        }
        let power = max[0] * max[1] * max[2];
        sum += power;
    }
    sum
}
