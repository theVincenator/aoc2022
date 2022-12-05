use regex::Regex;
use std::fs;

fn main() {
    task_one();
    task_two();
}

fn task_one() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");

    let num_stacks = 9;
    let num_initial_rows = 8;

    let mut stacks = init_stacks(&input, num_stacks, num_initial_rows);

    // parse commands
    let num_reg = Regex::new(r"\d+").unwrap();
    let command_iterator = input.lines().skip(num_initial_rows + 2);
    for command in command_iterator {
        // parse the three numbers in each line
        let comm_nums: Vec<usize> = num_reg
            .captures_iter(command)
            .map(|c| {
                c.get(0)
                    .expect("empty capture")
                    .as_str()
                    .parse::<usize>()
                    .expect("pattern mistmatch - no number")
            })
            .collect();

        println!("{command}");
        for _ in 0..comm_nums[0] {
            let c: char = stacks[comm_nums[1] - 1]
                .pop()
                .expect("tried to pick from empty stack");
            stacks[comm_nums[2] - 1].push(c);
            print!(".");
        }
        println!("");
    }

    // print
    println!("\nRESULT:\n\n");
    for stack in stacks {
        print!("{:?}", stack.last().expect("was empty").to_string());
    }
}

fn task_two() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");

    let num_stacks = 9;
    let num_initial_rows = 8;

    let mut stacks = init_stacks(&input, num_stacks, num_initial_rows);

    // parse commands
    let num_reg = Regex::new(r"\d+").unwrap();
    let command_iterator = input.lines().skip(num_initial_rows + 2);
    for command in command_iterator {
        // parse the three numbers in each line
        let comm_nums: Vec<usize> = num_reg
            .captures_iter(command)
            .map(|c| {
                c.get(0)
                    .expect("empty capture")
                    .as_str()
                    .parse::<usize>()
                    .expect("pattern mistmatch - no number")
            })
            .collect();

        println!("{command}");
        let num_crates = comm_nums[0];
        let i_from = comm_nums[1] - 1;
        let i_to = comm_nums[2] - 1;

        let mut gripper: Vec<char> = Vec::with_capacity(num_crates);
        for _ in 0..num_crates {
            let c: char = stacks[i_from]
                .pop()
                .expect("tried to pick from empty stack");

            gripper.push(c);
            print!(".");
        }
        println!("");
        for _ in 0..num_crates {
            let c: char = gripper.pop().expect("gripper was empty");

            stacks[i_to].push(c);
            print!(".");
        }
        println!("");
    }

    // print
    println!("\nRESULT:\n\n");
    for stack in stacks {
        print!("{:?}", stack.last().expect("was empty").to_string());
    }
}

fn init_stacks(input: &str, num_stacks: usize, num_initial_rows: usize) -> Vec<Vec<char>> {
    let line_vec: Vec<&str> = input.lines().collect();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for stack_index in 0..num_stacks {
        let mut new_stack: Vec<char> = Vec::new();

        for row_index in (0..num_initial_rows).rev() {
            let current_line: Vec<char> = line_vec[row_index].chars().collect();
            let current_crate: char = current_line[1 + stack_index * 4];
            if current_crate == ' ' {
                break;
            }
            new_stack.push(current_crate);
            print!("{current_crate}");
        }
        stacks.push(new_stack);
        println!(" - Stack pushed!");
    }

    return stacks;
}
