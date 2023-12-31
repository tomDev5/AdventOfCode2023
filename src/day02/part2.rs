const INPUT: &str = include_str!("../../data/day02/input.txt");

mod cube_count;

use cube_count::CubeCount;

fn main() {
    let sum: usize = INPUT
        .lines()
        .filter_map(|line| line.split(':').nth(1))
        .map(|line| line.split(';'))
        .map(|game| game.filter_map(|drawing_str| CubeCount::try_from(drawing_str).ok()))
        .map(|game| game.fold(CubeCount::default(), CubeCount::max_count))
        .map(|drawing| drawing.red * drawing.green * drawing.blue)
        .sum();
    assert_eq!(sum, 72596);
}
