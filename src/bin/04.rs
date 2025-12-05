advent_of_code::solution!(4);

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Grid {
    paper_roll_positions: HashSet<(isize, isize)>,
    dimension: usize,
}

impl Grid {
    fn count_neighbors(&self) -> HashMap<(isize, isize), usize> {
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

        let mut counter: HashMap<(isize, isize), usize> =
            HashMap::with_capacity(self.paper_roll_positions.len());
        self.paper_roll_positions.iter().for_each(|&(x, y)| {
            let mut is_alone = true;
            NEIGHBORS
                .iter()
                .map(|(nx, ny)| (x + nx, y + ny))
                .filter(|&(x, y)| {
                    x >= 0 && y >= 0 && x < self.dimension as isize && y < self.dimension as isize
                })
                .for_each(|e| {
                    if self.paper_roll_positions.contains(&e) {
                        is_alone = false;
                        *counter.entry(e).or_default() += 1;
                    }
                });
            if is_alone {
                counter.insert((x, y), 1);
            }
        });

        counter
    }
}

impl FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut peekable_lines = input.lines().peekable();
        let dimension = peekable_lines.peek().map(|line| line.len()).unwrap();

        let mut paper_roll_positions: HashSet<(isize, isize)> = HashSet::new();
        paper_roll_positions.extend(peekable_lines.enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, cell)| (cell == '@').then_some((x as isize, y as isize)))
        }));

        Ok(Grid {
            paper_roll_positions,
            dimension,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input.parse::<Grid>().unwrap();

    Some(
        grid.count_neighbors()
            .into_values()
            .filter(|c| *c < 4)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input.parse::<Grid>().unwrap();

    let mut last_paper_roll_count = grid.paper_roll_positions.len();
    let mut nb_removed = 0;
    loop {
        nb_removed += grid
            .count_neighbors()
            .into_iter()
            .filter(|(_, count)| *count < 4)
            .inspect(|(pos, _)| {
                grid.paper_roll_positions.remove(pos);
            })
            .count();
        if grid.paper_roll_positions.len() == last_paper_roll_count {
            break;
        }
        last_paper_roll_count = grid.paper_roll_positions.len();
    }

    Some(nb_removed as u64)
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
        assert_eq!(result, Some(43));
    }
}
