advent_of_code::solution!(4);

use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u64> {
    const NEIGHBORS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut peekable_lines = input.lines().peekable();
    let grid_size = peekable_lines.peek().map(|line| line.len()).unwrap() as isize;

    let mut paper_roll_positions: HashSet<(isize, isize)> = HashSet::new();
    paper_roll_positions.extend(peekable_lines.enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(x, cell)| (cell == '@').then_some((x as isize, y as isize)))
    }));

    println!(
        "Size of paper roll positions: {}",
        paper_roll_positions.len()
    );

    let mut counter: HashMap<(isize, isize), usize> =
        HashMap::with_capacity(paper_roll_positions.len());
    paper_roll_positions.iter().for_each(|&(x, y)| {
        NEIGHBORS
            .iter()
            .map(|(nx, ny)| (x + nx, y + ny))
            //.filter(|&(x, y)| x >= 0 && y >= 0 && x < grid_size && y < grid_size)
            .for_each(|e| {
                if paper_roll_positions.contains(&e) {
                    *counter.entry(e).or_default() += 1;
                }
            })
    });

    println!("Size of counter: {}", counter.len());

    for y in 0..grid_size {
        for x in 0..grid_size {
            print!(
                "{}",
                counter
                    .get(&(x, y))
                    .map_or('.', |&c| char::from_digit(c as u32, 10).unwrap())
            )
        }
        println!("");
    }

    Some(counter.into_values().filter(|c| *c < 4usize).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
