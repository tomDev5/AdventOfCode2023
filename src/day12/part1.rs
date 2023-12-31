use crate::num_valid_permutations::num_valid_permutations;
use itertools::Itertools;

mod num_valid_permutations;

const INPUT: &str = include_str!("../../data/day12/input.txt");

fn main() {
    let result: usize = INPUT
        .lines()
        .filter_map(|line| line.split_whitespace().collect_tuple())
        .map(|(line, hash_groups)| {
            (
                line,
                hash_groups
                    .split(',')
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect_vec(),
            )
        })
        .map(|(line, hash_groups)| num_valid_permutations(line.to_string(), hash_groups))
        .sum();

    println!("result: {:?}", result)
}
