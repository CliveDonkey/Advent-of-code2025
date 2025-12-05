use num_traits::pow;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage = 0;

    let lines = input.split("\n");
    for line in lines {
        let mut batteries: Vec<u64> = vec![];
        for c in line.chars() {
            let digit = c.to_digit(10)? as u64;
            batteries.push(digit);
        }

        //finding highest digit:
        let mut max_i = vec_max(batteries.as_slice());
        if max_i == batteries.len() - 1 {
            max_i = vec_max(&batteries.as_slice()[..max_i]);
        }
        let max_i = max_i;

        let second = vec_max(&batteries.as_slice()[(max_i + 1)..]);
        joltage += batteries[max_i] * 10 + batteries[second + max_i + 1];
    }

    Some(joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage = 0;
    let mut prev_joltage = 0;

    let lines = input.split("\n");
    for line in lines {
        let mut batteries: Vec<u64> = vec![];
        for c in line.chars() {
            let digit = c.to_digit(10)? as u64;
            batteries.push(digit);
        }

        //finding higest digit with enouch space behind
        let mut slice = batteries.as_slice();
        for i in 0..12 {
            let max_i = vec_max(&slice[..slice.len() + i - 11]);

            joltage += slice[max_i] * pow(10, 11 - i);
            slice = &slice[max_i + 1..];
        }
    }

    Some(joltage)
}

fn vec_max(nums: &[u64]) -> usize {
    let mut i_max = 0;
    for (i, value) in nums.iter().enumerate() {
        if value > &nums[i_max] {
            i_max = i;
        }
    }
    i_max
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
