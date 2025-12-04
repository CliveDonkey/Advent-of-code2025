use core::iter::Iterator;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(",");
    let sum = 0;

    for range in ranges {
        let mut start_end = range.split("-");
        let start = start_end.next()?;
        let end = start_end.next()?;

        //brute-force approach, can be much faster
        let start: u64 = start
            .trim()
            .parse()
            .expect("Could not convert {start} to a number");

        let end: u64 = end
            .trim()
            .parse()
            .expect("Could not convert {end} to a number");

        for i in start..=end {
            let label = i as String;
            let (part1, part2) = label.
        }
    }

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
