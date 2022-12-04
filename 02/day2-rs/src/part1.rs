use day2_rs::{calc_scores_part1, calc_scores_part2, map_to_ints};
use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").expect("Could not read file...");
    let lines= binding.lines().collect::<Vec<&str>>();
    let plays = map_to_ints(&lines);

    // part 1
    let p1 = calc_scores_part1(&plays);
    let p2 = calc_scores_part2(&plays);

    println!("Part 1 score: {}", p1);
    println!("Part 2 score: {}", p2);
}