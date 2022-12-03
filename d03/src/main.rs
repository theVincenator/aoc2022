use std::fs;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");

    let mut priosum = 0;

    for line in input.lines() {
        let (comp1, comp2) = line.split_at(line.len() / 2);

        for cur in comp1.chars() {
            if comp2.find(cur).is_some() {
                priosum += getprio(cur);
                break;
            }
        }
    }

    println!("Sum of all priorities: {priosum}");

    let mut line_iter = input.lines();
    let mut priosum2 = 0;

    loop {
        if let Some(backpack1) = line_iter.next() {
            let backpack2 = line_iter.next().expect("err");
            let backpack3 = line_iter.next().expect("err");
            for b1char in backpack1.chars() {
                if backpack2.find(b1char).is_some() && backpack3.find(b1char).is_some() {
                    // badge found -> its b1char
                    priosum2 += getprio(b1char);
                    break;
                }
            }
        } else {
            // loop over
            println!("Task 2 priosum is {priosum2}");
            break;
        };
    }
}

fn getprio(c: char) -> usize {
    ALPHABET.find(c).expect("oob") + 1
}
