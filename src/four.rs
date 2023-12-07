pub fn p1() -> u32 {
    let input = std::fs::read_to_string("input4.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let mut points = 0;
        let mut first = true;

        let nums = line.split(':').collect::<Vec<_>>()[1]
            .split('|')
            .collect::<Vec<_>>();

        let winning = nums[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let played = nums[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        for number in played {
            if winning.contains(&number) {
                if first {
                    points += 1;
                    first = false;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
    }

    sum
}

pub fn p2() -> u32 {
    let input = std::fs::read_to_string("input4.txt").unwrap();
    let mut cards: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();

    for i in 1..=input.lines().count() {
        cards.insert(i as u32, 1);
    }

    for line in input.lines() {
        let card = line.split(':').collect::<Vec<_>>();

        let mut winners = 0;

        let id = card[0].split_whitespace().collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();

        let nums = card[1].split('|').collect::<Vec<_>>();

        let winning = nums[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let played = nums[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        for number in played {
            if winning.contains(&number) {
                winners += 1;
            }
        }

        for i in (id + 1)..=(id + winners) {
            cards.insert(i, cards[&i] + cards[&id]);
        }
    }

    let mut sum = 0;
    for (_, n) in cards {
        sum += n;
    }

    sum
}
