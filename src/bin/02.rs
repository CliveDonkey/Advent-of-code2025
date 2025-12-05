use core::iter::Iterator;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(",");
    let mut sum = 0;

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
            let label = i.to_string();
            let (part1, part2) = label.split_at(label.len() / 2);

            if part1 == part2 {
                sum += i;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split(",");
    let mut sum = 0;

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
            let label = i.to_string();
            for j in 1..=(label.len() / 2) {
                if label.len() % j != 0 {
                    continue;
                }

                let mut newlabel: &str = label.as_str();
                let first: &str;
                (first, newlabel) = newlabel.split_at(j);

                let mut result = true;
                while newlabel.len() >= j {
                    let substring: &str;
                    (substring, newlabel) = newlabel.split_at(j);
                    if substring != first {
                        result = false;
                        break;
                    }
                }

                if result {
                    sum += i;
                    break;
                }
            }
        }
    }

    Some(sum)
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
