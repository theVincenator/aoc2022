use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");
    let mut ez_score = 0;
    let mut normal_score = 0;
    for line in input.lines() {
        let round = (
            line.chars().next().expect("string was empty"),
            line.chars().last().expect("string was empty"),
        );
        ez_score += calc_ez_score(round);
        normal_score += calc_normal_score(round);
    }

    println!("Score 1: {ez_score}");
    println!("Score 2: {normal_score}");
}

fn calc_ez_score(round: (char, char)) -> usize {
    match round {
        ('A', 'X') => 3 + 1,
        ('A', 'Y') => 6 + 2,
        ('A', 'Z') => 0 + 3,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 6 + 1,
        ('C', 'Y') => 0 + 2,
        ('C', 'Z') => 3 + 3,
        _ => panic!(),
    }
}

fn calc_normal_score(round: (char, char)) -> usize {
    match round {
        ('A', 'X') => 0 + 3,
        ('A', 'Y') => 3 + 1,
        ('A', 'Z') => 6 + 2,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 0 + 2,
        ('C', 'Y') => 3 + 3,
        ('C', 'Z') => 6 + 1,
        _ => panic!(),
    }
}
