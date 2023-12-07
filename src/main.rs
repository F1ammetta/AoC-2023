mod four;
mod one;
mod three;
mod two;

fn main() {
    println!("Advent of Code 2023");
    println!("==================");
    println!("Day 1");
    let now = std::time::Instant::now();

    let res_1 = one::p1();

    let res_2 = one::p2();

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    println!("Result 1: {}", res_1);
    println!("Result 2: {}", res_2);

    println!("==================");
    println!("Day 2");
    let now = std::time::Instant::now();

    let res_1 = two::p1();

    let res_2 = two::p2();

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    println!("Result 1: {}", res_1);
    println!("Result 2: {}", res_2);

    println!("==================");
    println!("Day 3");
    let now = std::time::Instant::now();

    let res_1 = three::p1();

    let res_2 = three::p2();

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    println!("Result 1: {}", res_1);
    println!("Result 2: {}", res_2);

    println!("==================");
    println!("Day 4");
    let now = std::time::Instant::now();

    let res_1 = four::p1();

    let res_2 = four::p2();

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    println!("Result 1: {}", res_1);
    println!("Result 2: {}", res_2);
}
