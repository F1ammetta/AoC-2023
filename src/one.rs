pub fn p1() -> u32 {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let mut first = Option::None;
        let mut last_idx: usize = 0;

        for (idx, c) in line.chars().enumerate() {
            match c.to_digit(10) {
                Some(i) => match first {
                    Option::None => {
                        first = Option::Some(i);
                    }
                    Option::Some(_) => {
                        last_idx = idx;
                    }
                },
                _ => {}
            }
        }

        let last = match line[last_idx..last_idx + 1].parse::<u32>() {
            Ok(i) => i,
            Err(_) => first.unwrap(),
        };
        sum += first.unwrap() * 10 + last;
    }

    sum
}

pub fn p2() -> u32 {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let lines = input.lines();
    let mut sum = 0;

    let mut digits = std::collections::HashMap::new();
    digits.insert("zero", 0);
    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);

    for line in lines {
        let mut first = Option::None;
        let mut last_idx: usize = 0;
        let mut is_last_text = false;
        let mut last_len = 0;

        for (idx, c) in line.chars().enumerate() {
            match c.to_digit(10) {
                Some(i) => match first {
                    Option::None => {
                        first = Option::Some(i);
                    }
                    Option::Some(_) => {
                        last_idx = idx;
                        is_last_text = false;
                    }
                },
                _ => {
                    let mut word3 = "";
                    let mut word4 = "";
                    let mut word5 = "";

                    if idx + 3 <= line.len() {
                        word3 = &line[idx..idx + 3];
                    }
                    if idx + 4 <= line.len() {
                        word4 = &line[idx..idx + 4];
                    }
                    if idx + 5 <= line.len() {
                        word5 = &line[idx..idx + 5];
                    }

                    for (key, value) in digits.iter() {
                        if word3 == *key {
                            match first {
                                Option::None => {
                                    first = Option::Some(*value);
                                }
                                Option::Some(_) => {
                                    last_idx = idx;
                                    is_last_text = true;
                                    last_len = 3;
                                }
                            }
                        } else if word4 == *key {
                            match first {
                                Option::None => {
                                    first = Option::Some(*value);
                                }
                                Option::Some(_) => {
                                    last_idx = idx;
                                    is_last_text = true;
                                    last_len = 4;
                                }
                            }
                        } else if word5 == *key {
                            match first {
                                Option::None => {
                                    first = Option::Some(*value);
                                }
                                Option::Some(_) => {
                                    last_idx = idx;
                                    is_last_text = true;
                                    last_len = 5;
                                }
                            }
                        }
                    }
                }
            }
        }
        let last;

        if is_last_text {
            last = digits
                .get(&line[last_idx..last_idx + last_len])
                .unwrap()
                .clone();
        } else {
            last = match line[last_idx..last_idx + 1].parse::<u32>() {
                Ok(n) => n,
                Err(_) => first.unwrap(),
            };
        }

        let res = first.unwrap() * 10 + last;
        sum += res;
    }
    sum
}
