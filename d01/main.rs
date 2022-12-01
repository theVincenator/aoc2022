use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");

    let mut calorie_sums: Vec<u32> = Vec::new();
    let mut max_sum = 0;
    let mut current_sum = 0;

    for line in input.lines() {
        if !line.is_empty() {
            current_sum += line.parse::<u32>().expect("Could not parse number");
            println!("cs {}", current_sum);
        } else {
            println!("total {}", current_sum);
            calorie_sums.push(current_sum);
            if current_sum > max_sum {
                max_sum = current_sum
            }
            current_sum = 0;
        }
    }

    println!("Max calorie sum: {}", max_sum);

    calorie_sums.sort();
    let len = calorie_sums.len();
    let topthree = calorie_sums.get(len - 1).expect("oob")
        + calorie_sums.get(len - 2).expect("oob")
        + calorie_sums.get(len - 3).expect("oob");

    println!("top three calories sum: {}", topthree);
}
