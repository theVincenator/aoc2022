use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");
    println!("chars until start marker: {:?}", detect_marker(&input, 4));
    println!("chars until start marker: {:?}", detect_marker(&input, 14));
}

fn detect_marker(message: &str, len: usize) -> usize {
    let mut buffer: Vec<char> = Vec::with_capacity(len);
    let mut count = 0;
    for c in message.chars() {
        // push new symbol and cound
        if buffer.len() == len {
            buffer.remove(0);
        }
        buffer.push(c);
        count += 1;

        // test for marker
        if buffer.clone().iter().unique().collect::<Vec<&char>>().len() == len {
            return count;
        }
    }

    return usize::MAX;
}
