advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut digits = Vec::new();
    Some(
        input
            .lines()
            .map(|line| {
                digits.clear();
                digits.extend(line.chars().map(|c| c.to_digit(10).unwrap()));

                let (p_left, left) = digits[..digits.len() - 1]
                    .iter()
                    .enumerate()
                    .max_by(|(px, x), (py, y)| x.cmp(y).then(py.cmp(px)))
                    .unwrap();
                let right = digits[p_left + 1..].iter().max().unwrap();

                (left * 10 + right) as u64
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut digits = Vec::new();
    Some(
        input
            .lines()
            .map(|line| {
                digits.clear();
                digits.extend(line.chars().map(|c| c.to_digit(10).unwrap()));

                let mut joltage = 0u64;
                let mut left_bound = 0;
                for i in (0..12).rev() {
                    let (p_left, left) = digits[left_bound..digits.len() - i]
                        .iter()
                        .enumerate()
                        .max_by(|(px, x), (py, y)| x.cmp(y).then(py.cmp(px)))
                        .unwrap();

                    left_bound += p_left + 1;
                    joltage = joltage * 10 + *left as u64;
                }
                joltage
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
