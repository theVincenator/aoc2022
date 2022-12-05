use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");

    for line in input.lines() {
        if !line.is_empty() {
            println!("{line}");
        } else {
            println!("<emptyline>");
        }
    }
}
