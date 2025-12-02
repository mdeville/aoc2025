advent_of_code::solution!(1);

const WHEEL_SIZE: i32 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (command, value) = line.split_at(1);
                let value = value.parse::<i32>().unwrap();
                match command {
                    "R" => value,
                    "L" => -value,
                    _ => unreachable!(),
                }
            })
            .fold(
                (WHEEL_SIZE / 2, 0),
                |(current_position, nb_zeros), value| {
                    let next_position = (current_position + value).rem_euclid(WHEEL_SIZE);
                    (next_position, nb_zeros + (next_position == 0) as u64)
                },
            )
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (command, value) = line.split_at(1);
                let value = value.parse::<i32>().unwrap();
                match command {
                    "R" => value,
                    "L" => -value,
                    _ => unreachable!(),
                }
            })
            .fold(
                (WHEEL_SIZE / 2, 0),
                |(current_position, mut nb_zeros), value| {
                    let clicks = current_position + value;
                    let next_position = clicks.rem_euclid(WHEEL_SIZE);
                    nb_zeros += if value >= 0 {
                        clicks.div_euclid(WHEEL_SIZE)
                    } else {
                        ((WHEEL_SIZE - current_position).rem_euclid(WHEEL_SIZE) - value)
                            .div_euclid(WHEEL_SIZE)
                    } as u64;
                    (next_position, nb_zeros)
                },
            )
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
