advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim_end()
            .split(',')
            .map(|pair| pair.split_once('-').unwrap())
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .flat_map(|(start, end)| start..=end)
            .filter_map(|num| {
                let nb_digits = num.ilog10() + 1;
                if nb_digits % 2 == 0 {
                    let (left, right) = (
                        num / 10u64.pow(nb_digits / 2),
                        num % 10u64.pow(nb_digits / 2),
                    );
                    return (left == right).then_some(num);
                }
                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim_end()
            .split(',')
            .map(|pair| pair.split_once('-').unwrap())
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .flat_map(|(start, end)| start..=end)
            .filter_map(|num: u64| {
                let nb_digits = num.ilog10() + 1;
                'outer: for chunk_size in 1..=nb_digits / 2 {
                    let first = num % 10u64.pow(chunk_size);
                    let mut rest = num / 10u64.pow(chunk_size);
                    for _ in 1..nb_digits.div_euclid(chunk_size) {
                        let next = rest % 10u64.pow(chunk_size);
                        if first != next {
                            continue 'outer;
                        }
                        rest /= 10u64.pow(chunk_size);
                    }
                    return (rest == 0).then_some(num);
                }
                None
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
