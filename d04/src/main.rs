use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read file");
    let num_reg = Regex::new(r"\d+").unwrap();

    let mut include_count = 0;
    let mut overlap_count = 0;

    for line in input.lines() {
        let section_ids: Vec<usize> = num_reg
            .captures_iter(line)
            .map(|c| {
                c.get(0)
                    .expect("empty capture")
                    .as_str()
                    .parse::<usize>()
                    .expect("pattern mistmatch - no number")
            })
            .collect();

        let start1 = section_ids[0];
        let end1 = section_ids[1];
        let start2 = section_ids[2];
        let end2 = section_ids[3];

        let range1 = start1..end1 + 1;
        let range2 = start2..end2 + 1;

        if range1.contains(&start2) && range1.contains(&end2)
            || range2.contains(&start1) && range2.contains(&end1)
        {
            // one is contained in other
            println!("{line}\t is containing & overlapping");
            include_count += 1;
            overlap_count += 1;
        } else if range1.contains(&start2)
            || range1.contains(&end2)
            || range2.contains(&start1)
            || range2.contains(&end1)
        {
            // one is overlapping
            println!("{line}\t is overlapping");
            overlap_count += 1;
        }

        println!("...")
    }

    println!("Number of contained ranges is: {include_count}");
    println!("Number of overlapping ranges is: {overlap_count}");
}
